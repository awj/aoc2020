#[derive(Debug)]
pub struct Seat {
    row: u32,
    col: u32
}

impl Seat {
    pub fn parse(input: &str) -> Seat {
        let mut rmin = 0;
        let mut rmax = 127;
        let mut cmin = 0;
        let mut cmax = 7;

        let parts = input.chars();

        for c in parts {
            match c {
                'F' => rmax = rmin + ((rmax - rmin) / 2),
                'B' => rmin = rmin + 1 + ((rmax - rmin) / 2),
                'R' => cmin = cmin + 1 + ((cmax - cmin) / 2),
                'L' => cmax = cmin + ((cmax - cmin) / 2),
                _ => panic!("unknown char {}", c)
            }
        }

        if rmin != rmax || cmin != cmax {
            panic!("indeterminant input: {} rmin {} rmax {} cmin {} cmax {}", input, rmin, rmax, cmin, cmax)
        }

        Seat {
            row: rmax,
            col: cmax
        }
    }

    pub fn seat_id(&self) -> u32 {
        (self.row * 8) + self.col
    }
}
