/// Task: base64 encoding/decoding implementation
use core::ops::{Add, Rem, Sub, SubAssign};
use lazy_static::lazy_static;
use num::traits::One;
use std::fmt::Display;

// TASK 1 ##############################################################################

#[derive(Debug, Clone)]
pub enum Base64Error {
    /// Provided base64 input is invalid, i.e. it contains chars outside base64 range.
    ParsingFailed,
    /// Input base64 string is malformed, i.e. size is not multiple of 4.
    AlignmentMismatch,
}

impl Display for Base64Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Base64Error::ParsingFailed => write!(f, "input contains chars outside base64 range"),
            Base64Error::AlignmentMismatch => write!(f, "input size is not multiple of 4"),
        }
    }
}

lazy_static! {
    static ref BASE64_CHARS: [char; 64] = {
        let chars = ('A'..='Z')
            .chain('a'..='z')
            .chain('0'..='9')
            .chain(['+', '/'].into_iter());
        chars.collect::<Vec<_>>().try_into().unwrap()
    };
}

fn base64_to_index(input: char) -> Result<u8, Base64Error> {
    let char_byte: u8 = input.try_into().map_err(|_| Base64Error::ParsingFailed)?;
    match input {
        '+' => Ok(62),
        '/' => Ok(63),
        '0'..='9' => Ok(char_byte - 48 + 52),
        'A'..='Z' => Ok(char_byte - 65),
        'a'..='z' => Ok(char_byte - 97 + 26),
        _ => Err(Base64Error::ParsingFailed),
    }
}

pub fn hex_to_base64(input_hex: &str) -> Result<String, hex::FromHexError> {
    Ok(bytes_to_base64(hex::decode(input_hex)?))
}
pub fn base64_to_hex(input_base64: &str) -> Result<String, Base64Error> {
    Ok(hex::encode(base64_to_bytes(input_base64)?))
}

/// Example: base = 7, num = 3, next_multiple of 3 for 7 = 9
/// Note: suitable only for positive numbers.
pub fn next_multiple<
    T: Copy + Add<Output = T> + Sub<Output = T> + Rem<Output = T> + SubAssign + One,
>(
    base: T,
    num: T,
) -> T {
    let mut multiple = base + num - T::one();
    multiple -= multiple % num;
    multiple
}

pub fn bytes_to_base64<T: AsRef<[u8]>>(input_data: T) -> String {
    let src_ref = input_data.as_ref();
    let mut result = Vec::new();
    result.reserve(next_multiple(src_ref.len(), 3));
    let mut it = src_ref.iter();
    loop {
        let it_0 = it.next();
        let it_1 = it.next();
        let it_2 = it.next();

        if it_0.is_none() {
            break;
        }

        let comp_0 = it_0.map(|ok| *ok >> 2);
        let comp_1 = it_0.map(|ok| 0x3f & *ok << 4 | it_1.map(|v| *v >> 4).unwrap_or(0));
        let comp_2 = it_1.map(|ok| 0x3f & *ok << 2 | it_2.map(|v| *v >> 6).unwrap_or(0));
        let comp_3 = it_2.map(|ok| 0x3f & *ok);

        result.extend(
            vec![comp_0, comp_1, comp_2, comp_3]
                .into_iter()
                .map(|comp| comp.map(|idx| BASE64_CHARS[idx as usize]).unwrap_or('=')),
        );

        if it_2.is_none() {
            break;
        }
    }

    result.into_iter().collect()
}
pub fn base64_to_bytes(input_base64: &str) -> Result<Vec<u8>, Base64Error> {
    let mut it = input_base64.chars();
    let mut result = Vec::new();
    // result.reserve(next_multiple(src_ref.len(), 3));
    loop {
        let it_0 = if let Some(it_in) = it.next() {
            Some(base64_to_index(it_in)?)
        } else {
            break;
        };
        macro_rules! proc_it {
            ($($i: ident), *) => {
                $(
                    let $i = it.next().ok_or(Base64Error::AlignmentMismatch)?;
                    let $i = if $i == '=' {
                        None
                    } else {
                        Some(base64_to_index($i)?)
                    };
                )*
            }
        }
        proc_it!(it_1, it_2, it_3);

        let comp_0 = it_0.map(|ok| ok << 2 | it_1.map(|v| v >> 4).unwrap_or(0));
        let mut comp_1 = it_1.map(|ok| ok << 4 | it_2.map(|v| v >> 2).unwrap_or(0));
        let mut comp_2 = it_2.map(|ok| ok << 6 | it_3.unwrap_or(0));
        if let Some(comp) = comp_1 {
            if comp == 0 && it_2.is_none() {
                comp_1 = None
            }
        };
        if let Some(comp) = comp_2 {
            if comp == 0 && it_3.is_none() {
                comp_2 = None
            }
        };

        macro_rules! proc_comp {
            ($($i: ident),*) => {
                $(
                    if let Some(comp) = $i {
                        result.push(comp);
                    } else {
                        break;
                    }
                )*
            }
        }
        proc_comp!(comp_0, comp_1, comp_2);
    }
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn general_usecases_to_base64_should_pass() {
        assert_eq!(hex_to_base64("").unwrap(), "");
        assert_eq!(hex_to_base64("66").unwrap(), "Zg==");
        assert_eq!(hex_to_base64("666f").unwrap(), "Zm8=");
        assert_eq!(hex_to_base64("666f6f").unwrap(), "Zm9v");
        assert_eq!(hex_to_base64("666f6f62").unwrap(), "Zm9vYg==");
        assert_eq!(hex_to_base64("666f6f6261").unwrap(), "Zm9vYmE=");
        assert_eq!(hex_to_base64("666f6f626172").unwrap(), "Zm9vYmFy");
        assert_eq!(hex_to_base64(
            "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d").unwrap(),
            "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t");
    }
    #[test]
    fn general_usecases_from_base64_should_pass() {
        assert_eq!(base64_to_hex("").unwrap(), "");
        assert_eq!(base64_to_hex("Zg==").unwrap(), "66");
        assert_eq!(base64_to_hex("Zm8=").unwrap(), "666f");
        assert_eq!(base64_to_hex("Zm9v").unwrap(), "666f6f");
        assert_eq!(base64_to_hex("Zm9vYg==").unwrap(), "666f6f62");
        assert_eq!(base64_to_hex("Zm9vYmE=").unwrap(), "666f6f6261");
        assert_eq!(base64_to_hex("Zm9vYmFy").unwrap(), "666f6f626172");
        assert_eq!(base64_to_hex("SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t").unwrap(),
            "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d");
    }
}
