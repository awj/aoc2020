mod day1;

use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file = &args[1];

    if let Ok(contents) = fs::read_to_string(file) {
        match day1::find_pair_with_total(contents.lines().map(|x| x.parse::<i32>().unwrap())) {
            Some((x,y)) => println!("Result {}", x * y),
            None => println!("No pairs")
        }
    }

}
