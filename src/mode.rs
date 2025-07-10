use std::{fmt, fs};

pub enum Mode {
    Wordlist(Contents),
    BruteForce,
}

type Contents = Vec<String>;

impl Mode {
    pub fn new(path: Option<String>) -> Self {
        match path {
            Some(path) => {
                let contents = fs::read_to_string(&path);
                match contents {
                    Ok(contents) => Mode::Wordlist(string_to_vec(contents)),
                    Err(e) => panic!("Failed to read file {path}.\nError: {e}"),
                }
            }
            None => Self::BruteForce,
        }
    }
}

impl fmt::Debug for Mode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Mode::Wordlist(contents) => {
                write!(f, "Mode::Wordlist(len = {})", contents.len())
            }
            Mode::BruteForce => {
                write!(f, "Mode::BruteForce")
            }
        }
    }
}

fn string_to_vec(string: String) -> Vec<String> {
    string.lines().map(|line| line.to_string()).collect()
}
