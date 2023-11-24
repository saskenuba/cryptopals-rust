use std::ops::BitXor;

use anyhow::ensure;
use bitvec::prelude::*;

use crate::AnyResult;

pub fn fixed_xor<T1, T2>(b1: T1, b2: T2) -> AnyResult<Vec<u8>>
where
    T1: AsRef<[u8]>,
    T2: AsRef<[u8]>,
{
    let b1 = b1.as_ref();
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
pub fn fixed_xor_byte<T1>(b1: T1, b2: u8) -> AnyResult<Vec<u8>>
where
    T1: AsRef<[u8]>,
{
    let b1 = b1.as_ref();
    let buffer: BitVec<_> = b1.iter().map(|bb1| bb1.bitxor(b2)).collect();

    Ok(buffer.into_vec())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hex::ToHex;

    #[test]
    fn success_xor_two_buffers() {
        let source = "1c0111001f010100061a024b53535009181c";
        let xor_against_hex = "686974207468652062756c6c277320657965"
            .as_bytes()
            .to_hex()
            .unwrap();

        let source_hex_bytes = source.as_bytes().to_hex().unwrap();
        let res = fixed_xor(&source_hex_bytes, &xor_against_hex).unwrap();

        let expected = b"746865206b696420646f6e277420706c6179"
            .as_slice()
            .to_hex()
            .unwrap();
        assert_eq!(*expected, res);
    }

    #[test]
    fn a() {
        let a = 10 ^ 10;
    }
}
