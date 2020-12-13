use std::convert::TryInto;
use std::str::FromStr;
use std::num::ParseIntError;

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

impl Direction {
    fn flip(&self) -> Direction {
        match self {
            Direction::North => Direction::South,
            Direction::South => Direction::North,
            Direction::East => Direction::West,
            Direction::West => Direction::East,
        }
    }

    fn change(&self, turn: &Turn, amount: u32) -> Direction {
        match (self, turn, amount) {
            // all 180 degree turns are a flip
            (_, _, 180) => self.flip(),

            (Direction::North, Turn::Right, 90) => Direction::East,
            (Direction::North, Turn::Right, 270) => Direction::West,

            (Direction::North, Turn::Left, 90) => Direction::West,
            (Direction::North, Turn::Left, 270) => Direction::East,

            (Direction::South, Turn::Right, 90) => Direction::West,
            (Direction::South, Turn::Right, 270) => Direction::East,

            (Direction::South, Turn::Left, 90) => Direction::East,
            (Direction::South, Turn::Left, 270) => Direction::West,

            (Direction::East, Turn::Right, 90) => Direction::South,
            (Direction::East, Turn::Right, 270) => Direction::North,

            (Direction::East, Turn::Left, 90) => Direction::North,
            (Direction::East, Turn::Left, 270) => Direction::South,

            (Direction::West, Turn::Right, 90) => Direction::North,
            (Direction::West, Turn::Right, 270) => Direction::South,

            (Direction::West, Turn::Left, 90) => Direction::South,
            (Direction::West, Turn::Left, 270) => Direction::North,

            (_, _, _) => panic!("unexpected args: {:?}, {:?}, {:?}", self, turn, amount),
        }
    }
}

#[derive(Debug)]
pub struct Location {
    x: i32,
    y: i32
}

impl Location {
    pub fn origin() -> Location {
        Location { x: 0, y: 0 }
    }
    pub fn manhattan(&self) -> u32 {
        (self.x.abs() + self.y.abs()).try_into().unwrap()
    }
}

#[derive(Debug)]
pub struct Boat {
    loc: Location,
    dir: Direction
}

impl Boat {
    fn new() -> Boat {
        Boat {
            loc: Location::origin(),
            dir: Direction::East
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

    fn move_north(&mut self, amount: u32) { self.loc.y += amount as i32; }
    fn move_south(&mut self, amount: u32) { self.loc.y -= amount as i32; }
    fn move_west(&mut self, amount: u32) { self.loc.x -= amount as i32; }
    fn move_east(&mut self, amount: u32) { self.loc.x += amount as i32; }

    fn perform(&mut self, inst: Instruction) {
        match inst {
            Instruction::Forward(amount) => {
                match self.dir {
                    Direction::North => self.move_north(amount),
                    Direction::South => self.move_south(amount),
                    Direction::East => self.move_east(amount),
                    Direction::West => self.move_west(amount)
                }
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
                self.dir = self.dir.change(&Turn::Right, amount)
            },

            Instruction::Left(amount) => {
                self.dir = self.dir.change(&Turn::Left, amount)
            },
        }
    }
}
