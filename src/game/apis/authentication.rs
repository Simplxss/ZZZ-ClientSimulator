use crate::game::packet::Packet;

pub fn build_get_player_token_req() -> Packet {
    Packet {
        cmd_type: crate::game::proto::v1_0_0::cmd_types::GET_PLAYER_TOKEN_REQ,
        head: Vec::new(),
        body: Vec::new(),
    }
}