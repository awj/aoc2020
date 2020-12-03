// mod day1;
mod day3;

use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file = &args[1];

    if let Ok(contents) = fs::read_to_string(file) {
        let forest = day3::Forest::parse(contents.lines());

        println!("trees: {}", forest.trees_along((3, 1)));
    }
}
