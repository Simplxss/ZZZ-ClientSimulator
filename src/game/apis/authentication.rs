use crate::game::packet::Packet;
use crate::game::protos::cmd_types;

pub fn build_player_get_token_cs_req() -> Packet {
    Packet {
        cmd_type: cmd_types::PLAYER_GET_TOKEN_CS_REQ,
        head: Vec::new(),
        body: Vec::new(),
    }
}