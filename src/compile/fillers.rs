use bytes::{BufMut, Bytes, BytesMut};

pub fn fill_with_random(bytes: &mut BytesMut, n: usize) {
    for _ in 0..n {
        bytes.put_u8(rand::random());
    }
}

pub fn fill_with_pattern(bytes: &mut BytesMut, pattern: &Bytes, n: usize) {
    if pattern.is_empty() {
        for _ in 0..n {
            bytes.put_u8(0x00);
        }
    } else {
        for i in 0..n {
            bytes.put_u8(pattern[i % pattern.len()]);
        }
    }
}
