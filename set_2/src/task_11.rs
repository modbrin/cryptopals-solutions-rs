// Task: Detect AES MODE, ECB or CBC
use crate::prelude::pad_to_block_size;
use openssl::symm::{Cipher, Crypter, Mode};
use rand::Rng;
use set_1::task_7::AES_BLOCK_SIZE;
use std::collections::BTreeSet;

pub fn encrypt_aes_ecb<T: AsRef<[u8]>>(cleartext: T, key: T) -> Vec<u8> {
    let cleartext_ref = cleartext.as_ref();
    let key_ref = key.as_ref();
    let mut output = vec![0u8; cleartext_ref.len() + AES_BLOCK_SIZE];
    let mut crypter = Crypter::new(Cipher::aes_128_ecb(), Mode::Encrypt, key_ref, None)
        .expect("failed to construct crypter");
    let count = crypter
        .update(cleartext_ref, output.as_mut())
        .expect("crypter update action failed");
    output.truncate(count);
    output
}

pub fn encrypt_aes_cbc<T: AsRef<[u8]>>(cleartext: T, key: T, iv: T) -> Vec<u8> {
    let cleartext_ref = cleartext.as_ref();
    let key_ref = key.as_ref();
    let iv_ref = iv.as_ref();
    assert_eq!(iv_ref.len(), AES_BLOCK_SIZE, "IV must be of block size");

    let mut output = vec![0u8; cleartext_ref.len() + AES_BLOCK_SIZE];
    let mut crypter = Crypter::new(Cipher::aes_128_cbc(), Mode::Encrypt, key_ref, Some(iv_ref))
        .expect("failed to construct crypter");
    let count = crypter
        .update(cleartext_ref, output.as_mut())
        .expect("crypter update action failed");
    output.truncate(count);
    output
}

pub fn random_bytes(count: usize) -> Vec<u8> {
    (0..count).map(|_| rand::random::<u8>()).collect()
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AesMode {
    /// Electronic CodeBook
    ECB,
    /// Cipher Block Chaining
    CBC,
}

pub fn encrypt_message<T: AsRef<[u8]>>(cleartext: T) -> (Vec<u8>, AesMode) {
    let append_front = rand::thread_rng().gen_range(5..=10);
    let append_back = rand::thread_rng().gen_range(5..=10);
    let cleartext_bytes = random_bytes(append_front)
        .into_iter()
        .chain(cleartext.as_ref().iter().copied())
        .chain(random_bytes(append_back).into_iter())
        .collect::<Vec<_>>();
    let random_key = random_bytes(AES_BLOCK_SIZE);
    let padded_input = pad_to_block_size(cleartext_bytes.as_slice(), AES_BLOCK_SIZE);
    if rand::random::<bool>() {
        let cipher = encrypt_aes_ecb(padded_input.as_slice(), random_key.as_slice());
        (cipher, AesMode::ECB)
    } else {
        let random_iv = random_bytes(AES_BLOCK_SIZE);
        let cipher = encrypt_aes_cbc(
            padded_input.as_slice(),
            random_key.as_slice(),
            random_iv.as_slice(),
        );
        (cipher, AesMode::CBC)
    }
}

pub fn detect_aes_mode<T: AsRef<[u8]>>(cipher: T) -> AesMode {
    let cipher_ref = cipher.as_ref();
    let mut known_blocks = BTreeSet::<&[u8]>::new();
    assert_eq!(
        cipher_ref.len() % AES_BLOCK_SIZE,
        0,
        "Ciphertext size is not aligned to block size"
    );
    for i in 0..cipher_ref.len() / AES_BLOCK_SIZE {
        let block = &cipher_ref[i * AES_BLOCK_SIZE..(i + 1) * AES_BLOCK_SIZE];
        if known_blocks.contains(block) {
            return AesMode::ECB;
        }
        known_blocks.insert(block);
    }
    AesMode::CBC
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude::*;
    use set_1::prelude::*;
    use std::{
        fs::File,
        io::{BufRead, BufReader},
    };

    const COMMON_TEXT: &str = "I'm back and I'm ringin' the bell \nA rockin' on the mike while the fly girls yell \nIn ecstasy in the back of me \nWell that's my DJ Deshay cuttin' all them Z's \nHittin' hard and the girlies goin' crazy \nVanilla's on the mike, man I'm not lazy. \n\nI'm lettin' my drug kick in \nIt controls my mouth and I begin \nTo just let it flow, let my concepts go \nMy posse's to the side yellin', Go Vanilla Go! \n\nSmooth 'cause that's the way I will be \nAnd if you don't give a damn, then \nWhy you starin' at me \nSo get off 'cause I control the stage \nThere's no dissin' allowed \nI'm in my own phase \nThe girlies sa y they love me and that is ok \nAnd I can dance better than any kid n' play \n\nStage 2 -- Yea the one ya' wanna listen to \nIt's off my head so let the beat play through \nSo I can funk it up and make it sound good \n1-2-3 Yo -- Knock on some wood \nFor good luck, I like my rhymes atrocious \nSupercalafragilisticexpialidocious \nI'm an effect and that you can bet \nI can take a fly girl and make her wet. \n\nI'm like Samson -- Samson to Delilah \nThere's no denyin', You can try to hang \nBut you'll keep tryin' to get my style \nOver and over, practice makes perfect \nBut not if you're a loafer. \n\nYou'll get nowhere, no place, no time, no girls \nSoon -- Oh my God, homebody, you probably eat \nSpaghetti with a spoon! Come on and say it! \n\nVIP. Vanilla Ice yep, yep, I'm comin' hard like a rhino \nIntoxicating so you stagger like a wino \nSo punks stop trying and girl stop cryin' \nVanilla Ice is sellin' and you people are buyin' \n'Cause why the freaks are jockin' like Crazy Glue \nMovin' and groovin' trying to sing along \nAll through the ghetto groovin' this here song \nNow you're amazed by the VIP posse. \n\nSteppin' so hard like a German Nazi \nStartled by the bases hittin' ground \nThere's no trippin' on mine, I'm just gettin' down \nSparkamatic, I'm hangin' tight like a fanatic \nYou trapped me once and I thought that \nYou might have it \nSo step down and lend me your ear \n'89 in my time! You, '90 is my year. \n\nYou're weakenin' fast, YO! and I can tell it \nYour body's gettin' hot, so, so I can smell it \nSo don't be mad and don't be sad \n'Cause the lyrics belong to ICE, You can call me Dad \nYou're pitchin' a fit, so step back and endure \nLet the witch doctor, Ice, do the dance to cure \nSo come up close and don't be square \nYou wanna battle me -- Anytime, anywhere \n\nYou thought that I was weak, Boy, you're dead wrong \nSo come on, everybody and sing this song \n\nSay -- Play that funky music Say, go white boy, go white boy go \nplay that funky music Go white boy, go white boy, go \nLay down and boogie and play that funky music till you die. \n\nPlay that funky music Come on, Come on, let me hear \nPlay that funky music white boy you say it, say it \nPlay that funky music A little louder now \nPlay that funky music, white boy Come on, Come on, Come on \nPlay that funky music \n";

    #[test]
    fn test_encryption_validity_ecb() {
        let input_padded = pad_to_block_size(COMMON_TEXT.as_bytes(), AES_BLOCK_SIZE);
        let key: &[u8] = "YELLOW SUBMARINE".as_bytes();

        let file = File::open("../set_1/res/task7.txt").expect("Failed to open file.");
        let mut file_content = String::new();
        BufReader::new(file)
            .lines()
            .for_each(|line| file_content.push_str(line.unwrap_or(String::from("")).as_str()));

        let cipher = base64_to_bytes(&file_content).expect("Failed to decode base64.");
        let cipher_comp = encrypt_aes_ecb(input_padded.as_slice(), key);
        assert_eq!(cipher, cipher_comp);
    }

    #[test]
    fn test_encryption_validity_cbc() {
        let input_padded = pad_to_block_size(COMMON_TEXT.as_bytes(), AES_BLOCK_SIZE);
        let key: &[u8] = "YELLOW SUBMARINE".as_bytes();
        let iv = vec![0u8; AES_BLOCK_SIZE];

        let file = File::open("res/task10.txt").expect("Failed to open file.");
        let mut file_content = String::new();
        BufReader::new(file)
            .lines()
            .for_each(|line| file_content.push_str(line.unwrap_or(String::from("")).as_str()));

        let cipher = base64_to_bytes(&file_content).expect("Failed to decode base64.");
        let cipher_comp = encrypt_aes_cbc(input_padded.as_slice(), key, iv.as_slice());
        assert_eq!(cipher, cipher_comp);
    }

    #[test]
    fn test_aes_cbc_ebc_different_output() {
        let input_padded = pad_to_block_size(COMMON_TEXT.as_bytes(), AES_BLOCK_SIZE);
        let key: &[u8] = "YELLOW SUBMARINE".as_bytes();
        let iv = vec![0u8; AES_BLOCK_SIZE];

        let cipher_comp_ecb = encrypt_aes_ecb(input_padded.as_slice(), key);
        let cipher_comp_cbc = encrypt_aes_cbc(input_padded.as_slice(), key, iv.as_slice());
        assert_ne!(cipher_comp_ecb, cipher_comp_cbc);
    }

    #[test]
    fn test_random_generation() {
        assert_eq!(random_bytes(AES_BLOCK_SIZE).len(), AES_BLOCK_SIZE);
        assert_ne!(random_bytes(AES_BLOCK_SIZE), random_bytes(AES_BLOCK_SIZE));
    }

    #[test]
    fn detect_aes_ecb_cbc_mode_should_pass() {
        let malicious_input = vec![100; AES_BLOCK_SIZE * 3];
        for _ in 0..100 {
            let (ciphertext, mode) = encrypt_message(malicious_input.as_slice());
            assert_eq!(detect_aes_mode(ciphertext.as_slice()), mode);
        }
    }
}
