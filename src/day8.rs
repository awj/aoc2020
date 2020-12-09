use std::collections::HashSet;

pub struct Machine<'a> {
    pub acc: i32,
    pub instructions: &'a Vec<Instruction>,
    pub executed: HashSet<i32>
}

impl<'a> Machine<'a> {
    pub fn new(instructions: &'a Vec<Instruction>) -> Machine<'a> {
        Machine {
            acc: 0,
            instructions: instructions,
            executed: HashSet::new()
        }
    }

    pub fn run(&mut self) {
        let mut pointer: i32 = 0;

        loop {
            let inst = &self.instructions[pointer as usize];
            println!("running: {:?}, acc: {}, pointer: {}", inst, self.acc, pointer);
            self.acc = inst.acc(self.acc);
            pointer = inst.next(pointer);
            if self.executed.contains(&pointer) {
                break;
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
    Nop
}

impl Instruction {
    pub fn parse(input: &str) -> Instruction {
        let parts: Vec<&str> = input.split(" ").collect();
        println!("parts: {:?}", parts);
        match parts[0] {
            "nop" => Instruction::Nop,
            "acc" => Instruction::Acc(parts[1].parse::<i32>().unwrap()),
            "jmp" => Instruction::Jump(parts[1].parse::<i32>().unwrap()),
            _ => panic!("unknown instruction {}", parts[0])
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
