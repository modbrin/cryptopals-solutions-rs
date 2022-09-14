/// Task: Implement pkcs#7 padding

pub fn pad_to_block_size<T: AsRef<[u8]>>(input: T, block_size: usize) -> Vec<u8> {
    let input_ref = input.as_ref();
    let pad_size = block_size - input_ref.len() % block_size;
    input_ref
        .iter()
        .cloned()
        .chain((0..pad_size).map(|_| pad_size as u8))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pad_to_block_size_should_pass() {
        assert_eq!(
            pad_to_block_size(b"YELLOW SUBMARINE", 20),
            b"YELLOW SUBMARINE\x04\x04\x04\x04"
        );
        assert_eq!(pad_to_block_size(b"12345", 3), b"12345\x01");
        assert_eq!(pad_to_block_size(b"12", 2), b"12\x02\x02");
        assert_eq!(pad_to_block_size(b"12345678", 2), b"12345678\x02\x02");
        assert_eq!(pad_to_block_size(b"1234567", 5), b"1234567\x03\x03\x03");
        assert_eq!(pad_to_block_size(b"123456", 5), b"123456\x04\x04\x04\x04");
        assert_eq!(pad_to_block_size(b"", 3), b"\x03\x03\x03");
    }
}
