use std::net::SocketAddr;

use tokio_kcp::{KcpConfig, KcpStream};

use crate::common::{ec2b::Ec2b, mt64, rsa, xor};

use super::packet::Packet;

use super::protos::cmd_types;

struct Player {
    account_uid: String,
    token: String,
    device: String,
    rsa_ver: u32,
    client_rand_key: u64,
}

pub struct Session {
    kcp: KcpStream,
    server_secret_key: Box<[u8]>,
    session_key: Option<Box<[u8]>>,

    player: Player,
}

impl Session {
    pub async fn new(
        addr: SocketAddr,
        rsa_ver: u32,
        client_secret_key: &str,
        account_uid: &str,
        token: &str,
        device: &str,
    ) -> Result<Self, String> {
        let dec_client_secret_key = match base64::decode(client_secret_key) {
            Ok(v) => v,
            Err(e) => panic!("Failed to decode client secret key: {}", e),
        };
        let ec2b = match Ec2b::read(&dec_client_secret_key) {
            Ok(v) => v,
            Err(e) => return Err(format!("Failed to decode Ec2b: {}", e)),
        };

        Ok(Self {
            kcp: KcpStream::connect(&KcpConfig::default(), addr)
                .await
                .unwrap(),
            server_secret_key: mt64::gen_server_secret_key(ec2b.derive_seed(), 4096),
            session_key: None,

            player: Player {
                account_uid: account_uid.to_string(),
                token: token.to_string(),
                device: device.to_string(),
                rsa_ver,
                client_rand_key: rand::random::<u64>(),
            },
        })
    }

    pub async fn send(&mut self, mut packet: Packet) -> Result<(), String> {
        xor::xor(
            &mut packet.body,
            match packet.cmd_type {
                cmd_types::PLAYER_GET_TOKEN_CS_REQ => &self.server_secret_key,
                _ => self.session_key.as_ref().unwrap(),
            },
        );
        let payload: Vec<u8> = packet.into();

        let _ = self.kcp.send(&payload).await;

        Ok(())
    }

    pub async fn recv(&mut self) -> Packet {
        let mut buf = vec![0u8; 4096];
        let n = self.kcp.recv(&mut buf).await.unwrap();

        let mut packet = Packet::from(&buf[..n]);
        xor::xor(
            &mut packet.body,
            match packet.cmd_type {
                cmd_types::PLAYER_GET_TOKEN_SC_RSP => &self.server_secret_key,
                _ => self.session_key.as_ref().unwrap(),
            },
        );

        packet
    }

    pub async fn get_token(&mut self) -> Result<(), String> {
        let client_rand_key = match rsa::rsa_encrypt(
            &self.player.client_rand_key.to_le_bytes(),
            self.player.rsa_ver,
        ) {
            Ok(v) => v,
            Err(e) => panic!("Failed to encrypt client rand key: {}", e),
        };
        let req = crate::game::apis::authentication::build_player_get_token_cs_req(
            self.player.account_uid.clone(),
            self.player.token.clone(),
            self.player.device.clone(),
            client_rand_key,
        );
        self.send(req).await?;

        let packet = self.recv().await;
        let rsp = crate::game::apis::authentication::parse_player_get_token_sc_rsp(packet);

        let decrypted = match rsa::rsa_decrypt(&rsp.server_rand_key, self.player.rsa_ver) {
            Ok(v) => v,
            Err(e) => return Err(format!("Failed to decrypt server rand key: {}", e)),
        };

        match rsa::rsa_verify_sign(&decrypted, &rsp.sign, self.player.rsa_ver) {
            Ok(true) => (),
            Ok(false) => return Err("Failed to verify sign".to_string()),
            Err(e) => return Err(format!("Failed to verify sign: {}", e)),
        };

        let server_rand_key = u64::from_le_bytes(match decrypted[..8].try_into() {
            Ok(v) => v,
            Err(e) => return Err(format!("Failed to convert to u64: {}", e)),
        });

        self.session_key = Some(
            mt64::gen_server_secret_key(self.player.client_rand_key ^ server_rand_key, 4096)
                .to_vec()
                .into_boxed_slice(),
        );
        Ok(())
    }
}
