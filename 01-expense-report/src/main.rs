use std::{
    env, fs,
    io::{prelude::*, BufReader},
};

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = parse_args(&args);
    let mut expenses = expenses_from_file(path);
    expenses.sort_by(|a, b| a.cmp(b));
    let (a, b) =
        expenses_with_sum(2020, &expenses).expect("Couldn't find expenses that summed to 2020");
    let total = a * b;

    println!("{}", total);
}

fn parse_args(args: &[String]) -> &str {
    if args.len() < 2 {
        panic!("Not enough arguments");
    }
    &args[1]
}

fn expenses_from_file(path: &str) -> Vec<i32> {
    let file = fs::File::open(path).expect("Unable to read file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| {
            l.expect("Could not parse line")
                .parse::<i32>()
                .expect("Malformed input")
        })
        .collect()
}

fn expenses_with_sum(expected_sum: i32, expenses: &[i32]) -> Option<(i32, i32)> {
    let mut start_index: usize = 0;
    let mut end_index: usize = expenses.len() - 1;

    while start_index != end_index {
        let sum = expenses[start_index] + expenses[end_index];

        if sum == expected_sum {
            return Some((expenses[start_index], expenses[end_index]));
        } else if sum < expected_sum {
            start_index += 1;
        } else {
            end_index -= 1;
        }
    }

    None
}
