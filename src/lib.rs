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

/// Generates the next string in sequence.
///
/// The first character is kept in the inclusive `first_min..=first_max`
/// range. All following characters use the global `MIN_CHAR..=MAX_CHAR`
/// range. When the current length is exhausted, the next length starts with
/// `first_min` followed by `MIN_CHAR` values.
pub fn next_string(s: &mut Vec<u8>, first_min: u8, first_max: u8) {
    debug_assert!(
        MIN_CHAR <= first_min && first_min <= first_max && first_max <= MAX_CHAR,
        "first-character range must be inside MIN_CHAR..=MAX_CHAR"
    );

    if s.is_empty() {
        s.push(first_min);
        return;
    }

    // Increment the suffix first. The first character has its own range.
    let mut i = s.len();
    while i > 1 {
        i -= 1;
        if s[i] < MAX_CHAR {
            s[i] += 1;
            return;
        }
        s[i] = MIN_CHAR;
    }

    if s[0] < first_max {
        s[0] += 1;
        return;
    }

    // The local first-character range is exhausted at this length.
    s[0] = first_min;
    s.insert(1, MIN_CHAR);
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
        next_string(&mut s, MIN_CHAR, MAX_CHAR);
        assert_eq!(s, b"AB");

        let mut s = b"AZ".to_vec();
        next_string(&mut s, MIN_CHAR, MAX_CHAR);
        assert_eq!(s, b"A[");

        let mut s = b"ZZ".to_vec();
        next_string(&mut s, MIN_CHAR, MAX_CHAR);
        assert_eq!(s, b"Z[");
    }

    #[test]
    fn test_next_string_with_first_character_range() {
        let mut s = b"Lzz".to_vec();
        next_string(&mut s, b'L', b'V');
        assert_eq!(s, b"MAA");

        let mut s = b"Vzz".to_vec();
        next_string(&mut s, b'L', b'V');
        assert_eq!(s, b"LAAA");
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
