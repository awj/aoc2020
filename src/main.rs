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
        let instructions: Vec<day8::Instruction> = contents.lines().map(|l| { println!("{}", l); day8::Instruction::parse(l) }).collect();

        for (i, inst) in instructions.iter().enumerate() {
            let mut machine = day8::Machine::with_flip(&instructions, i as i32);

            match machine.run() {
                day8::ExecutionResult::Terminate => {
                    println!("succeeded by flipping: {}, {:?}", i, inst);
                    println!("acc: {}", machine.acc);
                    break
                },
                _ => ()
            }
        }
    }
}
