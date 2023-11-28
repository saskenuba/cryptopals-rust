use std::io::BufRead;

use cryptopals::challenges::set1;
use cryptopals::hex::Hex;

fn challenge_3() {
    let challenge3_str = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
    let res = set1::decode_hex_str_to_english(challenge3_str.parse::<Hex>().unwrap()).unwrap();
    assert_eq!(&res[0], "Cooking MC's like a pound of bacon");
}

fn challenge_4() {
    let flatten = include_bytes!("../../assets/set1-challenge4.txt")
        .lines()
        .flatten();

    let a: Vec<_> = flatten
        .flat_map(|new_str| new_str.parse::<Hex>())
        .flat_map(|hex_str| set1::decode_hex_str_to_english(hex_str))
        .filter(|a| !a.is_empty())
        .collect();

    let res = &a[3][0];
    assert_eq!(res, "Now that the party is jumping\n");
}

pub fn main() {
    challenge_3();
    challenge_4();
}
