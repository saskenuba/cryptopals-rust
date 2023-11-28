use std::collections::BTreeMap;

use itertools::Itertools;

use crate::hex::Hex;
use crate::xor::XorExt;
use crate::AnyResult;

static ENGLISH_LETTERS: &[char] = &['e', 't', 'a', 'o', 'i', 'n', 's', 'h', 'r', 'd'];

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
