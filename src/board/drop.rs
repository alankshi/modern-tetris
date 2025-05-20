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
    fn empty_board_drop_flat_i_center() {
        let mut board = Board::new();
        let mut correct_col_heights = [0u8; 10];

        for i in 0..22 {
            board.hard_drop(Piece::new(PieceType::I));
            correct_col_heights[3] += 1;
            correct_col_heights[4] += 1;
            correct_col_heights[5] += 1;
            correct_col_heights[6] += 1;

            assert_eq!(
                correct_col_heights,
                board.column_heights,
                "Incorrect column heights after placing {} flat I-pieces. Expected {:?}, was {:?}",
                i + 1,
                correct_col_heights,
                board.column_heights,
            );
        }
    }
}
