use core::str::SplitTerminator;
// use std::convert::TryFrom;

// mod day1;
//mod day3;
// mod day4;
// mod day5;
// mod day6;
// mod day7;
mod day8;

use std::env;
use std::fs;

fn each_section<'a>(input: &'a str) -> SplitTerminator<'a, &str> {
    input.split_terminator("\n\n")
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let file = &args[1];

    if let Ok(contents) = fs::read_to_string(file) {
        let instructions = contents.lines().map(|l| { println!("{}", l); day8::Instruction::parse(l) }).collect();

        let mut machine = day8::Machine::new(&instructions);

        machine.run();

        println!("acc: {}", machine.acc);
    }
}
