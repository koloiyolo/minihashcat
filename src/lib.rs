use std::{fmt::Debug, fs, process};
pub mod cli;
pub mod hasher;
pub mod mode;

/// If Option is not None returns value, else returns default value.
pub fn get_option_value<T>(option: Option<T>, default: T) -> T {
    match option {
        Some(v) => v,
        None => default,
    }
}

/// If Result is Ok returns value, else handles error and returns default value.
pub fn get_result_value<T, E: Debug>(result: Result<T, E>, default: T) -> T {
    match result {
        Ok(v) => v,
        Err(e) => {
            eprintln!("{e:?}");
            default
        }
    }
}

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
        assert_eq!(parse_string_to_bool(str), false);
        let str = String::from("nO");
        assert_eq!(parse_string_to_bool(str), false);
        let str = String::from("No");
        assert_eq!(parse_string_to_bool(str), false);
        let str = String::from("false");
        assert_eq!(parse_string_to_bool(str), false);
    }

    #[test]
    fn test_parse_string_to_bool_yes() {
        let str = String::from("");
        assert_eq!(parse_string_to_bool(str), true);
        let str = String::from("y");
        assert_eq!(parse_string_to_bool(str), true);
        let str = String::from("Yes");
        assert_eq!(parse_string_to_bool(str), true);
        let str = String::from("YES");
        assert_eq!(parse_string_to_bool(str), true);
        let str = String::from("true");
        assert_eq!(parse_string_to_bool(str), true);
    }
    #[test]
    fn test_get_option_value() {
        assert_eq!(get_option_value(Some(3), 1), 3);
        assert_eq!(get_option_value(Some("test1"), "test2"), "test1");
        assert_eq!(get_option_value(None, 1), 1);
        assert_eq!(
            get_option_value(Some(String::new()), String::from("Test")),
            String::new()
        );
        assert_eq!(
            get_option_value(Some(vec![1, 2, 3, 4]), Vec::new()),
            vec![1, 2, 3, 4]
        );
        assert_eq!(get_option_value(None, vec![1, 2, 3, 4]), vec![1, 2, 3, 4]);
    }
}
