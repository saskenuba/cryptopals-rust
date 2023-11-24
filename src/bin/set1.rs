use std::collections::BTreeMap;

use cryptopals::hex::ToHex;
use cryptopals::xor::fixed_xor_byte;
use itertools::Itertools;

static ENGLISH_LETTERS: &[char] = &['e', 't', 'a', 'o', 'i', 'n', 's', 'r', 'h', 'l'];

fn challenge_3() -> Vec<String> {
    let decipher_str = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736"
        .to_hex()
        .unwrap();
    let char_range = 0..=255u8;

    let res = char_range
        .into_iter()
        .filter_map(|char| {
            let decipher_xor = fixed_xor_byte(decipher_str.as_ref(), char).unwrap();
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

    res.into_iter()
        .sorted_by(|prev, next| Ord::cmp(&next.1, &prev.1))
        .take(5)
        .flat_map(|(xor_char, _)| fixed_xor_byte(decipher_str.as_ref(), xor_char))
        .flat_map(String::from_utf8)
        .collect()
}

pub fn main() {
    let res = &challenge_3()[1];
    assert_eq!(&*res, "Cooking MC's like a pound of bacon");
}
