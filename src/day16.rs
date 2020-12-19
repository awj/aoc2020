use std::str::FromStr;
use std::num::ParseIntError;

#[derive(Debug)]
pub struct Condition {
    min: u32,
    max: u32
}

impl FromStr for Condition {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split("-").collect();

        Ok(Condition {
            min: parts[0].parse::<u32>()?,
            max: parts[1].parse::<u32>()?
        })
    }
}

impl Condition {
    fn valid(&self, input: u32) -> bool {
        self.min <= input && input <= self.max
    }
}

#[derive(Debug)]
pub struct Rule<'a> {
    field: &'a str,
    conditions: Vec<Condition>
}

impl<'a> Rule<'a> {
    fn new(s: &'a str) -> Result<Self, ParseIntError> {
        let halves: Vec<&str> = s.split(": ").collect();
        let field = halves[0];

        let mut conditions = Vec::new();

        for piece in halves[1].split(" or ") {
            conditions.push(piece.parse::<Condition>()?);
        }

        Ok(Rule{
            field: field,
            conditions: conditions
        })
    }

    fn valid(&self, input: u32) -> bool {
        self.conditions.iter().any(|c| c.valid(input))
    }
}

fn check(rules: &Vec<Rule>, input: u32) -> bool {
    rules.iter().any(|r| r.valid(input))
}

type Ticket = Vec<u32>;

fn invalid_value(rules: &Vec<Rule>, input: &Ticket) -> Option<u32> {
    for i in input {
        if rules.iter().all(|r| !r.valid(*i)) {
            return Some(*i)
        }
    }

    None
}

fn error_rate(rules: &Vec<Rule>, inputs: &Vec<Ticket>) -> u32 {
    let mut rate = 0;

    for ticket in inputs {
        if let Some(val) = invalid_value(&rules, &ticket) {
            println!("bad ticket: {:?}, val: {}", ticket, val);
            rate += val;
        }
    }

    rate
}

enum ParseState {
    Rules,
    YourTicket,
    OtherTickets
}

fn ticket(line: &str) -> Ticket {
    println!("ticketing: '{}'", line);
    line.split(",").map(|c| c.parse::<u32>().unwrap()).collect()
}

pub fn run(input: &str) {
    let mut rules = Vec::new();
    let mut your_ticket = None;
    let mut tickets = Vec::new();

    let mut parse_state = ParseState::Rules;

    for line in input.lines() {
        if line.is_empty() {
            match parse_state {
                ParseState::Rules => parse_state = ParseState::YourTicket,
                ParseState::YourTicket => parse_state = ParseState::OtherTickets,
                _ => panic!("dunno how this happened!")
            }
        } else {
            if line != "your ticket:" && line != "nearby tickets:" {
                match parse_state {
                    ParseState::Rules => rules.push(Rule::new(line).unwrap()),
                    ParseState::YourTicket => your_ticket = Some(ticket(line)),
                    ParseState::OtherTickets => tickets.push(ticket(line))
                }
            }
        }
    }

    println!("rules: {:?}", rules);

    println!("error rate: {}", error_rate(&rules, &tickets));
}
