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

pub type Piece = [Side; 4];

pub const pieces: [Piece; 16] = [
    [
        Side(Link::Track, Link::Road),
        Side(Link::River, Link::Track),
        Side(Link::River, Link::None),
        Side(Link::Track, Link::Road),
    ],
    [
        Side(Link::River, Link::Track),
        Side(Link::Path, Link::Track),
        Side(Link::River, Link::Road),
        Side(Link::Road, Link::Path),
    ],
    [
        Side(Link::Road, Link::Path),
        Side(Link::River, Link::Path),
        Side(Link::Road, Link::None),
        Side(Link::River, Link::Path),
    ],
    [
        Side(Link::Road, Link::Track),
        Side(Link::Track, Link::River),
        Side(Link::Path, Link::Track),
        Side(Link::Road, Link::River),
    ],
    [
        Side(Link::Path, Link::River),
        Side(Link::None, Link::Road),
        Side(Link::Path, Link::River),
        Side(Link::None, Link::Road),
    ],
    [
        Side(Link::Road, Link::Track),
        Side(Link::Road, Link::None),
        Side(Link::Track, Link::Path),
        Side(Link::River, Link::River),
    ],
    [
        Side(Link::Track, Link::Path),
        Side(Link::Path, Link::River),
        Side(Link::River, Link::Road),
        Side(Link::Road, Link::Track),
    ],
    [
        Side(Link::Road, Link::River),
        Side(Link::Path, Link::Track),
        Side(Link::Track, Link::Road),
        Side(Link::River, Link::Track),
    ],
    [
        Side(Link::None, Link::Track),
        Side(Link::None, Link::Track),
        Side(Link::Track, Link::River),
        Side(Link::River, Link::Track),
    ],
    [
        Side(Link::River, Link::Road),
        Side(Link::Path, Link::River),
        Side(Link::Road, Link::River),
        Side(Link::None, Link::River),
    ],
    [
        Side(Link::Track, Link::Road),
        Side(Link::Path, Link::Road),
        Side(Link::Track, Link::River),
        Side(Link::River, Link::Path),
    ],
    [
        Side(Link::Track, Link::Path),
        Side(Link::River, Link::Road),
        Side(Link::River, Link::Path),
        Side(Link::Road, Link::Track),
    ],
    [
        Side(Link::River, Link::Track),
        Side(Link::Path, Link::River),
        Side(Link::Track, Link::Path),
        Side(Link::River, Link::River),
    ],
    [
        Side(Link::River, Link::Path),
        Side(Link::Track, Link::Path),
        Side(Link::None, Link::Road),
        Side(Link::Road, Link::River),
    ],
    [
        Side(Link::Path, Link::Track),
        Side(Link::Road, Link::River),
        Side(Link::Track, Link::None),
        Side(Link::Road, Link::River),
    ],
    [
        Side(Link::Track, Link::River),
        Side(Link::River, Link::Road),
        Side(Link::Road, Link::River),
        Side(Link::River, Link::Path),
    ],
];
