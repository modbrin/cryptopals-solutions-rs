// Task: Byte-at-a-time ECB decryption
use crate::prelude::encrypt_aes_ecb;

pub fn brute_last_byte_in_block(
    cipher_block: &[u8],
    key: &[u8],
    known_prefix: &[u8],
) -> Option<u8> {
    for b in 0..=255u8 {
        let mut cleartext = known_prefix.iter().copied().collect::<Vec<_>>();
        cleartext.push(b);
        let test_block = encrypt_aes_ecb(cleartext.as_slice(), key);
        if cipher_block == test_block.as_slice() {
            return Some(b);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use crate::prelude::{pad_to_block_size, random_bytes};
    use set_1::task_7::AES_BLOCK_SIZE;

    const EXPECTED_TEXT: &str = "Rollin' in my 5.0\nWith my rag-top down so my hair can blow\nThe girlies on standby waving just to say hi\nDid you stop? No, I just drove by\n";

    use super::*;
    use set_1::task_1::base64_to_bytes;
    use std::fs::File;
    use std::io::{BufRead, BufReader};

    #[test]
    fn decrypt_aes_cbc_should_pass() {
        let known_key = random_bytes(AES_BLOCK_SIZE);

        let file = File::open("res/task12.txt").expect("Failed to open file.");
        let mut file_content = String::new();
        BufReader::new(file)
            .lines()
            .for_each(|line| file_content.push_str(line.unwrap_or(String::from("")).as_str()));

        let known_text = base64_to_bytes(&file_content).expect("Failed to decode base64.");
        let padded_cipher = pad_to_block_size(known_text.as_slice(), AES_BLOCK_SIZE);
        let cipher = encrypt_aes_ecb(padded_cipher.as_slice(), known_key.as_slice());

        let mut probe_block = vec![42u8; AES_BLOCK_SIZE - 1];
        let mut decrypted_message = Vec::new();
        let mut decrypted_block = Vec::new();
        for block_i in 0..cipher.len() / AES_BLOCK_SIZE {
            for byte_i in 1..=16 {
                if decrypted_message.len() + decrypted_block.len() >= known_text.len() {
                    break;
                }
                let prefix = vec![42u8; AES_BLOCK_SIZE - byte_i];
                let prefixed_cipher = prefix
                    .iter()
                    .copied()
                    .chain(known_text.iter().copied())
                    .collect::<Vec<_>>();
                let padded_cipher = pad_to_block_size(prefixed_cipher.as_slice(), AES_BLOCK_SIZE);
                let cipher = encrypt_aes_ecb(padded_cipher.as_slice(), known_key.as_slice());
                let cipher_ref = cipher.as_slice();
                let byte = brute_last_byte_in_block(
                    &cipher_ref[block_i * AES_BLOCK_SIZE..(block_i + 1) * AES_BLOCK_SIZE],
                    known_key.as_slice(),
                    probe_block.as_slice(),
                )
                .expect("Brute failed");
                decrypted_block.push(byte);
                probe_block.remove(0);
                probe_block.push(byte);
            }
            probe_block = decrypted_block.iter().skip(1).copied().collect::<Vec<_>>();
            decrypted_message.extend(decrypted_block.iter().copied());
            decrypted_block.clear();
        }

        let decrypted_text = decrypted_message
            .iter()
            .map(|&v| v as char)
            .collect::<String>();

        assert_eq!(decrypted_text.as_str(), EXPECTED_TEXT);
    }
}
