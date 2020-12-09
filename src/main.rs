use core::str::SplitTerminator;
// use std::convert::TryFrom;

// mod day1;
//mod day3;
// mod day4;
// mod day5;
// mod day6;
mod day7;

use std::env;
use std::fs;

fn each_section<'a>(input: &'a str) -> SplitTerminator<'a, &str> {
    input.split_terminator("\n\n")
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let file = &args[1];

    if let Ok(contents) = fs::read_to_string(file) {
        let bags = contents.lines().map(day7::parse_line).collect();
        let containments = day7::containments(&bags);

        let result = day7::all_containments("shiny gold", &containments);

        println!("{:?}", result.len());
    }
}
