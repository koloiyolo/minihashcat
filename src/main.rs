use clap::Parser;
use crossbeam::channel;
use minihashcat::cli::Cli;
use minihashcat::hasher::*;
use minihashcat::mode::Mode;
use minihashcat::{
    get_hash_file_contents, get_option_value, get_result_value, next_string, parse_string_to_bool,
};
use std::{num::NonZeroUsize, thread, time::Duration};

fn main() {
    let args = Cli::parse();
    // Extract args
    let hash_file = args.hash_file;
    let algorithm = args.algorithm;
    let wordlist_file = args.wordlist_file;
    let threads = args.threads;
    let min = args.min;
    let max = args.max + 1;
    let verbose = parse_string_to_bool(args.verbose);

    let hash = get_hash_file_contents(hash_file);
    let algorithm = get_option_value(algorithm, "".to_string());
    let mode = Mode::new(wordlist_file);
    let hasher = create_hasher(&algorithm);
    let thread_count = get_option_value(
        threads,
        get_result_value(
            thread::available_parallelism(),
            NonZeroUsize::new(1).unwrap(),
        )
        .get()
            - 1,
    );

    if verbose {
        println!("\nHash to crack: {hash}");
        println!("Hashing algorithm: {}", &hasher.name());
        println!("Mode: {mode:?}");
        println!("Threads: {thread_count}");
        println!("Running...\n");
    }

    let min_char = b'A';
    let max_char = b'z';
    let slice_len = max_char - min_char;
    let thread_len = slice_len / thread_count as u8;
    let success_count = 1;

    let (sender, receiver) = channel::bounded(success_count);
    for i in 0..thread_count {
        let function: Box<dyn FnOnce() + Send> = match mode {
            Mode::Wordlist(ref wordlist) => {
                let sender = sender.clone();
                let algorithm = algorithm.clone();
                let hash = hash.clone();
                let wordlist = wordlist.clone();
                let slice_len = wordlist.len() / thread_count;

                let wordlist_fn = move || {
                    let hasher = create_hasher(&algorithm);
                    let index = i * slice_len;
                    let end = if i == thread_count - 1 {
                        wordlist.len()
                    } else {
                        index + slice_len
                    };

                    let wordlist_slice = &wordlist[index..end];

                    for word in wordlist_slice {
                        if hasher.compare_hash(&word.as_bytes(), &hash) {
                            let _ = sender.send(word.as_bytes().to_vec());
                            return;
                        }
                    }
                    // Let's other threads to finish
                    let sleep_duration = Duration::from_secs(5);
                    thread::sleep(sleep_duration);

                    let not_found_error = "Not Found".as_bytes().to_vec();
                    let _ = sender.send(not_found_error);
                };

                Box::new(wordlist_fn)
            }
            Mode::BruteForce => {
                let sender = sender.clone();
                let algorithm = algorithm.clone();
                let hash = hash.clone();
                let min_length = min;
                let max_length = max;

                let brute_force_fn = move || {
                    let hasher = create_hasher(&algorithm);
                    let min_char = min_char + (i as u8 * thread_len);
                    let mut compared: Vec<u8> = vec![min_char; min_length];

                    while compared.len() < max_length {
                        if hasher.compare_hash(&compared, &hash) {
                            let _ = sender.send(compared);
                            break;
                        }
                        next_string(&mut compared);
                    }
                    let not_found_error = "Not Found".as_bytes().to_vec();
                    let _ = sender.send(not_found_error);
                };
                Box::new(brute_force_fn)
            }
        };

        thread::spawn(function);
    }

    match receiver.recv() {
        Ok(result) => {
            if verbose {
                println!("Result:")
            }
            println!(
                "{}",
                String::from_utf8(result).unwrap_or_else(|e| (format!("\nParsing Error {e}")))
            );
        }
        Err(_) => {
            println!("No thread succeeded.");
        }
    }
}
