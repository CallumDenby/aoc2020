#![feature(variant_count)]

use std::fs;

mod solutions;
use solutions::{day1, day2};

fn main() {
    let day = std::env::args()
        .nth(1)
        .expect("no day given")
        .parse()
        .expect("day was not a number");

    let input = fs::read_to_string(format!("./data/day{}.txt", day))
        .expect(&format!("missing input data for day ./data/day{}.txt", day));

    match day {
        1 => day1::solve(input),
        2 => day2::solve(input),
        _ => eprintln!("Not implemented yet, don't be impatient!"),
    }
}
