// mod day1;
mod day2;
use day2::PWLine;

use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file = &args[1];

    if let Ok(contents) = fs::read_to_string(file) {
        let input = contents.lines().map(|x| PWLine::parse(x).unwrap() );

        let num_valid = input.filter(|p| p.valid()).count();

        println!("num_valid: {}", num_valid);
    }
}
