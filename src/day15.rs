use std::collections::{HashSet, HashMap};
use std::convert::TryFrom;

type Memnum = u32;

#[derive(Debug)]
pub struct MemoryGame {
    latest: Memnum,
    history: HashMap<Memnum, usize>,
    turn: usize
}

impl MemoryGame {
    pub fn next_num(&self) -> Memnum {
        match self.history.get(&self.latest) {
            Some(prev) => { Memnum::try_from(self.turn - prev).unwrap() },
            None => 0
        }
    }

    pub fn nth_spoken(&mut self, count: usize) -> Memnum {
        let mut num = 0;
        while self.turn <= count {
            num = self.next_num();

            if self.turn % 10 == 0 {
                println!("turn: {}, num: {}", self.turn, num);
            }
            self.said(num);
        }

        return num;
    }

    pub fn said(&mut self, num: Memnum) {
        self.history.insert(self.latest, self.turn);
        self.latest = num;
        self.turn += 1;
    }

    pub fn turn(&mut self) -> Memnum {
        let next = self.next_num();

        self.said(next);

        next
    }

    pub fn new(initial: &str) -> MemoryGame {
        let mut game = MemoryGame {
            latest: 0,
            history: HashMap::new(),
            turn: 1
        };

        for piece in initial.split(",") {
            println!("piece: {}", piece);
            let num = piece.parse::<Memnum>().unwrap();

            if game.turn == 1 {
                game.latest = num;
                game.turn = 2;
            } else {
                game.said(num);
            }
        }

        game
    }
}
