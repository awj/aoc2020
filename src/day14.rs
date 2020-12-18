use std::collections::{HashMap, HashSet};
use core::num::ParseIntError;
use std::convert::TryFrom;

#[derive(Debug)]
pub struct Mem {
    set_bits: usize,
    floating_bits: HashSet<usize>,
    memory: HashMap<usize, u64>
}

impl Mem {
    pub fn new() -> Mem {
        Mem {
            set_bits: 0,
            floating_bits: HashSet::new(),
            memory: HashMap::new()
        }
    }

    pub fn set_mask(&mut self, input: &str) {
        let mut set_bits = 0;
        self.floating_bits.clear();

        for (i, char) in input.chars().rev().enumerate() {
            match char {
                '0' => (),
                '1' => { set_bits |= 2_usize.pow(i as u32); },
                'X' => { self.floating_bits.insert(2_usize.pow(i as u32)); }
                _ => ()
            }
        }

        self.set_bits = set_bits;
    }

    pub fn set(&mut self, location: usize, value: u64) {
        let mut locations: Vec<usize> = vec![location | self.set_bits];

        println!("floating bits: {:?}", self.floating_bits);
        println!("initial location: {:?}", locations[0]);
        for bit in self.floating_bits.iter() {
            println!("bit mask: {}", bit);
            // Get *current* size of the vector
            let size = locations.len();
            println!("locations {:?}", locations);

            for i in 0..size {
                let loc = locations[i];

                // Update the location with the variation that includes the zero bit
                locations[i] = loc & !bit;
                // push the variation that includes the one bit
                locations.push(loc | bit);
            }
        }


        println!("writing to locations {:?}", locations);
        for loc in locations {
            self.memory.insert(loc, value);
        }
    }

    pub fn sum(&self) -> u64 {
        self.memory.values().sum()
    }
}

pub fn execute(instructions: &str) -> Result<u64,ParseIntError> {
    let mut mem = Mem::new();

    for line in instructions.lines() {
        let parts:Vec<&str> = line.split(" = ").collect();

        if parts[0] == "mask" {
            mem.set_mask(&parts[1]);
        } else {
            let location_s: String = parts[0].chars().filter(|&c| "0123456789".contains(c)).collect::<String>();
            let location = location_s.parse::<usize>()?;
            let value = parts[1].parse::<u64>()?;
            mem.set(location, value);
        }
    }

    Ok(mem.sum())
}
