use std::collections::HashMap;
use std::ops::Index;

#[derive(Clone, Debug, PartialEq)]
pub enum Seat {
    Occupied,
    Empty,
    Floor
}

impl Seat {
    pub fn parse(input: char) -> Option<Seat> {
        match input {
            '.' => Some(Seat::Floor),
            'L' => Some(Seat::Empty),
            '#' => Some(Seat::Occupied),
            _ => { println!("unknown character: {:?}", input); None }
        }
    }

    pub fn change(&self) -> Seat {
        match self {
            Seat::Occupied => Seat::Empty,
            Seat::Empty => Seat::Occupied,
            Seat::Floor => Seat::Floor,
        }
    }

    pub fn should_change(&self, nearby: usize) -> bool {
        match self {
            Seat::Floor => false,
            Seat::Empty => nearby == 0,
            Seat::Occupied => nearby >= 4
        }
    }
}

pub struct Layout {
    seats: Vec<Vec<Seat>>,
    pub row_size: usize,
    pub col_size: usize
}

// Kinda proud of this, tbh
impl Index<(usize, usize)> for Layout {
    type Output = Seat;

    fn index(&self, location: (usize, usize)) -> &Seat {
        let (x, y) = location;

        &self.seats[x][y]
    }
}

impl Layout {
    pub fn parse(input: &str) -> Option<Layout> {
        let mut col_size = 0;
        let mut row_size = 0;
        let mut seats: Vec<Vec<Seat>> = Vec::new();

        for line in input.lines() {
            col_size += 1;
            row_size = line.len();

            let mut row: Vec<Seat> = Vec::new();

            for c in line.chars() {
                let seat: Seat = Seat::parse(c)?;
                row.push(seat)
            }

            seats.push(row);
        }

        Some(Layout {
            seats: seats,
            row_size: row_size - 1,
            col_size: col_size - 1
        })
    }

    pub fn num_occupied(&self) -> usize {
        self.seats.iter().flatten().filter(|s| **s == Seat::Occupied).count()
    }

    pub fn occupied(&self, location: (usize, usize)) -> bool {
        self[location] == Seat::Occupied
    }

    pub fn apply(&mut self, changes: &HashMap<(usize, usize), Seat>) {
        for ((x,y), val) in changes.iter() {
            self.seats[*x][*y] = val.clone();
        }
    }

    pub fn occupants_around(&self, location: (usize, usize)) -> usize {
        let mut count = 0;
        let (x, y) = location;

        if x > 0 {
            if y > 0             && self[(x-1, y-1)] == Seat::Occupied { count += 1 }
            if                      self[(x-1, y)] == Seat::Occupied { count += 1 }
            if y < self.row_size && self[(x-1, y+1)] == Seat::Occupied { count += 1 }
        }

        if y > 0 && self[(x, y-1)] == Seat::Occupied { count += 1 }
        // Skip (x,y)
        if y < self.row_size && self[(x, y+1)] == Seat::Occupied { count += 1 }

        if x < self.col_size {
            if y > 0             && self[(x+1, y-1)] == Seat::Occupied { count += 1 }
            if                      self[(x+1, y)] == Seat::Occupied { count += 1 }
            if y < self.row_size && self[(x+1,y+1)] == Seat::Occupied { count += 1 }
        }

        count
    }

    pub fn step(&mut self) -> usize {
        let mut changes: HashMap<(usize, usize), Seat> = HashMap::new();

        for (x, row) in self.seats.iter().enumerate() {
            for (y, seat) in row.iter().enumerate() {
                let nearby = self.occupants_around((x, y));
                if seat.should_change(nearby) {
                    changes.insert((x,y), seat.change());
                }
            }
        }

        self.apply(&changes);

        changes.len()
    }
}
