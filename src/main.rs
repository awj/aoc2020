// use std::convert::TryFrom;

// mod day1;
//mod day3;
// mod day4;
mod day5;

use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file = &args[1];

    if let Ok(contents) = fs::read_to_string(file) {
        let mut highest = 0;
        let mut seat_ids: Vec<u32> = contents.lines().map(|l| {
            day5::Seat::parse(l).seat_id()
        }).collect();

        seat_ids.sort();

        let mut last = seat_ids[1];

        for (i, id) in seat_ids.iter().enumerate() {
            if i > 0 && i < seat_ids.len() - 1 {
                if id - last > 1 {
                    println!("your seat: {}, id: {}, prev: {}", last + 1, id, last);
                }

                last = *id;
            }
        }
    }
}
