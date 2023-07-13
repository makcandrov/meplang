use std::ops::{Deref, DerefMut};

use bytes::Bytes;

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct Bytes32(pub [u8; 32]);

impl Deref for Bytes32 {
    type Target = [u8; 32];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Bytes32 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Bytes32 {
    pub fn from_bytes(bytes: &Bytes, right_padding: bool) -> Option<Self> {
        if bytes.len() > 32 {
            return None;
        }

        let mut res = [0; 32];
        let start_index = if right_padding { 32 - bytes.len() } else { 0 };
        for i in 0..bytes.len() {
            res[start_index + i] = bytes[i];
        }
        Some(Self(res))
    }

    pub fn leading_zeros(&self) -> usize {
        let mut i = 0;
        while self[i] == 0 {
            i += 1;
            if i == 32 {
                break;
            }
        }
        i
    }

    pub fn is_zero(&self) -> bool {
        self.leading_zeros() == 32
    }

    pub fn right_content(&self) -> &[u8] {
        &self[self.leading_zeros()..32]
    }
}
