use std::convert::TryInto;
use std::str::FromStr;
use std::num::ParseIntError;

use std::ops::{AddAssign, Mul};

#[derive(Debug)]
enum Instruction {
    North(u32),
    South(u32),
    East(u32),
    West(u32),
    Left(u32),
    Right(u32),
    Forward(u32)
}

impl FromStr for Instruction {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self,Self::Err> {
        Ok(match s.chars().nth(0).unwrap() {
            'N' => Instruction::North(s[1..].parse::<u32>()?),
            'S' => Instruction::South(s[1..].parse::<u32>()?),
            'E' => Instruction::East(s[1..].parse::<u32>()?),
            'W' => Instruction::West(s[1..].parse::<u32>()?),
            'L' => Instruction::Left(s[1..].parse::<u32>()?),
            'R' => Instruction::Right(s[1..].parse::<u32>()?),
            'F' => Instruction::Forward(s[1..].parse::<u32>()?),
            _ => panic!("unrecongizable instruction: {}", s)
        })
    }
}

#[derive(Debug)]
enum Turn { Right, Left }

#[derive(Debug)]
enum Direction {
    North,
    South,
    East,
    West
}

#[derive(Clone, Copy, Debug)]
pub struct Location {
    x: i32,
    y: i32
}

impl Mul<u32> for Location {
    type Output = Self;

    fn mul(self, rhs: u32) -> Self::Output {
        Self {
            x: self.x * (rhs as i32),
            y: self.y * (rhs as i32)
        }
    }
}

impl AddAssign<Location> for Location {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y
        };
    }
}

impl Location {
    pub fn origin() -> Location {
        Location { x: 0, y: 0 }
    }
    pub fn manhattan(&self) -> u32 {
        (self.x.abs() + self.y.abs()).try_into().unwrap()
    }

    // Return a location that is this location rotated about the origin. (only
    // supports 90 degree increments)
    pub fn rotate(&self, turn: &Turn, degrees: u32) -> Location {
        let (sin, cos) = match (turn, degrees) {
            (Turn::Left, 90) => (1, 0),
            (Turn::Left, 180) => (0, -1),
            (Turn::Left, 270) => (-1, 0),
            (Turn::Right, 90) => (-1, 0),
            (Turn::Right, 180) => (0, -1),
            (Turn::Right, 270) => (1, 0),
            (_,_) => panic!("unknown rotation: {:?}, {}", turn, degrees)
        };

        // general formula for rotation about origin
        // new_x = x * cos(a) - y * sin(a)
        // new_y = x * sin(a) + y * cos(a)
        Location {
            x: self.x * cos - self.y * sin,
            y: self.x * sin + self.y * cos
        }
    }
}

#[derive(Debug)]
pub struct Boat {
    loc: Location,
    way: Location
}

impl Boat {
    fn new() -> Boat {
        Boat {
            loc: Location::origin(),
            way: Location { x: 10, y: 1 }
        }
    }

    pub fn simulate(inst: &str) -> Boat {
        let mut boat = Boat::new();

        let instructions: Vec<Instruction> = inst.lines().map({|l| l.parse::<Instruction>().unwrap() }).collect();

        for i in instructions {
            println!("performing: {:?}, {:?}", boat, i);
            boat.perform(i);
            println!("boat: {:?}", boat);
        }

        boat
    }

    pub fn distance_travelled(&self) -> u32 {
        self.loc.manhattan()
    }

    fn move_north(&mut self, amount: u32) { self.way.y += amount as i32; }
    fn move_south(&mut self, amount: u32) { self.way.y -= amount as i32; }
    fn move_west(&mut self, amount: u32) { self.way.x -= amount as i32; }
    fn move_east(&mut self, amount: u32) { self.way.x += amount as i32; }

    fn perform(&mut self, inst: Instruction) {
        match inst {
            Instruction::Forward(amount) => {
                self.loc += self.way * amount
            },

            Instruction::North(amount) => {
                self.move_north(amount)
            },

            Instruction::South(amount) => {
                self.move_south(amount)
            },

            Instruction::East(amount) => {
                self.move_east(amount)
            },

            Instruction::West(amount) => {
                self.move_west(amount)
            },

            Instruction::Right(amount) => {
                self.way = self.way.rotate(&Turn::Right, amount)
            },

            Instruction::Left(amount) => {
                self.way = self.way.rotate(&Turn::Left, amount)
            },
        }
    }
}
