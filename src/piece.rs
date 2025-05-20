mod movement;
mod piece_type;
mod position;
mod rotation;
mod util;

pub use piece_type::PieceType;
use position::Position;
use util::Orientation;
pub use util::{DEFAULT_ORIENTATION, DEFAULT_POSITION, UNIQUE_TYPES};

pub struct Piece {
    kind: PieceType,
    mask: [u8; 4],
    orientation: Orientation,
    position: Position,
}

impl Piece {
    #[must_use]
    /// Creates a new piece of kind `kind` with a north orientation at (0, 0)
    pub fn new(kind: PieceType) -> Piece {
        Piece {
            kind,
            mask: kind.mask(DEFAULT_ORIENTATION),
            orientation: DEFAULT_ORIENTATION,
            position: DEFAULT_POSITION,
        }
    }

    #[must_use]
    /// Returns the PieceType of the piece
    pub fn kind(&self) -> PieceType {
        self.kind
    }

    #[must_use]
    /// Returns the mask of the piece
    pub fn mask(&self) -> [u8; 4] {
        self.mask
    }

    #[must_use]
    /// Returns the orientation of the piece
    pub fn orientation(&self) -> Orientation {
        self.orientation
    }

    #[must_use]
    /// Returns the position of the piece
    pub fn position(&self) -> &Position {
        &self.position
    }

    #[must_use]
    /// Returns the x position of the piece
    pub fn x(&self) -> i32 {
        self.position.x()
    }

    #[must_use]
    /// Returns the y position of the piece
    pub fn y(&self) -> i32 {
        self.position.y()
    }
}
