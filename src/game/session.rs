use std::net::SocketAddr;

use tokio_kcp::{KcpConfig, KcpStream};

use super::packet::Packet;

struct Player {}

pub struct Session {
    kcp: KcpStream,
    session_key: Option<Box<[u8]>>,

    player: Option<Player>,
}

impl Session {
    pub async fn new(addr: SocketAddr) -> Self {
        Self {
            kcp: KcpStream::connect(&KcpConfig::default(), addr).await.unwrap(),
            session_key: None,

            player: None,
        }
    }

    pub async fn send(&mut self, packet: Packet) -> Result<(), String> {
        let payload: Vec<u8> = packet.into();

        self.kcp.send(&payload).await;

        Ok(())
    }

    pub fn set_session_key(&mut self, value: Box<[u8]>) {
        self.session_key = Some(value);
    }
}
