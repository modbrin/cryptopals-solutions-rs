// Task: ECB cut-and-paste
use crate::prelude::{encrypt_aes_ecb, pad_to_block_size};
use set_1::task_7::{decrypt_aes_ecb, AES_BLOCK_SIZE};
use std::collections::HashMap;

pub fn encode_to_kv_sequence(data: &Vec<(String, String)>) -> String {
    let sanitize = |s: &str| -> String { s.replace('&', "").replace('=', "") };
    let strs: Vec<_> = data
        .iter()
        .map(|(k, v)| format!("{}={}", sanitize(k), sanitize(v)))
        .collect();
    strs.join("&")
}

pub fn parse_kv_sequence(input: &str) -> HashMap<String, String> {
    let mut result = HashMap::new();
    for pair in input.split('&') {
        let mut kv = pair.split('=');
        match (kv.next(), kv.next()) {
            (Some(key), Some(val)) => {
                result.insert(key.to_string(), val.to_string());
            }
            _ => (),
        }
    }
    result
}

pub fn profile_for(email: &str) -> String {
    let profile = vec![
        ("email".to_string(), email.to_string()),
        ("uid".to_string(), "10".to_string()),
        ("role".to_string(), "user".to_string()),
    ];
    encode_to_kv_sequence(&profile)
}

pub fn encrypt_kv(input: &str, key: impl AsRef<[u8]>) -> Vec<u8> {
    let padded = pad_to_block_size(input.bytes().collect::<Vec<_>>().as_slice(), AES_BLOCK_SIZE);
    encrypt_aes_ecb(padded.as_slice(), key.as_ref())
}

pub fn decrypt_kv<T: AsRef<[u8]>>(cipher: T, key: T) -> String {
    decrypt_aes_ecb(cipher, key)
        .iter()
        .map(|&v| v as char)
        .collect::<String>()
}

#[cfg(test)]
mod tests {
    use crate::prelude::random_bytes;

    use super::*;

    #[test]
    fn encode_to_kv_sequence_should_pass() {
        let encoded = encode_to_kv_sequence(&vec![
            ("KeyA".to_string(), "ValA".to_string()),
            ("KeyB".to_string(), "ValB".to_string()),
            ("KeyC=A&KeyB=B".to_string(), "ValC=A&ValB=B".to_string()),
        ]);
        assert_eq!(
            encoded.as_str(),
            "KeyA=ValA&KeyB=ValB&KeyCAKeyBB=ValCAValBB"
        );
    }

    #[test]
    fn parse_kv_sequence_should_pass() {
        let parsed = parse_kv_sequence("KeyA=ValA&KeyB=ValB&KeyC=ValC");
        assert_eq!(parsed.get(&"KeyA".to_string()), Some(&"ValA".to_string()));
        assert_eq!(parsed.get(&"KeyB".to_string()), Some(&"ValB".to_string()));
        assert_eq!(parsed.get(&"KeyC".to_string()), Some(&"ValC".to_string()));
    }

    #[test]
    fn cut_paste() {
        let known_key = random_bytes(AES_BLOCK_SIZE);
        // email=berloga@babros.eu&uid=10&role=user
        // email=aaa@bbb.ccadmin\x0b\x0b\x0b\x0b\x0b\x0b\x0b\x0b\x0b\x0b\x0b&uid=10&role=user
        let cut_profile =
            profile_for("aaa@bbb.ccadmin\x0b\x0b\x0b\x0b\x0b\x0b\x0b\x0b\x0b\x0b\x0b");
        let cut_encrypted = encrypt_kv(cut_profile.as_str(), &known_key);
        let cut_block = &cut_encrypted.as_slice()[16..32];
        // email=aaa@bbccdd.ee&uid=10&role=user
        let paste_profile = profile_for("aaa@bbccdd.ee");
        let mut paste_encrypted = encrypt_kv(paste_profile.as_str(), &known_key);
        for i in 32usize..48 {
            paste_encrypted[i] = cut_block[i - 32];
        }

        let decrypted = decrypt_kv(&paste_encrypted, &known_key);
        let vals = parse_kv_sequence(decrypted.as_str());
        println!("{:?}", decrypted);
        assert_eq!(vals.len(), 3);
        assert_eq!(
            vals.get(&"email".to_string()),
            Some(&"aaa@bbccdd.ee".to_string())
        );
        assert_eq!(vals.get(&"uid".to_string()), Some(&"10".to_string()));
        assert_eq!(vals.get(&"role".to_string()), Some(&"admin".to_string()));
    }
}
