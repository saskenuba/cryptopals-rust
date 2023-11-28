use std::ops::BitXor;

use anyhow::ensure;
use bitvec::prelude::*;

use crate::AnyResult;

pub trait XorExt {
    fn fixed_xor_slice(self, b2: impl AsRef<[u8]>) -> AnyResult<Vec<u8>>;
    fn fixed_xor_byte(self, b2: u8) -> Vec<u8>;

    fn repeated_xor_key(self, key: impl AsRef<[u8]>) -> Vec<u8>;
}

impl XorExt for &[u8] {
    fn fixed_xor_slice(self, b2: impl AsRef<[u8]>) -> AnyResult<Vec<u8>> {
        let b1 = self;
        let b2 = b2.as_ref();

        ensure!(b1.len() == b2.len());
        let buffer: BitVec<_> = b1
            .view_bits::<Lsb0>()
            .iter()
            .zip(b2.view_bits::<Lsb0>().iter())
            .map(|(bb1, bb2)| *bb1 ^ *bb2)
            .collect();

        Ok(buffer.into_vec())
    }

    fn fixed_xor_byte(self, b2: u8) -> Vec<u8> {
        let buffer: BitVec<_> = self.iter().map(|bb1| bb1.bitxor(b2)).collect();
        buffer.into_vec()
    }

    /// Returns a
    ///
    /// "this is a text" with the key "ICE"
    /// internally it will extend the key and XOR each byte
    ///
    /// this is a text
    /// ICEICEICEICEIC
    fn repeated_xor_key(self, key: impl AsRef<[u8]>) -> Vec<u8> {
        let key_bytes = key.as_ref();
        let cycle = key_bytes.repeat(self.len() / key_bytes.len() + 1);

        self.chunks(key_bytes.len())
            .zip(cycle.chunks(key_bytes.len()))
            .flat_map(|(original_bytes, key_bytes)| original_bytes.fixed_xor_slice(&key_bytes[..original_bytes.len()]))
            .flatten()
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use std::ops::Deref;

    use super::*;
    use crate::hex::Hex;

    #[test]
    fn success_xor_two_buffers() {
        let source = "1c0111001f010100061a024b53535009181c"
            .parse::<Hex>()
            .unwrap();
        let xor_against_hex = "686974207468652062756c6c277320657965"
            .parse::<Hex>()
            .unwrap();

        let res = source.fixed_xor_slice(xor_against_hex.deref()).unwrap();
        let expected = "746865206b696420646f6e277420706c6179"
            .parse::<Hex>()
            .unwrap();
        assert_eq!(*expected, res);
    }
    #[test]
    fn success_repeated_xor_ice_key() {
        let source = b"Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal";
        let key = b"ICE";
        let expected = "0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f";

        let source_hex_bytes = source.repeated_xor_key(key);
        assert_eq!(source_hex_bytes, expected.parse::<Hex>().unwrap().as_ref());
    }
}
