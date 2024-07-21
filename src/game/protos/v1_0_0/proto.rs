#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PacketHead {
    #[prost(uint32, tag = "1")]
    pub packet_id: u32,
    #[prost(uint32, tag = "11")]
    pub request_id: u32,
    #[prost(bool, tag = "14")]
    pub dneigcmldhk: bool,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlayerGetTokenCsReq {
    #[prost(uint32, tag = "1")]
    pub field1: u32,
    #[prost(string, tag = "2")]
    pub token: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub account_uid: ::prost::alloc::string::String,
    #[prost(uint32, tag = "5")]
    pub field5: u32,
    #[prost(uint32, tag = "6")]
    pub field6: u32,
    #[prost(string, tag = "9")]
    pub device: ::prost::alloc::string::String,
    #[prost(uint32, tag = "11")]
    pub field11: u32,
    #[prost(uint32, tag = "13")]
    pub field13: u32,
    #[prost(string, tag = "14")]
    pub client_rand_key: ::prost::alloc::string::String,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlayerGetTokenScRsp {
    #[prost(string, tag = "2")]
    pub sign: ::prost::alloc::string::String,
    #[prost(uint32, tag = "3")]
    pub uid: u32,
    #[prost(string, tag = "4")]
    pub server_rand_key: ::prost::alloc::string::String,
}
