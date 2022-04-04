/// Task: Detect AES in ECB mode
use std::collections::HashSet;

pub fn find_aes_ecb_repeats<T: AsRef<[u8]>>(cipher: T) -> u32 {
    let mut occurances = HashSet::new();
    let cipher_ref = cipher.as_ref();
    const BLOCK_SIZE_AES_ECB: usize = 16; // Cipher::aes_128_ecb().block_size();
    let mut repeats: u32 = 0;
    for block in cipher_ref.as_chunks::<BLOCK_SIZE_AES_ECB>().0.iter() {
        // ignore remainder, it's unique anyway
        if occurances.contains(block) {
            repeats += 1;
        } else {
            occurances.insert(block);
        }
    }
    repeats
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::{BufRead, BufReader};

    #[test]
    fn decipher_aes_ecb_should_pass() {
        let file = File::open("res/task8.txt").expect("Failed to open file.");
        // let mut file_content = String::new();

        let mut likely_ecb_cipher = String::new();
        let mut max_repeats: u32 = 0;

        for line in BufReader::new(file).lines() {
            let line = line.unwrap();
            let repeats = find_aes_ecb_repeats(hex::decode(&line).expect("Malformed hex input."));
            if repeats > max_repeats {
                max_repeats = repeats;
                likely_ecb_cipher = line;
            }
        }

        let answer = "d880619740a8a19b7840a8a31c810a3d08649af70dc06f4fd5d2d69c744cd283e2dd052f6b641dbf9d11b0348542bb5708649af70dc06f4fd5d2d69c744cd2839475c9dfdbc1d46597949d9c7e82bf5a08649af70dc06f4fd5d2d69c744cd28397a93eab8d6aecd566489154789a6b0308649af70dc06f4fd5d2d69c744cd283d403180c98c8f6db1f2a3f9c4040deb0ab51b29933f2c123c58386b06fba186a";
        assert_eq!((likely_ecb_cipher.as_str(), max_repeats), (answer, 3u32));
    }
}
