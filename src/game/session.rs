use std::net::SocketAddr;

use tokio_kcp::{KcpConfig, KcpStream};

use crate::common::{ec2b::Ec2b, mt64, xor};

use super::packet::Packet;

pub enum KeyType {
    None,
    ServerSecretKey,
    SessionKey,
}

struct Player {}

pub struct Session {
    kcp: KcpStream,
    server_secret_key: Box<[u8]>,
    session_key: Option<Box<[u8]>>,

    player: Option<Player>,
}

impl Session {
    pub async fn new(addr: SocketAddr, client_secret_key: &str) -> Self {
        let ec2b =
            Ec2b::read(&base64::decode(client_secret_key).expect("Failed to decode secret key"))
                .expect("Failed to read Ec2b data");
        Self {
            kcp: KcpStream::connect(&KcpConfig::default(), addr)
                .await
                .unwrap(),
            server_secret_key: mt64::gen_server_secret_key(ec2b.derive_seed(), 4096),
            session_key: None,

            player: None,
        }
    }

    pub async fn send(&mut self, mut packet: Packet, key_type: KeyType) -> Result<(), String> {
        xor::xor(
            &mut packet.body,
            match key_type {
                KeyType::None => &[0u8; 4096],
                KeyType::ServerSecretKey => &self.server_secret_key,
                KeyType::SessionKey => self.session_key.as_ref().unwrap(),
            },
        );
        let payload: Vec<u8> = packet.into();

        let _ = self.kcp.send(&payload).await;

        Ok(())
    }

    pub fn set_session_key(&mut self, value: Box<[u8]>) {
        self.session_key = Some(value);
    }
}
