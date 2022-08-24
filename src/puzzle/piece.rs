use std::{iter, slice};

#[derive(Copy, Clone)]
pub enum Link {
    None,
    Road,
    Track,
    Path,
    River,
}

#[derive(Copy, Clone)]
pub struct Side(pub Link, pub Link);

#[derive(Copy, Clone)]
pub struct Piece {
    sides: [Side; 4],
}

impl Piece {
    pub fn new(sides: [Side; 4]) -> Self {
        Self { sides }
    }

    pub fn sides(&self) -> &[Side] {
        self.sides.as_ref()
    }
}
