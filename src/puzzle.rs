mod piece;

use crate::utils::cartesian::Cartesian;
use piece::*;

pub struct Puzzle {
    possible: [u64; 16],
}

impl Puzzle {
    fn get(&self, (x, y): (usize, usize)) -> u64 {
        self.possible[(x * 4) + y]
    }

    fn mask(&mut self, (x, y): (usize, usize), m: u64) {
        self.possible[(x * 4) + y] &= m;
    }

    fn set(&mut self, (x, y): (usize, usize), (p, r): (usize, usize)) {
        // Set piece
        self.mask((x, y), 1 << ((p * 4) + r));

        // Take piece from all possibilities
        for xy in Cartesian::new(0..4, 0..4) {
            self.mask(xy, !(15 << (p * 4)));
        }

        // Remove unmatching from neighbours
        for (i, xy) in [(x, y + 1), (x + 1, y), (x, y - 1), (x - 1, y)]
            .into_iter()
            .enumerate()
        {
            // Side to match
            let side = pieces[p][i];
            // Go through all possibilities
            let current = self.get(xy);
            for (po, ro) in Cartesian::new(0..16, 0..4).filter(|&(po, ro)| {
                (1 << ((po * 4) + ro)) & current != 0 && pieces[po][(ro + 2) % 4] != side
            }) {
                // Remove unmatching sides
                self.mask(xy, !(1 << ((po * 4) + ro)));
            }
        }
    }

    pub fn place_all(&mut self, (x, y): (usize, usize)) {
        let state = self.possible;
        // Go through all possibilities
        let current = self.get((x, y));
        for (p, r) in
            Cartesian::new(0..16, 0..4).filter(|(p, r)| (1 << ((p * 4) + r)) & current != 0)
        {
            // Set possibility
            self.set((x, y), (p, r));
            // TODO place next piece
            // Undo changes
            self.possible = state;
        }
    }

    pub fn new() -> Self {
        Self {
            possible: [u64::MAX; 16],
        }
    }
}
