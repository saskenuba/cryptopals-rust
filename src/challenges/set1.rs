use std::collections::BTreeMap;

use anyhow::ensure;
use bitvec::prelude::*;
use itertools::Itertools;

use crate::hex::Hex;
use crate::xor::XorExt;
use crate::AnyResult;

static ENGLISH_LETTERS: &[char] = &['e', 't', 'a', 'o', 'i', 'n', 's', 'h', 'r', 'd'];

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

/// Returns a Vec<String> containing...
pub fn decode_hex_str_to_english(xor_encoded_str: Hex) -> AnyResult<Vec<String>> {
    let char_range = 0..=255u8;

    let res = char_range
        .into_iter()
        .filter_map(|char| {
            let decipher_xor = xor_encoded_str.as_ref().fixed_xor_byte(char);
            let mut set = BTreeMap::new();

            let Ok(utf_str) = std::str::from_utf8(decipher_xor.as_ref()) else {
                return None;
            };

            utf_str.chars().for_each(|char| {
                if ENGLISH_LETTERS.contains(&char) {
                    set.entry(char).and_modify(|count| *count += 1).or_insert(1);
                }
            });
            Some((char, set))
        })
        .map(|(xor_char, set)| (xor_char, set.into_values().sum::<i32>()))
        .collect::<BTreeMap<_, _>>();

    Ok(res
        .into_iter()
        .sorted_by(|prev, next| Ord::cmp(&next.1, &prev.1))
        .take(5)
        .map(|(xor_char, _)| xor_encoded_str.as_ref().fixed_xor_byte(xor_char))
        .flat_map(String::from_utf8)
        .collect())
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
