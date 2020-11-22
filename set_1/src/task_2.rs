/// Task: Fixed XOR

pub fn fixed_length_xor(bytes_a: &[u8], bytes_b: &[u8]) -> Result<Vec<u8>, String> {
    if bytes_a.len() != bytes_b.len() {
        return Err("Input slices must be of same length".to_owned());
    }
    Ok(bytes_a
        .iter()
        .zip(bytes_b.iter())
        .map(|(byte_a, byte_b)| byte_a ^ byte_b)
        .collect())
}

pub fn fixed_length_xor_str(hex_a: &str, hex_b: &str) -> Result<String, String> {
    let bytes_a = hex::decode(hex_a).map_err(|_| "Decoding from hex failed for hex_a.")?;
    let bytes_b = hex::decode(hex_b).map_err(|_| "Decoding from hex failed for hex_b.")?;
    fixed_length_xor(&bytes_a, &bytes_b).map(hex::encode)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_xor_should_pass() {
        assert_eq!(
            fixed_length_xor_str(
                "1c0111001f010100061a024b53535009181c",
                "686974207468652062756c6c277320657965",
            ),
            Ok("746865206b696420646f6e277420706c6179".to_owned()),
        );
    }
}
