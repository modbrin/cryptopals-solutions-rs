/// Task: base64 encoding/decoding implementation

const BASE64_CHARS: [char; 64] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l',
    'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '0', '1', '2', '3', '4',
    '5', '6', '7', '8', '9', '+', '/',
];

#[inline]
fn lookup_base64(octet: u8) -> char {
    BASE64_CHARS[octet as usize]
}

#[inline]
fn lookup_index(ch: char) -> Result<u8, String> {
    Ok(match ch {
        'A'..='Z' => ch as u8 - 65,
        'a'..='z' => ch as u8 + 26 - 97,
        '0'..='9' => ch as u8 + 52 - 48,
        '+' => 62u8,
        '/' => 63u8,
        _ => return Err("Invalid base64 char encountered".to_owned()),
    })
}

fn extract_first_char(octet_a: u8) -> u8 {
    (octet_a & 0b11111100) >> 2
}

fn extract_second_char(octet_a: u8, octet_b: u8) -> u8 {
    (octet_a & 0b00000011) << 4 | (octet_b & 0b11110000) >> 4
}

fn extract_third_char(octet_b: u8, octet_c: u8) -> u8 {
    (octet_b & 0b00001111) << 2 | (octet_c & 0b11000000) >> 6
}

fn extract_fourth_char(octet_c: u8) -> u8 {
    octet_c & 0b00111111
}

/// Encode bytes representation to base64 char vector.
pub fn bytes_to_base64(input: &[u8]) -> Vec<char> {
    let mut out = Vec::new();
    let mut index = 0;
    // process main part
    while index + 3 <= input.len() {
        out.push(lookup_base64(extract_first_char(input[index])));
        out.push(lookup_base64(extract_second_char(
            input[index],
            input[index + 1],
        )));
        out.push(lookup_base64(extract_third_char(
            input[index + 1],
            input[index + 2],
        )));
        out.push(lookup_base64(extract_fourth_char(input[index + 2])));
        index += 3;
    }
    // process remaining chars and put filler `=`
    match input.len() - index {
        1 => {
            out.push(lookup_base64(extract_first_char(input[index])));
            out.push(lookup_base64(extract_second_char(input[index], 0u8)));
            out.push('=');
            out.push('=');
        }
        2 => {
            out.push(lookup_base64(extract_first_char(input[index])));
            out.push(lookup_base64(extract_second_char(
                input[index],
                input[index + 1],
            )));
            out.push(lookup_base64(extract_third_char(input[index + 1], 0u8)));
            out.push('=');
        }
        _ => (),
    }
    out
}

/// Encode hex representation to base64 string.
pub fn hex_to_base64(input: &str) -> Result<String, String> {
    let res = hex::decode(input).map_err(|_| "Decoding from hex failed.")?;
    Ok(bytes_to_base64(&res).into_iter().collect())
}

fn extract_first_octet(char_a: u8, char_b: u8) -> u8 {
    (char_a & 0b00111111) << 2 | (char_b & 0b00110000) >> 4
}

fn extract_second_octet(char_b: u8, char_c: u8) -> u8 {
    (char_b & 0b00001111) << 4 | (char_c & 0b00111100) >> 2
}

fn extract_third_octet(char_c: u8, char_d: u8) -> u8 {
    (char_c & 0b00000011) << 6 | (char_d & 0b00111111)
}

/// Convert base64-encoded char slice input to original byte representation.
pub fn base64_to_bytes(input: &[char]) -> Result<Vec<u8>, String> {
    let mut out = Vec::new();
    let mut index = 0;
    // process main part
    while index + 4 < input.len() {
        out.push(extract_first_octet(
            lookup_index(input[index])?,
            lookup_index(input[index + 1])?,
        ));
        out.push(extract_second_octet(
            lookup_index(input[index + 1])?,
            lookup_index(input[index + 2])?,
        ));
        out.push(extract_third_octet(
            lookup_index(input[index + 2])?,
            lookup_index(input[index + 3])?,
        ));
        index += 4;
    }
    // process remaining octets
    if !input.is_empty() {
        if input.len() % 4 != 0 {
            return Err("Wrong format, length must be a multiple of 4".to_owned());
        }
        out.push(extract_first_octet(
            lookup_index(input[index])?,
            lookup_index(input[index + 1])?,
        ));
        match (input[index + 2], input[index + 3]) {
            ('=', '=') => (), // one octet, already extracted
            (_, '=') => {
                // two octets, first is already extracted
                out.push(extract_second_octet(
                    lookup_index(input[index + 1])?,
                    lookup_index(input[index + 2])?,
                ));
            }
            _ => {
                out.push(extract_second_octet(
                    lookup_index(input[index + 1])?,
                    lookup_index(input[index + 2])?,
                ));
                out.push(extract_third_octet(
                    lookup_index(input[index + 2])?,
                    lookup_index(input[index + 3])?,
                ));
            }
        }
    }
    Ok(out)
}

/// Convert base64-encoded string input to original hex representation.
pub fn base64_to_hex(input: &str) -> Result<String, String> {
    // TODO: don't use another owned intermediary in convertion
    let input_converted: Vec<char> = input.chars().collect();
    let data = base64_to_bytes(&input_converted)?;
    Ok(hex::encode(data))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn general_usecases_to_base64_should_pass() {
        assert_eq!(hex_to_base64(""), Ok("".to_owned()));
        assert_eq!(hex_to_base64("66"), Ok("Zg==".to_owned()));
        assert_eq!(hex_to_base64("666f"), Ok("Zm8=".to_owned()));
        assert_eq!(hex_to_base64("666f6f"), Ok("Zm9v".to_owned()));
        assert_eq!(hex_to_base64("666f6f62"), Ok("Zm9vYg==".to_owned()));
        assert_eq!(hex_to_base64("666f6f6261"), Ok("Zm9vYmE=".to_owned()));
        assert_eq!(hex_to_base64("666f6f626172"), Ok("Zm9vYmFy".to_owned()));
        assert_eq!(hex_to_base64(
		"49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d"),
		Ok("SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t".to_owned()));
    }

    #[test]
    fn general_usecases_from_base64_should_pass() {
        assert_eq!(base64_to_hex(""), Ok("".to_owned()));
        assert_eq!(base64_to_hex("Zg=="), Ok("66".to_owned()));
        assert_eq!(base64_to_hex("Zm8="), Ok("666f".to_owned()));
        assert_eq!(base64_to_hex("Zm9v"), Ok("666f6f".to_owned()));
        assert_eq!(base64_to_hex("Zm9vYg=="), Ok("666f6f62".to_owned()));
        assert_eq!(base64_to_hex("Zm9vYmE="), Ok("666f6f6261".to_owned()));
        assert_eq!(base64_to_hex("Zm9vYmFy"), Ok("666f6f626172".to_owned()));
        assert_eq!(base64_to_hex("SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t"),
        Ok("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d".to_owned()));
    }
}
