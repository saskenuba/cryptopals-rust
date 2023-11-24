use std::borrow::Cow;
use std::fmt::{Debug, Formatter};
use std::ops::Deref;

use bitvec::field::BitField;
use bitvec::order::Msb0;
use bitvec::prelude::AsBits;

use crate::B64;

// A valid hexadecimal string as bytes
pub struct Hex<'a>(Cow<'a, [u8]>);

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

pub trait ToHex {
    type Error;

    fn to_hex<'a>(&self) -> Result<Hex<'a>, Self::Error>;
}

impl<T> ToHex for T
where
    T: AsRef<[u8]>,
{
    type Error = &'static str;

    fn to_hex<'a>(&self) -> Result<Hex<'a>, Self::Error> {
        let bytes_iter = self.as_ref().chunks_exact(2);
        let mut buffer = Vec::with_capacity(self.as_ref().len());

        for bytes in bytes_iter {
            let to_str = std::str::from_utf8(bytes).map_err(|_| "invalid ")?;
            let hex = u8::from_str_radix(to_str, 16).map_err(|_| "invalid hex bytes")?;
            buffer.push(hex);
        }
        Ok(Hex(Cow::from(buffer)))
    }
}

impl<'a> Hex<'a> {
    pub fn into_b64_string(self) -> String {
        let mut buffer = String::with_capacity(self.0.len());
        self.0.as_bits::<Msb0>().chunks(6).for_each(|bits| {
            let encoded = B64.chars().nth(dbg!(bits.load_be())).unwrap();
            buffer.push(encoded);
        });
        buffer
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn success_b64_conversion() {
        let source = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
        let expected = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";
        insta::assert_snapshot!(source.to_hex().unwrap().into_b64_string());
    }
    #[test]
    fn success_parse_str_as_hex_str() {
        let source = "49276d";
        let expected = &[0x49u8, 0x27, 0x6d];

        let res = source.to_hex();
        insta::assert_snapshot!(format!("{:?}", res));
    }
}
