use std::fs::File;
use std::io::{BufRead, BufReader};
use utf8_chars::BufReadCharsExt;

pub fn calculate_bytes(path: &str) -> Result<u64, String> {
    match std::fs::metadata(path) {
        Ok(info) => Ok(info.len()),
        Err(err) => Err(err.to_string()),
    }
}

pub fn calculate_lines(path: &str) -> Result<u64, String> {
    match File::open(path) {
        Ok(file) => {
            let reader = BufReader::new(file);
            Ok(reader.lines().count() as u64)
        }
        Err(err) => Err(err.to_string()),
    }
}

pub fn calculate_words(path: &str) -> Result<u64, String> {
    match File::open(path) {
        Ok(file) => {
            let reader = BufReader::new(file);
            let mut words: u64 = 0;
            for line in reader.lines() {
                if let Ok(line) = line {
                    words += line.split_whitespace().count() as u64;
                }
            }
            Ok(words)
        }
        Err(err) => Err(err.to_string()),
    }
}

pub fn calculate_chars(path: &str) -> Result<u64, String> {
    match File::open(path) {
        Ok(file) => {
            let mut reader = BufReader::new(file);
            let mut chars: u64 = 0;
            for c in reader.chars_raw() {
                if let Ok(c) = c {
                    if c != '\0' {
                        chars += 1
                    }
                }
            }
            Ok(chars)
        }
        Err(err) => Err(err.to_string()),
    }
}
