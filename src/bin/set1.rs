use std::collections::HashMap;
use std::io::BufRead;

use cryptopals::challenges::set1;
use cryptopals::challenges::set1::hamming_distance_bit;
use cryptopals::hex::Hex;
use itertools::Itertools;

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

fn challenge_6() {
    let decoded_cipher_str = Hex::from_b64_encoded_string(include_str!("../../assets/set1-challenge6.txt")).unwrap();

    let mut buffer = Vec::new();
    let keysizes = 1..=50;
    for size in keysizes {
        let blocks = 5;
        let distance: i32 = decoded_cipher_str
            .chunks(size)
            .take(blocks)
            .tuple_windows()
            .map(|(a, b)| hamming_distance_bit(a, b).unwrap() / size as i32)
            .sum();
        buffer.push((size, distance / blocks as i32));
    }
    buffer.sort_by(|(_, a), (_, b)| a.cmp(b));

    let first_n_keys = 5;
    let probable_keysizes: Vec<_> = buffer
        .into_iter()
        .map(|(size, _)| size)
        .take(first_n_keys)
        .collect();

    // transposing blocks helps to exploit the repeating nature of the key.
    // we effectively create "sub cipher texts", for each N char of the possible key, to decrypt it after with fixed_xor

    let mut buff = Vec::with_capacity(probable_keysizes.len());
    for keysize in probable_keysizes {
        let idx_vec: Vec<_> = (0..keysize)
            .map(|idx| {
                [idx]
                    .repeat(decoded_cipher_str.len())
                    .into_iter()
                    .enumerate()
                    .map(|(idx, lookup_idx)| lookup_idx + keysize * idx)
                    .filter(|c| c <= &decoded_cipher_str.len())
            })
            .map(|idx_vec| {
                idx_vec
                    .into_iter()
                    .flat_map(|idx| decoded_cipher_str.get(idx))
                    .collect_vec()
            })
            .collect();
        buff.push((keysize, idx_vec));
    }
    // let (keysize, blocks) = &buff[1];
    // println!("keysize: {:?}, #qty of blocks {:?}", keysize, blocks.len());
}

pub fn main() {
    challenge_3();
    challenge_4();
    challenge_6()
}
