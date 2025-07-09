use std::fs;

#[derive(Debug)]
pub enum Mode {
    Wordlist(Contents),
    BruteForce,
}

type Contents = String;

impl Mode {
    pub fn new(path: Option<String>) -> Self {
        match path {
            Some(path) => {
                let contents = fs::read_to_string(path);
                if let Ok(contents) = contents {
                    return Self::Wordlist(contents);
                } else {
                    panic!("Failed to read file");
                }
            }
            None => Self::BruteForce,
        }
    }
}
