#[macro_use]
extern crate lazy_static;

mod password;
use std::{
    env, fs,
    io::{prelude::*, BufReader},
};

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = parse_args(&args);
    let password_entries = password_entries_from_file(&path);
    let valid_count = password_entries
        .iter()
        .filter(|entry| entry.is_valid())
        .count();
    println!("{}", valid_count);
}

fn parse_args(args: &[String]) -> &str {
    if args.len() < 2 {
        panic!("Not enough arguments");
    }
    &args[1]
}

fn password_entries_from_file(path: &str) -> Vec<password::PasswordEntry> {
    let file = fs::File::open(path).expect("Unable to read file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| {
            l.expect("Could not parse line")
                .parse::<password::PasswordEntry>()
                .expect("Malformed input")
        })
        .collect()
}
