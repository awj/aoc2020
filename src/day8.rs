use std::collections::HashSet;

pub struct Machine<'a> {
    pub acc: i32,
    pub instructions: &'a Vec<Instruction>,
    pub executed: HashSet<i32>,
    pub flip: i32
}

pub enum ExecutionResult {
    // Execution looped and was short-circuited
    Loop,
    // Execution terminated successfully
    Terminate
}

impl<'a> Machine<'a> {
    pub fn new(instructions: &'a Vec<Instruction>) -> Machine<'a> {
        Machine {
            acc: 0,
            instructions: instructions,
            executed: HashSet::new(),
            flip: -1
        }
    }

    pub fn with_flip(instructions: &'a Vec<Instruction>, flip: i32) -> Machine<'a> {
        Machine {
            acc: 0,
            instructions: instructions,
            executed: HashSet::new(),
            flip: flip
        }
    }

    pub fn run(&mut self) -> ExecutionResult {
        let mut pointer: i32 = 0;

        loop {
            let inst = &self.instructions[pointer as usize];
            println!("running: {:?}, acc: {}, pointer: {}", inst, self.acc, pointer);
            self.acc = inst.acc(self.acc);
            if pointer == self.flip {
                pointer = inst.flip(pointer)
            } else {
                pointer = inst.next(pointer);
            }

            if self.executed.contains(&pointer) {
                return ExecutionResult::Loop
            } else if pointer == (self.instructions.len() as i32) {
                return ExecutionResult::Terminate
            } else {
                self.executed.insert(pointer);
            }
        }
    }
}

#[derive(Debug)]
pub enum Instruction {
    Acc(i32),
    Jump(i32),
    Nop(i32)
}

impl Instruction {
    pub fn parse(input: &str) -> Instruction {
        let parts: Vec<&str> = input.split(" ").collect();
        println!("parts: {:?}", parts);
        let offset = parts[1].parse::<i32>().unwrap();

        match parts[0] {
            "nop" => Instruction::Nop(offset),
            "acc" => Instruction::Acc(offset),
            "jmp" => Instruction::Jump(offset),
            _ => panic!("unknown instruction {}", parts[0])
        }
    }

    pub fn flip(&self, location: i32) -> i32 {
        match self {
            Instruction::Nop(distance) => location + distance,
            _ => location + 1
        }
    }

    // Get the instruction offset for this particular instruction
    pub fn next(&self, location: i32) -> i32 {
        match self {
            Instruction::Jump(distance) => location + distance,
            _ => location + 1
        }
    }

    pub fn acc(&self, original_acc: i32) -> i32 {
        match self {
            Instruction::Acc(val) => original_acc + val,
            _ => original_acc
        }
    }
}
