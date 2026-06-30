use std::{fs, process};
pub mod cli;
pub mod hasher;
pub mod mode;

/// If Result is Ok returns value, else handles error and returns default value.
/// Removes `\n` sign if found
/// ### Panic
/// When file doesn't exist.
pub fn get_hash_file_contents(path: String) -> String {
    match fs::read_to_string(&path) {
        Ok(v) => v.replace("\n", ""),
        Err(e) => {
            eprintln!("{e:?}");
            println!("Failed to fetch {path} contents");
            process::exit(1);
        }
    }
}

/// Generates new string based on pervious value in sequence
pub fn next_string(s: &mut Vec<u8>) {
    let mut i = s.len();
    while i > 0 {
        i -= 1;
        if s[i] < b'z' {
            s[i] += 1;
            return;
        } else {
            s[i] = b'A';
        }
    }
    // All characters were 'z', need to add new 'A' at the front
    s.insert(0, b'A');
}

/// Parses Yes / No CLI answers into bool
pub fn parse_string_to_bool(input: String) -> bool {
    let input = &input.to_lowercase()[..];
    !matches!(input, "no" | "n" | "false")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_string_to_bool_no() {
        let str = String::from("NO");
        assert!(!parse_string_to_bool(str));
        let str = String::from("nO");
        assert!(!parse_string_to_bool(str));
        let str = String::from("No");
        assert!(!parse_string_to_bool(str));
        let str = String::from("false");
        assert!(!parse_string_to_bool(str));
    }

    #[test]
    fn test_parse_string_to_bool_yes() {
        let str = String::from("");
        assert!(parse_string_to_bool(str));
        let str = String::from("y");
        assert!(parse_string_to_bool(str));
        let str = String::from("Yes");
        assert!(parse_string_to_bool(str));
        let str = String::from("YES");
        assert!(parse_string_to_bool(str));
        let str = String::from("true");
        assert!(parse_string_to_bool(str));
    }
}
