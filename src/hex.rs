use std::borrow::Cow;
use std::fmt::{Debug, Formatter};
use std::ops::Deref;
use std::str::FromStr;

use anyhow::anyhow;
use bitvec::prelude::*;
use itertools::Itertools;

use crate::{AnyResult, B64};

// A valid hexadecimal string as bytes
#[derive(Ord, PartialOrd, Eq, PartialEq)]
pub struct Hex<'a>(Cow<'a, [u8]>);

impl<'a> FromStr for Hex<'a> {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bytes_iter = s.as_bytes().chunks_exact(2);
        let mut buffer = Vec::with_capacity(s.as_bytes().len());

        for bytes in bytes_iter {
            let to_str = std::str::from_utf8(bytes).map_err(|_| anyhow!("Invalid UTF-8 string"))?;
            let hex = u8::from_str_radix(to_str, 16).map_err(|_| anyhow!("Couldn't convert to HEX."))?;
            buffer.push(hex);
        }
        Ok(Hex(Cow::from(buffer)))
    }
}

impl<'a> Deref for Hex<'a> {
    type Target = Cow<'a, [u8]>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a> Debug for Hex<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#x?}", self.0)
    }
}

impl<'a> Hex<'a> {
    pub fn into_b64_string(self) -> String {
        let mut buffer = String::with_capacity(self.0.len());
        self.0.as_bits::<Msb0>().chunks(6).for_each(|bits| {
            let encoded = B64.chars().nth(bits.load_be()).unwrap();
            buffer.push(encoded);
        });
        buffer
    }

    /// A      B      C
    /// 1      2      3
    /// 000001 000010 000011
    pub fn from_b64_encoded_string(encoded_str: &str) -> AnyResult<Self> {
        let mut buffer = BitVec::<u8, Msb0>::new();
        encoded_str.chars().for_each(|encoded_char| {
            let b64_idx = B64
                .chars()
                .position(|b64_char| b64_char == encoded_char)
                .unwrap() as u8;
            buffer.extend_from_bitslice(&b64_idx.view_bits::<Msb0>()[2..]);
        });
        let buffer_as_bytes = buffer.chunks(8).map(|bitslice| bitslice.load_be::<u8>());
        Ok(Self(Cow::from(buffer_as_bytes.collect_vec())))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn success_b64_encode() {
        let source = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
        let expected = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";
        insta::assert_snapshot!(Hex::from_str(source).unwrap().into_b64_string());
    }
    #[test]
    fn success_b64_decode() {
        let decode = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
        let source = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";
        assert_eq!(
            Hex::from_b64_encoded_string(source).unwrap(),
            decode.parse::<Hex>().unwrap()
        );
    }

    #[test]
    fn success_parse_str_as_hex_str() {
        let source = "49276d";
        let expected = &[0x49u8, 0x27, 0x6d];

        let res = Hex::from_str(source);
        insta::assert_snapshot!(format!("{:?}", res));
    }
}
