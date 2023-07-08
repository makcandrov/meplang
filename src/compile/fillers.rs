use bytes::{BufMut, Bytes, BytesMut};

use crate::pre_processing::opcode::push_length;

pub fn fill_with_random(bytes: &mut BytesMut, n: usize) {
    for i in (0..n).rev() {
        bytes.put_u8(loop {
            let op: u8 = rand::random();
            if let Some(push_length) = push_length(op) {
                if push_length > i {
                    continue;
                }
            }
            break op;
        });
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
