mod piece;

use piece::*;

pub struct Puzzle {
    possible: [Option<Side>; 16],
}
