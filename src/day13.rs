extern crate num;

use num::Integer;
use num::bigint::BigInt;

pub fn soonest_stop(time: u64, busses: &Vec<u64>) -> (u64, u64) {
    let mut closest = time * 2;
    let mut closest_bus = 0;

    for bus in busses {
        let mut arrival = bus * (time / bus);
        // Unless we've got an exact multiple, add one more for the arrival time
        // *after* the time
        if arrival < time { arrival += bus }

        if arrival < closest {
            closest = arrival;
            closest_bus = *bus;
        }
    }

    (closest_bus, closest - time)
}

pub fn validate_time(time: u64, busses: &Vec<u64>) -> (bool, u64, u64) {
    let mut new_inc = 0;
    let mut num_valid = 1;

    for (i, bus) in busses.iter().enumerate() {
        if *bus != 0 {
            if (time + (i as u64)) % *bus == 0 {
                if new_inc == 0 {
                    new_inc = *bus
                } else {
                    new_inc = new_inc * bus;
                    num_valid += 1
                }
            } else {
                return (false, num_valid, new_inc);
            }
        }
    }

    (true, num_valid, new_inc)
}

pub fn seek_time(busses: &Vec<u64>) -> u64 {
    let init = busses[0];

    let mut inc = busses[0];

    let mut t = init;
    let mut i = 0;

    println!("running with increment: {}", inc);

    loop {
        t += inc;

        let (valid, num_valid, new_inc) = validate_time(t, busses);
        println!("checked: {}, new increment: {}", t, new_inc);

        if valid {
            println!("t: {}", t);
            break;
        } else {
            inc = new_inc
        }

        i += 1;
        if i % 10_000_000 == 0 { println!("{}, {}", i, t) }
    }

    t

    // let mut inc = busses[0];
    // let mut t = busses[0];
    // let mut i: u64 = 0;

    // println!("final increment: {}", inc);

    // t = inc * (100000000000000 / inc);

    // loop {
    //     let result = validate_time(t, busses);

    //     if result {
    //         println!("t: {}", t);
    //         break;
    //     } else {
    //         t += inc;
    //     }

    //     i += 1;

    //     if i % 10_000_000 == 0 { println!("{}, {}", i, t) }
    // };

    // t
}

pub fn parse(input: &str) -> (u64, Vec<u64>) {
    let mut lines = input.lines();

    let mut options = Vec::new();
    let starget = lines.next().unwrap();
    println!("starget: {:?}", starget);
    let target = starget.parse::<u64>().unwrap();
    let busses = lines.next();

    println!("busses: {:?}", busses);

    for val in busses.unwrap().split(",") {
        match val {
            "x" => options.push(0),
            _ => options.push(val.parse::<u64>().unwrap())
        }
    };

    (target, options)
}
