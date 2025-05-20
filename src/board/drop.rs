use crate::{Board, Piece};

impl Board {
    /// Hard drops the passed piece, consuming it, then updates the board to
    /// represent the placed piece.
    pub fn hard_drop(&mut self, mut piece: Piece) {
        piece.hard_drop(self.column_heights);

        for pos in piece.get_pos_mask() {
            self.board[pos.y() as usize][pos.x() as usize] = Some(());
        }

        for (i, y_offset) in piece.upper_edge().into_iter().flatten() {
            self.column_heights[(piece.x() + i as i32) as usize] = piece.y() as u8 - y_offset + 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::PieceType;

    #[test]
    fn hard_drop() {
        let mut board = Board::new();

        for _ in 0..21 {
            board.hard_drop(Piece::new(PieceType::I));
        }

        println!("{}", board);
        println!("{:?}", board.column_heights);
    }
}
