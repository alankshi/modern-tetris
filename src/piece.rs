mod movement;
mod position;
mod rotation;
mod util;

use position::Position;
use util::Orientation;
pub use util::{DEFAULT_POSITION, PieceType, UNIQUE_TYPES};

pub struct Piece {
    kind: PieceType,
    orientation: Orientation,
    position: Position,
}

impl Piece {
    #[must_use]
    /// Creates a new piece of kind `kind` with a north orientation at (0, 0)
    pub fn new(kind: PieceType) -> Piece {
        Piece {
            kind,
            orientation: Orientation::North,
            position: DEFAULT_POSITION,
        }
    }

    /// Returns the PieceType of the piece
    pub fn kind(&self) -> PieceType {
        self.kind
    }

    /// Returns the orientation of the piece
    pub fn orientation(&self) -> Orientation {
        self.orientation
    }

    /// Returns the position of the piece
    pub fn position(&self) -> &Position {
        &self.position
    }

    /// Returns the x position of the piece
    pub fn x(&self) -> i32 {
        self.position.x()
    }

    /// Returns the y position of the piece
    pub fn y(&self) -> i32 {
        self.position.y()
    }
}
