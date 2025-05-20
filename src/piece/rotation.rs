use super::Piece;

impl Piece {
    /// Rotate the piece clockwise, changing its orientation accordingly
    pub fn rotate_cw(&mut self) {
        self.orientation = self.orientation.clockwise();
    }

    /// Rotate the piece counterclockwise, changing its orientation accordingly
    pub fn rotate_ccw(&mut self) {
        self.orientation = self.orientation.counterclockwise();
    }
}
