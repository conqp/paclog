//! Print the last transaction from the pacman logfile.
use std::fs::OpenOptions;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

use clap::Parser;

use pacmanlog::{Entry, Transaction, Transactions};

const DEFAULT_FILE: &str = "/var/log/pacman.log";

#[derive(Debug, Parser)]
struct Args {
    #[clap(index = 1)]
    index: isize,
    #[clap(long, short, default_value = DEFAULT_FILE)]
    file: String,
}

fn main() {
    let args = Args::parse();

    let transactions: Vec<Transaction> = BufReader::new(
        OpenOptions::new()
            .read(true)
            .open(args.file)
            .expect("Failed to open file"),
    )
    .lines()
    .filter_map(|line| line.ok().and_then(|line| Entry::from_str(&line).ok()))
    .transactions()
    .collect();

    if let Some(transaction) = transactions.get(if args.index < 0 {
        transactions
            .len()
            .checked_add_signed(args.index)
            .unwrap_or_default()
    } else {
        args.index.abs_diff(0)
    }) {
        println!("{transaction:?}");
    }
}
