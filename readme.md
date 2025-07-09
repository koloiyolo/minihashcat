# minihashcat
Simple `hashcat` clone.

## About
CLI Rust program that imitates `hashcat` that was written as part of mu Rust learning journey.\
It is not a part of any guide or anything like that, it is a personal goal.

### Example
To run this program simply use:

before compiling
```bash
cargo run -- example.txt
```

after compiling
```bash
./minihashcat example.txt
```

Example file contains greeting in polish 😉

If You want to write to file You can do:
```bash
./minihashcat example.txt > result.txt
```
For list of optional arguments and defaults use:
```bash
./minihashcat --help
```

### Current state
For now it supports:
* [x] Multithreading
* [x] BruteForce
* [ ] Wordlist files

Algorithms:
* [x] MD2
* [x] MD4
* [x] SHA-2 256
* [x] SHA-2 384
* [x] SHA-2 512
* [x] SHA-3 256
* [x] SHA-3 384
* [x] SHA-3 512

### What I learned
This program helped me learn how to handle:
* Multithreading
* Error Handling
* Pattern matching
* Building and using `trait` in Rust

The following libraries were used:
* `clap` for CLI interface
* `crossbeam::channel` as alternative to `std::sync::mpsc` letting me to limit number of accepted messages
* hash algorithms from [RustCrypto/hashes](https://github.com/RustCrypto/hashes) collection.
