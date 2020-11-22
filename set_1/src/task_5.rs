/// Task: Repeating-key XOR

pub fn repeating_key_xor(plaintext: &str, key: &str) -> String {
    let vec: Vec<_> = plaintext
        .chars()
        .enumerate()
        .map(|(i, ch)| ch as u8 ^ key.as_bytes()[i % key.len()])
        .collect();
    hex::encode(vec)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_single_byte_xor_should_pass() {
        assert_eq!(
            repeating_key_xor(
                "Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal",
                "ICE"
            ),
            "0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f"
        );
    }
}
