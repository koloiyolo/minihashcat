use std::fs;

use crate::errors::MiniHashcatError;
pub mod cli;
pub mod errors;
pub mod hasher;
pub mod mode;

pub const MIN_CHAR: u8 = b'A';
pub const MAX_CHAR: u8 = b'z';

/// If Result is Ok returns value, else handles error and returns default value.
/// Removes `\n` sign if found
///
/// ## Panics
///
/// When the file doesn't exist.
pub fn get_hash_file_contents(path: &str) -> Result<String, MiniHashcatError> {
    match fs::read_to_string(path) {
        Ok(v) => Ok(v.replace("\n", "")),
        Err(_) => Err(MiniHashcatError::fine_not_found(path.to_string())),
    }
}

/// Generates new string based on pervious value in sequence
pub fn next_string(s: &mut Vec<u8>) {
    let mut i = s.len();
    while i > 0 {
        i -= 1;
        if s[i] < MAX_CHAR {
            s[i] += 1;
            return;
        } else {
            s[i] = MIN_CHAR;
        }
    }
    // All characters were 'z', need to add new 'A' at the front
    s.insert(0, MIN_CHAR);
}

/// Parses Yes / No CLI answers into bool
pub fn parse_string_to_bool(input: &str) -> bool {
    let input = &input.to_lowercase()[..];
    !matches!(input, "no" | "n" | "false")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_string_to_bool_no() {
        let str = "NO";
        assert!(!parse_string_to_bool(str));
        let str = "nO";
        assert!(!parse_string_to_bool(str));
        let str = "No";
        assert!(!parse_string_to_bool(str));
        let str = "false";
        assert!(!parse_string_to_bool(str));
    }

    #[test]
    fn test_parse_string_to_bool_yes() {
        let str = "";
        assert!(parse_string_to_bool(str));
        let str = "y";
        assert!(parse_string_to_bool(str));
        let str = "Yes";
        assert!(parse_string_to_bool(str));
        let str = "YES";
        assert!(parse_string_to_bool(str));
        let str = "true";
        assert!(parse_string_to_bool(str));
    }

    #[test]
    fn test_next_string() {
        let mut s = b"AA".to_vec();
        next_string(&mut s);
        assert_eq!(s, b"AB");

        let mut s = b"AZ".to_vec();
        next_string(&mut s);
        assert_eq!(s, b"A[");

        let mut s = b"ZZ".to_vec();
        next_string(&mut s);
        assert_eq!(s, b"Z[");
    }

    #[test]
    fn test_get_hash_file_contents() {
        let file_name = "example.txt";

        let output = get_hash_file_contents(file_name).expect("checked value");

        assert_eq!(
            "32cdb619196200050ab0af581a10fb83cfc63b1a20f58d4bafb6313d55a3f0e9",
            &output
        );

        let file_name = "invalid_example.txt";
        let output = get_hash_file_contents(file_name);
        assert!(output.is_err());
    }
}
