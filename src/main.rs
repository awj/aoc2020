use std::convert::TryFrom;

// mod day1;
mod day3;

use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file = &args[1];

    if let Ok(contents) = fs::read_to_string(file) {
        let slopes = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
        let forest = day3::Forest::parse(contents.lines());

        let total_trees: i64 = slopes.iter().map(|slope| {
            let encounters = forest.trees_along(*slope);
            // Have to up-convert to i64 since multiplying tree counts causes
            // integer overflow for i32.
            i64::try_from(encounters).unwrap()
        }).product();

        println!("trees: {}", total_trees);
    }
}
