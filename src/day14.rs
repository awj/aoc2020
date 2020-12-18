use std::collections::HashMap;
use core::num::ParseIntError;

#[derive(Debug)]
pub struct Mem {
    set_bits: u64,
    unset_bits: u64,
    memory: HashMap<usize, u64>
}

impl Mem {
    pub fn new() -> Mem {
        Mem {
            set_bits: 0,
            unset_bits: 0,
            memory: HashMap::new()
        }
    }

    pub fn set_mask(&mut self, input: &str) {
        let mut set_bits = 0;
        let mut unset_bits = 0;

        for (i, char) in input.chars().rev().enumerate() {
            match char {
                '0' => { unset_bits |= 2_u64.pow(i as u32) },
                '1' => { set_bits |= 2_u64.pow(i as u32) },
                _ => ()
            }
        }

        self.set_bits = set_bits;
        self.unset_bits = unset_bits;
    }

    pub fn set(&mut self, location: usize, value: u64) -> u64 {
        let mut current = value.clone();

        current = current | self.set_bits;
        current = current & !self.unset_bits;

        println!("setting: {}, {} (from {})", location, current, value);

        self.memory.insert(location, current.clone());

        current
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
