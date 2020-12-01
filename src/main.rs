mod day1;

use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file = &args[1];

    if let Ok(contents) = fs::read_to_string(file) {
        // Have to .collect() here to get a vector out of the Map.
        let input:Vec<i32> = contents.lines().map(|x| x.parse::<i32>().unwrap()).collect();
        match day1::find_triple_with_total(&input) {
            Some((x,y, z)) => println!("Result {}", x * y * z),
            None => println!("No pairs")
        }
    }

}
