use std::collections::HashSet;
use std::convert::TryFrom;

type Memnum = u32;

#[derive(Debug)]
pub struct MemoryGame {
    latest: Memnum,
    spoken: HashSet<Memnum>,
    history: Vec<Memnum>,
    turn: usize
}

impl MemoryGame {
    pub fn next_num(&self) -> Memnum {
        if self.spoken.contains(&self.latest) {

            for(i,v) in self.history.iter().enumerate().rev() {
                if *v == self.latest {
                    return u32::try_from((self.turn - 1) - (i + 1)).unwrap();
                }
            }
            panic!("should not be here!");
        } else {
            0
        }
    }

    pub fn nth_spoken(&mut self, count: usize) -> Memnum {
        let mut num = 0;
        while self.turn <= count {
            num = self.next_num();

            println!("turn: {}, num: {}", self.turn, num);
            self.said(num);
        }

        return num;
    }

    pub fn said(&mut self, num: Memnum) {
        self.spoken.insert(self.latest);
        self.history.push(self.latest);
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
            spoken: HashSet::new(),
            history: Vec::new(),
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
