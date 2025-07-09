use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// File containing hash to compare to
    /// Supported extensions: *.txt
    #[arg()]
    pub hash_file: String,

    /// Number of threads used for hashing
    /// [default: std::threads::available_parallelism() - 1]
    #[arg(short, long)]
    pub threads: Option<usize>,

    /// File containing wordlist.
    /// Supported extensions: *.txt
    #[arg(short, long)]
    pub wordlist_file: Option<String>,

    /// Algorithm used for hashing. [default: SHA-2 256]
    #[arg(short, long)]
    pub algorithm: Option<String>,

    /// Minium length of cracked String.
    #[arg(long, default_value_t = 4)]
    pub min: usize,

    /// Maximum length of cracked String.
    #[arg(long, default_value_t = 16)]
    pub max: usize,

    /// Does program have to run in verbose mode. [supported: Yes / No]
    #[arg(short, long, default_value_t = String::from("Yes"))]
    pub verbose: String,
}
