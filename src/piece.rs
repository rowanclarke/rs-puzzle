pub enum Link {
    None,
    Road,
    Track,
    Path,
    River,
}

pub struct Side(Link, Link);

pub struct Piece {
    sides: [Side; 4],
}

impl Piece {
    fn new(sides: [Side; 4]) -> Self {
        Self { sides }
    }
}

pub const pieces: [Piece; 16] = [
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
];
