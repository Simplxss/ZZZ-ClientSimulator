pub fn xor(buf: &mut [u8], key: &[u8]) {
    buf.iter_mut()
        .enumerate()
        .for_each(|(i, v)| *v ^= key[i % key.len()]);
}