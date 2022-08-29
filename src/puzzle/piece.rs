#[derive(Copy, Clone, PartialEq)]
pub enum Link {
    None,
    Road,
    Track,
    Path,
    River,
}

#[derive(Copy, Clone)]
pub struct Side(pub Link, pub Link);

impl PartialEq<Self> for Side {
    fn eq(&self, rhs: &Self) -> bool {
        // Can they fit together?
        return self.0 == rhs.1 && self.1 == rhs.0;
    }
}

pub type Piece = [Side; 4];

pub const PIECES: [Piece; 16] = [
    [
        // 1 (Yellow Fisherman)
        Side(Link::Track, Link::River),
        Side(Link::Path, Link::Track),
        Side(Link::Road, Link::River),
        Side(Link::Road, Link::Track),
    ],
    [
        // 2 (Milk Truck)
        Side(Link::Track, Link::Road),
        Side(Link::River, Link::Track),
        Side(Link::River, Link::None),
        Side(Link::Track, Link::Road),
    ],
    [
        // 3 (Red-Ore Train)
        Side(Link::Path, Link::River),
        Side(Link::Track, Link::Path),
        Side(Link::River, Link::River),
        Side(Link::River, Link::Track),
    ],
    [
        // 4 (Orange-Ore Train)
        Side(Link::Track, Link::River),
        Side(Link::River, Link::Track),
        Side(Link::None, Link::Track),
        Side(Link::None, Link::Track),
    ],
    [
        // 5 (Red Fisherman)
        Side(Link::None, Link::Road),
        Side(Link::Path, Link::River),
        Side(Link::None, Link::Road),
        Side(Link::Path, Link::River),
    ],
    [
        // 6 (Black-White-Tailed Dog)
        Side(Link::Track, Link::Path),
        Side(Link::None, Link::Road),
        Side(Link::Road, Link::River),
        Side(Link::River, Link::Path),
    ],
    [
        // 7 (Blue Car)
        Side(Link::Path, Link::Track),
        Side(Link::Road, Link::River),
        Side(Link::Track, Link::None),
        Side(Link::Road, Link::River),
    ],
    [
        // 8 (Apple Truck)
        Side(Link::River, Link::Road),
        Side(Link::River, Link::Path),
        Side(Link::Road, Link::Track),
        Side(Link::Track, Link::Path),
    ],
    [
        // 9 (Red Car)
        Side(Link::Road, Link::Track),
        Side(Link::Road, Link::None),
        Side(Link::Track, Link::Path),
        Side(Link::River, Link::River),
    ],
    [
        // 10 (White-Tailed Dog)
        Side(Link::River, Link::Path),
        Side(Link::Track, Link::River),
        Side(Link::River, Link::Road),
        Side(Link::Road, Link::River),
    ],
    [
        // 11 (Black-Tailed Dog)
        Side(Link::Track, Link::Path),
        Side(Link::Path, Link::River),
        Side(Link::River, Link::Road),
        Side(Link::Road, Link::Track),
    ],
    [
        // 12 (Yellow Boat)
        Side(Link::River, Link::Track),
        Side(Link::Road, Link::River),
        Side(Link::Path, Link::Track),
        Side(Link::Track, Link::Road),
    ],
    [
        // 13 (Sun Truck)
        Side(Link::Path, Link::River),
        Side(Link::Road, Link::River),
        Side(Link::None, Link::River),
        Side(Link::River, Link::Road),
    ],
    [
        // 14 (Yellow Car)
        Side(Link::Track, Link::River),
        Side(Link::River, Link::Path),
        Side(Link::Track, Link::Road),
        Side(Link::Path, Link::Road),
    ],
    [
        // 15 (Orange Boat)
        Side(Link::Road, Link::Path),
        Side(Link::River, Link::Path),
        Side(Link::Road, Link::None),
        Side(Link::River, Link::Path),
    ],
    [
        // 16 (Blue-Ore Train)
        Side(Link::Path, Link::Track),
        Side(Link::River, Link::Road),
        Side(Link::Road, Link::Path),
        Side(Link::River, Link::Track),
    ],
];
