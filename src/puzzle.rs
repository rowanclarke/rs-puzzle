pub mod piece;

use crate::utils::cartesian::Cartesian;
use piece::*;

pub struct Puzzle {
    pub possible: [u64; 16],
    pub visited: u16,
}

impl Puzzle {
    fn get(&self, (x, y): (usize, usize)) -> Option<u64> {
        if x > 3 || y > 3 {
            return None;
        }
        Some(self.possible[(x * 4) + y])
    }

    fn mask(&mut self, (x, y): (usize, usize), m: u64) {
        self.possible[(x * 4) + y] &= m;
    }

    pub fn set(&mut self, (x, y): (usize, usize), (p, r): (usize, usize)) {
        // Take piece from all possibilities
        for xy in Cartesian::new(0..4, 0..4) {
            self.mask(xy, !(15 << (p * 4)));
        }
        // Set and visit piece
        self.possible[(x * 4) + y] = 1 << ((p * 4) + r);
        self.visited |= 1 << ((x * 4) + y);

        // Remove unmatching from neighbours
        for (i, xy) in [
            (x.checked_add(1), Some(y)),
            (Some(x), y.checked_add(1)),
            (x.checked_sub(1), Some(y)),
            (Some(x), y.checked_sub(1)),
        ]
        .into_iter()
        .enumerate()
        .filter_map(|(i, xy)| match xy {
            (Some(x), Some(y)) => Some((i, (x, y))),
            _ => None,
        }) {
            // Side to match
            let side = PIECES[p][(r + i) % 4];
            // Go through all possibilities
            if let Some(current) = self.get(xy) {
                for (po, ro) in Cartesian::new(0..16, 0..4).filter(|&(po, ro)| {
                    (1 << ((po * 4) + ro)) & current != 0 && PIECES[po][(ro + i + 2) % 4] != side
                }) {
                    // Remove unmatching sides
                    self.mask(xy, !(1 << ((po * 4) + ro)));
                }
            }
        }
    }

    // Get the (x, y) of the piece with the most constrait for optimisation
    pub fn get_least(&self) -> Option<(usize, usize)> {
        Cartesian::new(0..4, 0..4)
            .filter(|&(x, y)| self.visited & (1 << ((x * 4) + y)) == 0)
            .min_by_key(|&xy| {
                (0..64)
                    .map(|m| (1 << m) & self.get(xy).unwrap())
                    .filter(|&x| x != 0)
                    .count()
            })
    }

    // TODO Replace with place_next(&mut self)
    pub fn place_all(
        &mut self,
        (x, y): (usize, usize),
    ) -> Result<Vec<((usize, usize), (usize, usize))>, NoSolutionError> {
        let state = (self.possible, self.visited);
        // Go through all possibilities
        let current = self.get((x, y)).unwrap();
        for (p, r) in
            Cartesian::new(0..16, 0..4).filter(|(p, r)| (1 << ((p * 4) + r)) & current != 0)
        {
            // Set piece
            self.set((x, y), (p, r));
            if let Some(least) = self.get_least() {
                // Place next piece;
                match self.place_all(least) {
                    Ok(mut vec) => {
                        // Push solution
                        vec.push(((x, y), (p, r)));
                        return Ok(vec);
                    }
                    Err(_) => {
                        // Undo changes
                        (self.possible, self.visited) = state;
                    }
                }
            } else {
                let mut vec = Vec::with_capacity(16);
                vec.push(((x, y), (p, r)));
                return Ok(vec);
            }
        }
        return Err(NoSolutionError);
    }

    pub fn new() -> Self {
        Self {
            possible: [u64::MAX; 16],
            visited: 0,
        }
    }
}

pub struct NoSolutionError;
