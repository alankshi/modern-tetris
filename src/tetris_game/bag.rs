use rand::{Rng, rng};

use super::{piece::PieceType, piece::UNIQUE_PIECE_TYPES};

pub struct Bag {
    pub size: u32,
    curr_pieces: u32,
    unique_pieces_remaining: u8,
    pieces: [PieceType; 7],
    piece_counts: [u32; 7],
}

impl Bag {
    pub fn new(size: u32) -> Bag {
        Bag {
            size,
            curr_pieces: 0,
            unique_pieces_remaining: 0,
            pieces: UNIQUE_PIECE_TYPES,
            piece_counts: [0; 7],
        }
    }

    /// Draws a random piece from the remaining pieces in the bag.
    /// Refills the bag if it is empty.
    pub fn draw(&mut self) -> PieceType {
        if self.empty() {
            self.fill();
        }

        let chosen_idx = rng().random_range(0..self.unique_pieces_remaining) as usize;
        self.piece_counts[chosen_idx] -= 1;
        self.curr_pieces -= 1;

        if self.piece_counts[chosen_idx] == 0 {
            self.unique_pieces_remaining -= 1;

            let new_idx = self.unique_pieces_remaining as usize;
            self.pieces.swap(chosen_idx, new_idx);
            self.piece_counts.swap(chosen_idx, new_idx);

            return self.pieces[new_idx];
        }

        return self.pieces[chosen_idx];
    }

    /// Fills the bag with `size` pieces
    fn fill(&mut self) {
        let initial_count = self.size / 7 + if self.size % 7 == 0 { 0 } else { 1 };

        for i in self.unique_pieces_remaining as usize..7 {
            self.piece_counts[i] = initial_count;
        }
        self.curr_pieces = self.size;
        self.unique_pieces_remaining = 7;
    }

    /// Returns `true` if the bag is empty, `false` otherwise
    pub fn empty(&self) -> bool {
        self.curr_pieces == 0
    }
}
