use std::io::BufRead;
use std::ops::Deref;

use bitvec::macros::internal::funty::Fundamental;
use cryptopals::bytes::transpose_bytes_block;
use cryptopals::challenges::set1::{bruteforce_hex_by_fixed_xor, hamming_distance_bit};
use cryptopals::hex::Hex;
use itertools::Itertools;

fn challenge_3() {
    let challenge3_str = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
    let res = bruteforce_hex_by_fixed_xor(challenge3_str.parse::<Hex>().unwrap().deref(), None).unwrap();
    // dbg!(&res);
    assert_eq!(&res[0].1, "Cooking MC's like a pound of bacon");
}

fn challenge_4() {
    let flatten = include_bytes!("../../assets/set1-challenge4.txt")
        .lines()
        .map_while(Result::ok);

    let a: Vec<_> = flatten
        .flat_map(|new_str| new_str.parse::<Hex>())
        .flat_map(|hex_str| bruteforce_hex_by_fixed_xor(hex_str.deref(), None))
        .filter(|a| !a.is_empty())
        .collect();

    let res = a.get(3).and_then(|a| a.first()).unwrap();
    assert_eq!(res.1, "Now that the party is jumping\n");
}

fn challenge_6() {
    let decoded_cipher_str = Hex::from_b64_encoded_string(include_str!("../../assets/set1-challenge6.txt")).unwrap();
    let normalization_blocks_qty = 2;
    let top_n_keys_qty = 5;

    let mut buffer = Vec::new();
    let keysizes = 1..=40;
    for potential_keysize in keysizes {
        let distance: i32 = decoded_cipher_str
            .chunks_exact(potential_keysize)
            .take(normalization_blocks_qty)
            .tuple_windows()
            .map(|(a, b)| hamming_distance_bit(a, b).unwrap() / potential_keysize as i32)
            .sum();
        buffer.push((potential_keysize, distance));
    }

    let probable_keysizes: Vec<_> = buffer
        .into_iter()
        .sorted_by(|(_, a), (_, b)| a.cmp(b))
        .filter_map(|(size, _)| if size < 10 { Some(size) } else { None })
        .take(top_n_keys_qty)
        .collect();

    // transposing blocks helps to exploit the repeating nature of the key.
    // we effectively create "sub cipher texts", for each N char of the possible key, to decrypt it after with fixed_xor

    let cipherblocks_by_keysize: Vec<_> = probable_keysizes
        .into_iter()
        .map(|c| (c, transpose_bytes_block(&decoded_cipher_str, c as i32)))
        .collect();

    for (keysize, sub_cipherblocks) in cipherblocks_by_keysize {
        println!("=== start of keysize {} ===", keysize);
        let bruteforced_cipherblocks = sub_cipherblocks
            .into_iter()
            .flat_map(|cipherblock| bruteforce_hex_by_fixed_xor(cipherblock, Some(u8::MIN..=u8::MAX)))
            .collect_vec();

        let a = bruteforced_cipherblocks
            .into_iter()
            .flat_map(|c| c.get(0).cloned())
            .map(|(chr, decoded)| (chr.as_char().unwrap(), decoded))
            .map(|(c, _)| c)
            .collect::<String>();

        println!("Possible key: {}", &a);
        // println!(
        //     "{}",
        //     String::from_utf8(decoded_cipher_str.repeated_xor_key(a.as_bytes())).unwrap()
        // );

        println!("=== end of keysize {} ===\n", keysize)
    }
}

pub fn main() {
    challenge_3();
    challenge_4();
    challenge_6()
}
