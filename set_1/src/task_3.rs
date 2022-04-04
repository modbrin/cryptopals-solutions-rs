/// Task: Brute single-byte XOR
use lazy_static::lazy_static;

use std::cmp::Ordering;
use std::collections::HashMap;

lazy_static! {
    pub static ref ENGLISH_FREQUENCIES: HashMap<char, f32> = {
        [
            ('a', 0.0651738),
            ('b', 0.0124248),
            ('c', 0.0217339),
            ('d', 0.0349835),
            ('e', 0.1041442),
            ('f', 0.0197881),
            ('g', 0.0158610),
            ('h', 0.0492888),
            ('i', 0.0558094),
            ('j', 0.0009033),
            ('k', 0.0050529),
            ('l', 0.0331490),
            ('m', 0.0202124),
            ('n', 0.0564513),
            ('o', 0.0596302),
            ('p', 0.0596302),
            ('q', 0.0008606),
            ('r', 0.0497563),
            ('s', 0.0515760),
            ('t', 0.0729357),
            ('u', 0.0225134),
            ('v', 0.0082903),
            ('w', 0.0171272),
            ('x', 0.0013692),
            ('y', 0.0145984),
            ('z', 0.0007836),
            (' ', 0.1918182),
        ]
        .into_iter()
        .collect()
    };
}

pub fn rate_english_frequency<T: AsRef<[u8]>>(guess: T) -> f32 {
    guess
        .as_ref()
        .iter()
        .map(|ch| ENGLISH_FREQUENCIES.get(&(*ch as char)).unwrap_or(&0.0))
        .sum()
}

// ONLY WORKS FOR ASCII INPUT
pub fn brute_single_byte_xor<T: AsRef<[u8]>>(data: T) -> Option<(u8, Vec<u8>, f32)> {
    (0u8..=255)
        .map(|k| {
            let guess: Vec<u8> = data.as_ref().iter().map(|b| b ^ k).collect();
            let rating = rate_english_frequency(&guess);
            (guess, k, rating)
        })
        .max_by(|a, b| a.2.partial_cmp(&b.2).unwrap_or(Ordering::Equal))
        .map(|(vec, key, rating)| (key, vec.into_iter().collect(), rating))
}

pub fn brute_single_byte_xor_str(hex: &str) -> Option<(u8, String, f32)> {
    let bytes = hex::decode(hex).ok()?;
    brute_single_byte_xor(&bytes)
        .map(|(key, guess, rating)| (key, guess.into_iter().map(|b| b as char).collect(), rating))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn break_single_byte_xor_should_pass() {
        assert_eq!(
            brute_single_byte_xor_str(
                "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736"
            ),
            Some((
                88u8,
                "Cooking MC\'s like a pound of bacon".to_owned(),
                2.2462904
            ))
        );
    }
}
