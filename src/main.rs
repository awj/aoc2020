// use std::convert::TryFrom;

// mod day1;
//mod day3;
mod day4;

use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file = &args[1];

    if let Ok(contents) = fs::read_to_string(file) {
        println!("valid passports: {}", day4::valid_passports(contents.lines().collect()))
    }
}
