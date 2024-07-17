
const HEAD_MAGIC: u32 = 0x01234567;
const TAIL_MAGIC: u32 = 0x89abcdef;

#[derive(Debug)]
pub struct Packet {
    pub cmd_type: u16,
    pub head: Vec<u8>,
    pub body: Vec<u8>,
}

impl From<&[u8]> for Packet {
    fn from(value: &[u8]) -> Self {
        assert_eq!(
            HEAD_MAGIC,
            u32::from_be_bytes(value[..4].try_into().unwrap())
        );

        let cmd_type = u16::from_be_bytes(value[4..6].try_into().unwrap());
        let head_size = u16::from_be_bytes(value[6..8].try_into().unwrap()) as usize;
        let body_size = u32::from_be_bytes(value[8..12].try_into().unwrap()) as usize;

        assert_eq!(
            TAIL_MAGIC,
            u32::from_be_bytes(value[12 + head_size + body_size..].try_into().unwrap())
        );

        Self {
            cmd_type,
            head: value[12..12 + head_size].to_vec(),
            body: value[12 + head_size..12 + head_size + body_size].to_vec(),
        }
    }
}

impl From<Packet> for Vec<u8> {
    fn from(value: Packet) -> Self {
        let mut buf = Self::with_capacity(16 + value.head.len() + value.body.len());

        buf.extend(HEAD_MAGIC.to_be_bytes());
        buf.extend(value.cmd_type.to_be_bytes());
        buf.extend((value.head.len() as u16).to_be_bytes());
        buf.extend((value.body.len() as u32).to_be_bytes());
        buf.extend(value.head);
        buf.extend(value.body);
        buf.extend(TAIL_MAGIC.to_be_bytes());

        buf
    }
}