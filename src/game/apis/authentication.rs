use prost::Message;

use crate::game::packet::Packet;
use crate::game::protos::cmd_types;
use crate::game::protos::proto::{PlayerGetTokenCsReq, PlayerGetTokenScRsp};

pub fn build_player_get_token_cs_req(
    account_uid: String,
    token: String,
    device: String,
    client_rand_key: String,
) -> Packet {
    Packet {
        cmd_type: cmd_types::PLAYER_GET_TOKEN_CS_REQ,
        head: Vec::new(),
        body: PlayerGetTokenCsReq {
            field1: 12121,
            token,
            account_uid,
            field5: 3909,
            field6: 5571,
            device,
            field11: 6567,
            field13: 6874,
            client_rand_key,
        }
        .encode_to_vec(),
    }
}

pub fn parse_player_get_token_sc_rsp(packet: Packet) -> PlayerGetTokenScRsp {
    PlayerGetTokenScRsp::decode(&packet.body[..]).unwrap()
}
