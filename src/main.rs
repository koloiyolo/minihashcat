use clap::Parser;
use crossbeam::channel;
use minihashcat::cli::Cli;
use minihashcat::hasher::*;
use minihashcat::mode::Mode;
use minihashcat::{get_hash_file_contents, parse_string_to_bool};
use std::sync::Arc;
use std::{num::NonZeroUsize, thread};

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
    let algorithm = algorithm.unwrap_or("".to_string());
    let mode = Mode::new(wordlist_file);
    let hasher = Arc::new(HasherHandler::from(algorithm.as_str()));
    let thread_count = threads.unwrap_or(
        thread::available_parallelism()
            .unwrap_or(NonZeroUsize::new(1).expect("const context"))
            .get()
            / 2,
    );

    if verbose {
        println!("\nHash to crack: {hash}");
        println!("Hashing algorithm: {}", hasher.name());
        println!("Mode: {mode:?}");
        println!("Threads: {thread_count}");
        println!("Running...\n");
    }

    let success_count = 1;

    let (stop_sender, stop_receiver) = channel::bounded(success_count);
    for thread_id in 0..thread_count {
        let function: Box<dyn FnOnce() + Send> = match mode {
            Mode::Wordlist(ref wordlist_contents) => Mode::word_list_crack_hash_fn(
                wordlist_contents.clone(),
                hasher.clone(),
                hash.clone(),
                thread_id,
                thread_count,
                stop_sender.clone(),
            ),
            Mode::BruteForce => Mode::brute_force_crack_hash_fn(
                hasher.clone(),
                hash.clone(),
                min,
                max,
                thread_id as u8,
                thread_count,
                stop_sender.clone(),
            ),
        };

        thread::spawn(function);
    }

    match stop_receiver.recv() {
        Ok(result) => {
            if verbose {
                println!("Result:")
            }
            match String::from_utf8(result) {
                Ok(value) => println!("{value}"),
                Err(e) => eprintln!("Parsing Error {e}"),
            }
        }
        Err(e) => {
            eprintln!("No thread succeeded. {e}");
        }
    }
}
