use std::borrow::Cow;
use std::fmt::{Debug, Formatter};
use std::ops::Deref;
use std::str::FromStr;

use anyhow::anyhow;
use bitvec::macros::internal::funty::Fundamental;
use bitvec::prelude::*;
use itertools::Itertools;

use crate::{AnyResult, B64};

// A valid hexadecimal string as bytes
#[derive(Ord, PartialOrd, Eq, PartialEq)]
pub struct Hex<'a>(Cow<'a, [u8]>);

impl<'a> From<&'a [u8]> for Hex<'a> {
    fn from(value: &'a [u8]) -> Self {
        Self(Cow::Borrowed(value))
    }
}

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
    /// Converts a Hex string into a base64 encoded string.
    pub fn into_b64_string(self) -> String {
        let mut buffer = String::with_capacity(self.0.len() * 4);
        let bits_iter = self.0.view_bits::<Msb0>();
        let padding = bits_iter.len() % 3;

        bits_iter.chunks(6).for_each(|bits| {
            let bits = if bits.len() < 6 {
                let mut vec = bits.to_bitvec();
                vec.extend_from_bitslice(&bits![0; 6][0..6 - bits.len()]);
                vec
            } else {
                bits.to_bitvec()
            };
            let encoded = B64.chars().nth(bits.load_be()).unwrap();
            buffer.push(encoded);
        });

        (0..padding).for_each(|_| {
            buffer.push('=');
        });

        buffer
    }

    /// Decodes a b64 encoded string into Hex string.
    ///
    /// Ignores whitespaces, newlines.
    pub fn from_b64_encoded_string(encoded_str: &str) -> AnyResult<Self> {
        let mut bitbuffer = BitVec::<u8, Msb0>::new();

        let encoded_str = encoded_str
            .trim_end_matches(|c| !B64.contains(c))
            .chars()
            .filter(|c| !c.is_whitespace());

        for encoded_char in encoded_str {
            let b64_idx = B64
                .chars()
                .position(|b64_char| encoded_char == b64_char)
                .map(|a| a.as_u8())
                .ok_or_else(|| anyhow!("Invalid B64, byte: {}, char: {}", encoded_char.as_u8(), encoded_char))?;

            bitbuffer.extend_from_bitslice(&b64_idx.view_bits::<Msb0>()[2..]);
        }
        let final_buffer = bitbuffer
            .chunks(8)
            .map(|bitslice| bitslice.load_be::<u8>())
            .filter(|c| c != &0x0)
            .collect_vec();

        Ok(Self(Cow::from(final_buffer)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn success_b64_encode_no_padding() {
        let source = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
        let expected = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";
        insta::assert_snapshot!(Hex::from_str(source).unwrap().into_b64_string());
    }
    #[test]
    fn success_b64_encode_with_padding() {
        let source = "61626365";
        let expected = "YWJjZQ==";
        assert_eq!(Hex::from_str(source).unwrap().into_b64_string(), expected);
    }
    #[test]
    fn success_b64_decode_no_padding() {
        let source = "dGhpcyBpcyBhIHRlc3R0";
        let expected = "746869732069732061207465737474";
        assert_eq!(
            Hex::from_b64_encoded_string(source).unwrap(),
            expected.parse::<Hex>().unwrap()
        );
    }

    #[test]
    fn success_b64_decode_with_padding_but_no_padding() {
        let source = "dGhpcyBpcyBhIHRlc3Q"; // Should have one '='
        let expected = "7468697320697320612074657374";
        assert_eq!(
            Hex::from_b64_encoded_string(source).unwrap(),
            expected.parse::<Hex>().unwrap()
        );
    }

    #[test]
    fn success_b64_decode_padding() {
        let source = "d2FrYSB3YWthCg==";
        let expected = "77616b612077616b610a";
        let res = Hex::from_b64_encoded_string(source).unwrap();
        insta::assert_snapshot!(String::from_utf8(res.to_vec()).unwrap());
        assert_eq!(res, expected.parse::<Hex>().unwrap());
    }

    #[test]
    fn success_parse_str_as_hex_str() {
        let source = "49276d";
        let expected = &[0x49u8, 0x27, 0x6d];

        let res = Hex::from_str(source);
        insta::assert_snapshot!(format!("{:?}", res));
    }
}
