/// Task: Implement padding

pub fn pad_to_size<T: AsRef<[u8]>>(input: T, size: usize) -> Vec<u8> {
    let input_ref = input.as_ref();
    let pad_size = (size - (input_ref.len() % size)) % size;
    input_ref
        .iter()
        .cloned()
        .chain((0..pad_size).map(|_| b'\x04'))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decipher_aes_ecb_should_pass() {
        assert_eq!(
            pad_to_size(b"YELLOW SUBMARINE", 20),
            b"YELLOW SUBMARINE\x04\x04\x04\x04"
        );
        assert_eq!(pad_to_size(b"12345", 3), b"12345\x04");
        assert_eq!(pad_to_size(b"12", 2), b"12");
        assert_eq!(pad_to_size(b"12345678", 2), b"12345678");
        assert_eq!(pad_to_size(b"1234567", 5), b"1234567\x04\x04\x04");
        assert_eq!(pad_to_size(b"", 3), b"");
    }
}
