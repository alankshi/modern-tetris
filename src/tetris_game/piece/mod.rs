pub mod util;

use util::Position;
pub use util::{PieceType, UNIQUE_PIECE_TYPES};

pub struct Piece {
    kind: PieceType,
    rotation: u8,
    position: Position,
}

impl Piece {
    #[must_use]
    pub fn new(kind: PieceType) -> Piece {
        Piece {
            kind,
            rotation: 0,
            position: Position::new(),
        }
    }
}
