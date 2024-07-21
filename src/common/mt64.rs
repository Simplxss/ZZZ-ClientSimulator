use rand_mt::Mt64;

pub fn gen_server_secret_key(seed: u64, size: usize) -> Box<[u8]> {
    let mut mt = Mt64::new(seed);
    let mut buf = Vec::with_capacity(size as usize);

    for _ in 0..(size >> 3) {
        buf.extend_from_slice(&mt.next_u64().to_le_bytes());
    }
    buf.into_boxed_slice()
}
