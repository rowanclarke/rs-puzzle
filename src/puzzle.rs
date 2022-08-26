mod piece;

use crate::utils::cartesian::Cartesian;
use piece::*;
use std::collections::LinkedList;

pub struct Puzzle {
    possible: [u64; 16],
    pieces: [Piece; 16],
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

        // Take piece
        for xy in Cartesian::new(0..4, 0..4) {
            self.mask(xy, !(15 << (p * 4)));
        }

        // Remove unmatching from neighbours
        for (i, xy) in [(x, y + 1), (x + 1, y), (x, y - 1), (x - 1, y)]
            .into_iter()
            .enumerate()
        {
            let current = self.get(xy);
            for (po, ro) in Cartesian::new(0..16usize, 0..4usize)
                .filter(|(po, ro)| (1 << ((po * 4) + ro)) & current != 0)
            {
                let side = self.pieces[po][ro];
                // TODO
            }
        }
    }

    pub fn place_all(&mut self, (x, y): (usize, usize)) {
        let state = self.possible;
        let current = self.get((x, y));
        // Go through all possibilities
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
            pieces: [
                Piece::new([
                    Side(Link::Track, Link::Road),
                    Side(Link::River, Link::Track),
                    Side(Link::River, Link::None),
                    Side(Link::Track, Link::Road),
                ]),
                Piece::new([
                    Side(Link::River, Link::Track),
                    Side(Link::Path, Link::Track),
                    Side(Link::River, Link::Road),
                    Side(Link::Road, Link::Path),
                ]),
                Piece::new([
                    Side(Link::Road, Link::Path),
                    Side(Link::River, Link::Path),
                    Side(Link::Road, Link::None),
                    Side(Link::River, Link::Path),
                ]),
                Piece::new([
                    Side(Link::Road, Link::Track),
                    Side(Link::Track, Link::River),
                    Side(Link::Path, Link::Track),
                    Side(Link::Road, Link::River),
                ]),
                Piece::new([
                    Side(Link::Path, Link::River),
                    Side(Link::None, Link::Road),
                    Side(Link::Path, Link::River),
                    Side(Link::None, Link::Road),
                ]),
                Piece::new([
                    Side(Link::Road, Link::Track),
                    Side(Link::Road, Link::None),
                    Side(Link::Track, Link::Path),
                    Side(Link::River, Link::River),
                ]),
                Piece::new([
                    Side(Link::Track, Link::Path),
                    Side(Link::Path, Link::River),
                    Side(Link::River, Link::Road),
                    Side(Link::Road, Link::Track),
                ]),
                Piece::new([
                    Side(Link::Road, Link::River),
                    Side(Link::Path, Link::Track),
                    Side(Link::Track, Link::Road),
                    Side(Link::River, Link::Track),
                ]),
                Piece::new([
                    Side(Link::None, Link::Track),
                    Side(Link::None, Link::Track),
                    Side(Link::Track, Link::River),
                    Side(Link::River, Link::Track),
                ]),
                Piece::new([
                    Side(Link::River, Link::Road),
                    Side(Link::Path, Link::River),
                    Side(Link::Road, Link::River),
                    Side(Link::None, Link::River),
                ]),
                Piece::new([
                    Side(Link::Track, Link::Road),
                    Side(Link::Path, Link::Road),
                    Side(Link::Track, Link::River),
                    Side(Link::River, Link::Path),
                ]),
                Piece::new([
                    Side(Link::Track, Link::Path),
                    Side(Link::River, Link::Road),
                    Side(Link::River, Link::Path),
                    Side(Link::Road, Link::Track),
                ]),
                Piece::new([
                    Side(Link::River, Link::Track),
                    Side(Link::Path, Link::River),
                    Side(Link::Track, Link::Path),
                    Side(Link::River, Link::River),
                ]),
                Piece::new([
                    Side(Link::River, Link::Path),
                    Side(Link::Track, Link::Path),
                    Side(Link::None, Link::Road),
                    Side(Link::Road, Link::River),
                ]),
                Piece::new([
                    Side(Link::Path, Link::Track),
                    Side(Link::Road, Link::River),
                    Side(Link::Track, Link::None),
                    Side(Link::Road, Link::River),
                ]),
                Piece::new([
                    Side(Link::Track, Link::River),
                    Side(Link::River, Link::Road),
                    Side(Link::Road, Link::River),
                    Side(Link::River, Link::Path),
                ]),
            ],
        }
    }
}
