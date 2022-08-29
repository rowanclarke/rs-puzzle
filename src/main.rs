pub mod puzzle;
pub mod utils;

use puzzle::piece::*;
use puzzle::*;
use utils::cartesian::*;

fn main() {
    let mut puzzle = Puzzle::new();
    let result = puzzle.place_all(puzzle.get_least().unwrap());
    println!("{:?}", result.ok());
}
