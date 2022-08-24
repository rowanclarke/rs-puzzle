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

    fn set(&mut self, (x, y): (usize, usize), i: u64) {
        self.possible[(x * 4) + y] = i;
    }

    pub fn place_all(&mut self, (x, y): (usize, usize)) {
        let possible = self.get((x, y));
        for (p, r, i) in Cartesian::new(0..16, 0..4)
            .map(|(p, r)| (p, r, 1 << ((p * 4) + r)))
            .filter(|(_, _, i)| i & possible != 0)
        {
            // TODO
        }
        self.possible[(x * 4) + y] = possible;
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
