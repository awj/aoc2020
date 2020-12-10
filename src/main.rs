use core::str::SplitTerminator;
// use std::convert::TryFrom;

// mod day1;
//mod day3;
// mod day4;
// mod day5;
// mod day6;
// mod day7;
// mod day8;
mod day9;

use std::env;
use std::fs;

fn each_section<'a>(input: &'a str) -> SplitTerminator<'a, &str> {
    input.split_terminator("\n\n")
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let file = &args[1];

    if let Ok(contents) = fs::read_to_string(file) {
        let nums = contents.lines().map(|l| l.parse::<i64>().unwrap()).collect();

        let size = &args[2].parse::<usize>().unwrap();

        match day9::first_non_sum(&nums, *size) {
            Some(num) => {
                println!("found weakness: {}", num);
                match day9::numbers_adding_to(&nums, num) {
                    Some((start,end)) => println!("found nums: {:?}, {:?}, sum: {:?}", start, end, start + end),
                    None => println!("no dice!")
                }
            }
            None => println!("none found!")
        }
    }
}
