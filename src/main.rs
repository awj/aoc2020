use core::str::SplitTerminator;
// use std::convert::TryFrom;

// mod day1;
//mod day3;
// mod day4;
// mod day5;
// mod day6;
// mod day7;
// mod day8;
// mod day9;
mod day10;

use std::env;
use std::fs;

fn each_section<'a>(input: &'a str) -> SplitTerminator<'a, &str> {
    input.split_terminator("\n\n")
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let file = &args[1];

    if let Ok(contents) = fs::read_to_string(file) {
        let mut joltages: Vec<i32> = day10::optimal_config(&contents);

        println!("checking: {:?}", joltages);

        println!("configs: {:?}", day10::possible_configs(&joltages));
    }
}
