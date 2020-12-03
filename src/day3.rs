use std::convert::TryFrom;

#[derive(Debug)]
pub struct Forest {
    map: Vec<Vec<bool>>,
    width: i32,
    height: i32
}

impl Forest {
    pub fn trees_along(&self, slope: (i32, i32)) -> i32 {
        self._trees_along(slope, (0,0), 0)
    }

    pub fn parse(input: std::str::Lines) -> Forest {
        let mut height: i32 = 0;
        let mut width: i32 = 0;

        let map = input.map(|line| {
            width = i32::try_from(line.len()).unwrap();
            height += 1;
            line.chars().map(|c| c == '#').collect()
        }).collect();

        Forest {
            map: map,
            height: height,
            width: width
        }
    }

    fn _trees_along(&self, slope: (i32, i32), pos: (i32, i32), num_trees: i32) -> i32 {
        let (_x, y) = pos;

        if y >= self.height {
            num_trees
        } else {
            let next_pos = self.add(pos, slope);

            match self.tree_at(next_pos) {
                true => self._trees_along(slope, next_pos, num_trees + 1),
                false => self._trees_along(slope, next_pos, num_trees)
            }
        }
    }

    fn tree_at(&self, pos: (i32, i32)) -> bool {
        let (x, y) = pos;
        if y >= self.height {
            return false
        }

        let row = &self.map[y as usize];
        row[x as usize]
    }

    fn add(&self, pos: (i32, i32), slope: (i32, i32)) -> (i32, i32) {
        let (x, y) = pos;
        let (dx, dy) = slope;

        let nx = (x + dx) % self.width;

        (nx, y + dy)
    }
}
