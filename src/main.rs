// use std::convert::TryFrom;

// mod day1;
//mod day3;
// mod day4;
// mod day5;
mod day6;

use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file = &args[1];

    if let Ok(contents) = fs::read_to_string(file) {
        let sizes: Vec<usize> = day6::all_yes(&mut contents.lines());
        let sum: usize = sizes.iter().sum();

        println!("sum: {}", sum)
    }
}
