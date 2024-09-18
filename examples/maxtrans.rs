//! Find the largest transaction by amount of affected packages in the pacman logfile.
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
    env_logger::init();
    let args = Args::parse();
    let transaction = BufReader::new(
        OpenOptions::new()
            .read(true)
            .open(args.file)
            .expect("Failed to open file"),
    )
    .lines()
    .filter_map(|line| line.ok().and_then(|line| Entry::from_str(&line).ok()))
    .transactions()
    .max_by(|a, b| a.len().cmp(&b.len()))
    .expect("No transactions found");
    println!("{transaction:?}");
}
