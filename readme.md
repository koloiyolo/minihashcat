# minihashcat
Simple `hashcat` clone.

## About
CLI Rust program that imitates `hashcat` that was written as part of mu Rust learning journey.\
It is not a part of any guide or anything like that, it is a personal goal.

### Examples
First You need to compile this program:
```bash
cargo build --release
```

If You got the binary You can run this for `brute force` mode
```bash
./minihashcat example.txt
```

If You'd like to use a `wordlist` run:
```bash
./minihashcat example.txt -w your_wordlist_file.txt
```

Example file contains greeting in polish 😉

If You want to write your result to file You can do:
```bash
./minihashcat example.txt -v no > result.txt
```

For list of optional arguments and defaults use:
```bash
./minihashcat --help
```

### Current state
For now it supports:
* [x] Multithreading
* [x] BruteForce
* [x] Wordlist files

Algorithms:
* [x] MD2
* [x] MD4
* [ ] MD5
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
* Interface building with `trait`
* Anonymous functions

The following libraries were used:
* `clap` for CLI interface
* `crossbeam::channel` as alternative to `std::sync::mpsc` letting me to limit number of accepted messages
* hash algorithms from [RustCrypto/hashes](https://github.com/RustCrypto/hashes) collection.
