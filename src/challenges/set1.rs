use std::collections::BTreeMap;
use std::iter::{IntoIterator, Iterator};
use std::ops::RangeInclusive;

use anyhow::ensure;
use bitvec::macros::internal::funty::Fundamental;
use bitvec::prelude::*;
use itertools::Itertools;

use crate::xor::XorExt;
use crate::{dictionary, AnyResult};

/// Calculates the Hamming distance between two numbers, i.e, calculates the amount of differing bits between two
/// strings.
///
/// **Warning**: `s1` and `s2` must match in length.
pub fn hamming_distance_bit(s1: impl AsRef<[u8]>, s2: impl AsRef<[u8]>) -> AnyResult<i32> {
    let s1 = s1.as_ref();
    let s2 = s2.as_ref();

    ensure!(s1.len() == s2.len(), "s1 and s2 must match in length.");

    Ok(s1
        .as_bits::<Lsb0>()
        .into_iter()
        .zip(s2.as_bits::<Lsb0>())
        .map(|(a, b)| *a ^ *b)
        .filter(|c| c == &true)
        .count() as i32)
}

/// Bruteforces a `xor_encoded_str` by trying chars from 0 to 255, and returns the top 5 on the
/// format of:
/// (probable char, decoded_string with probable char).
pub fn bruteforce_hex_by_fixed_xor(
    xor_encoded_str: impl AsRef<[u8]>,
    range: Option<RangeInclusive<u8>>,
) -> AnyResult<Vec<(u8, String)>> {
    let char_range = range.unwrap_or(u8::MIN..=u8::MAX);

    let res = char_range
        .into_iter()
        .filter_map(|char| xor_by_char_and_score(&xor_encoded_str, char).map(|score| (char, score)))
        .collect::<BTreeMap<_, _>>();

    Ok(res
        .into_iter()
        .sorted_by(|prev, next| Ord::cmp(&next.1, &prev.1))
        .take(5)
        .inspect(|(char, score)| println!("char: {:?} score: {}", char.as_char(), score))
        .map(|(xor_char, _)| (xor_char, xor_encoded_str.as_ref().fixed_xor_byte(xor_char)))
        .map(|(xor_char, bytes)| (xor_char, String::from_utf8(bytes).unwrap()))
        .collect())
}

fn xor_by_char_and_score(xor_encoded_str: impl AsRef<[u8]>, char: u8) -> Option<i32> {
    let decipher_xor = xor_encoded_str.as_ref().fixed_xor_byte(char);

    let Ok(utf_str) = std::str::from_utf8(decipher_xor.as_ref()) else {
        return None;
    };

    let score = dictionary::score_english_letters_phrase(utf_str);
    Some(score)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn success_hamming_distance() {
        let s1 = "this is a test";
        let s2 = "wokka wokka!!!";
        let res = hamming_distance_bit(s1, s2).unwrap();
        assert_eq!(res, 37);

        let s1 = "what is this";
        let s2 = "i dont knoww";
        let res = hamming_distance_bit(s1, s2).unwrap();
        assert_eq!(res, 39);
    }
}
