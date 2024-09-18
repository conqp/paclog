//! Print the last transaction from the pacman logfile.
use std::fs::OpenOptions;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

use clap::Parser;

use pacmanlog::{Entry, Transactions};

const DEFAULT_FILE: &str = "/var/log/pacman.log";

#[derive(Debug, Parser)]
struct Args {
    #[clap(index = 1, default_value = DEFAULT_FILE)]
    file: String,
}

fn main() {
    let args = Args::parse();

    for transaction in BufReader::new(
        OpenOptions::new()
            .read(true)
            .open(args.file)
            .expect("Failed to open file"),
    )
    .lines()
    .filter_map(|line| line.ok().and_then(|line| Entry::from_str(&line).ok()))
    .transactions()
    {
        println!("{transaction:?}");
    }
}
