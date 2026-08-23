use std::{fs, sync::Arc, thread, time::Duration};

use crossbeam::channel::Sender;

use crate::{
    MAX_CHAR, MIN_CHAR,
    errors::MiniHashcatError,
    hasher::{Hasher, HasherHandler},
    next_string,
};

const CHARSET_LEN: usize = (MAX_CHAR - MIN_CHAR + 1) as usize;

pub enum Mode {
    Wordlist(Contents),
    BruteForce,
}

type Contents = Arc<Vec<String>>;

impl Mode {
    pub fn brute_force_crack_hash_fn(
        hasher: Arc<HasherHandler>,
        hash: Arc<Vec<u8>>,
        min_length: usize,
        max_length: usize,
        thread_id: usize,
        thread_count: usize,
        stop_sender: Sender<Vec<u8>>,
    ) -> Box<dyn FnOnce() + Send> {
        let first_start = thread_id * CHARSET_LEN / thread_count;
        let first_end = (thread_id + 1) * CHARSET_LEN / thread_count;
        let first_min = MIN_CHAR + first_start as u8;
        let first_max = MIN_CHAR + (first_end - 1) as u8;

        let brute_force_fn = move || {
            let mut compared = vec![MIN_CHAR; min_length];
            if let Some(first) = compared.first_mut() {
                *first = first_min;
            }

            while compared.len() < max_length {
                if hasher.compare_hash(&compared, hash.as_slice()) {
                    let _ = stop_sender.send(compared);
                    break;
                }
                next_string(&mut compared, first_min, first_max);
            }
            let not_found_error = "Not Found".as_bytes().to_vec();
            let _ = stop_sender.send(not_found_error);
        };
        Box::new(brute_force_fn)
    }

    pub fn word_list_crack_hash_fn(
        wordlist_contents: Contents,
        hasher: Arc<HasherHandler>,
        hash: Arc<Vec<u8>>,
        thread_id: usize,
        thread_count: usize,
        stop_sender: Sender<Vec<u8>>,
    ) -> Box<dyn FnOnce() + Send> {
        let wordlist_fn = move || {
            let index = thread_id * CHARSET_LEN;
            let end = if thread_id == thread_count - 1 {
                wordlist_contents.len()
            } else {
                index + CHARSET_LEN
            };

            let wordlist_slice = &wordlist_contents[index..end];

            for word in wordlist_slice {
                if hasher.compare_hash(word.as_bytes(), hash.as_slice()) {
                    let _ = stop_sender.send(word.as_bytes().to_vec());
                    return;
                }
            }
            // Let's other threads to finish
            let sleep_duration = Duration::from_secs(5);
            thread::sleep(sleep_duration);

            let not_found_error = "Not Found".as_bytes().to_vec();
            let _ = stop_sender.send(not_found_error);
        };

        Box::new(wordlist_fn)
    }
}

impl TryFrom<Option<String>> for Mode {
    type Error = MiniHashcatError;

    fn try_from(value: Option<String>) -> Result<Self, Self::Error> {
        match value {
            Some(path) => {
                let contents = fs::read_to_string(&path)
                    .map_err(|_| MiniHashcatError::fine_not_found(path))?;
                Ok(Mode::Wordlist(string_to_vec(contents)))
            }
            None => Ok(Self::BruteForce),
        }
    }
}

#[cfg(not(tarpaulin_include))]
impl std::fmt::Debug for Mode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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

/// Parses provided [String] value and parses it into [Mode::Wordlist] contents.
fn string_to_vec(string: String) -> Contents {
    Arc::new(string.lines().map(|line| line.to_string()).collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mode_constructor() {
        let value = None;
        assert!(matches!(Mode::try_from(value), Ok(Mode::BruteForce)));

        let value = Some("example.txt".to_string());
        assert!(matches!(Mode::try_from(value), Ok(Mode::Wordlist(_))));

        let value = Some("invalid_example.txt".to_string());
        assert!(Mode::try_from(value).is_err());
    }
}
