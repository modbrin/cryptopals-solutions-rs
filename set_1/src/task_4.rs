/// Task: Find single-byte XOR
use crate::prelude::brute_single_byte_xor;
use std::cmp::Ordering;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename).unwrap();
    Ok(io::BufReader::new(file).lines())
}

pub fn find_single_byte_xor<P>(path: P) -> Result<(u8, String, f32), String>
where
    P: AsRef<Path>,
{
    let lines = read_lines(path).map_err(|_| "Failed to read file".to_owned())?;
    lines
        .filter_map(|lr| lr.ok())
        .filter_map(|line| brute_single_byte_xor(&line).ok())
        .max_by(|a, b| a.2.partial_cmp(&b.2).unwrap_or(Ordering::Equal))
        .ok_or_else(|| "Can't find best candidate".to_owned())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_single_byte_xor_should_pass() {
        assert_eq!(
            find_single_byte_xor("res/task4.txt"),
            Ok((53u8, "Now that the party is jumping\n".to_owned(), 2.123412))
        );
    }
}
