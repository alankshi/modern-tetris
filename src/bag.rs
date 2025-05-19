extern crate rand;

use self::rand::{Rng, rng};

use crate::piece::{PieceType, UNIQUE_TYPES};

#[derive(Debug)]
pub struct Bag {
    size: u32,
    curr_pieces: u32,
    unique_pieces_remaining: u8,
    pieces: [PieceType; 7],
    piece_counts: [u32; 7],
}

impl Bag {
    #[must_use]
    /// Creates a new, empty bag of size `size`
    pub fn new(size: u32) -> Bag {
        Bag {
            size,
            curr_pieces: 0,
            unique_pieces_remaining: 0,
            pieces: UNIQUE_TYPES,
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

        self.pieces[chosen_idx]
    }

    /// Fills the bag with `self.size` pieces
    fn fill(&mut self) {
        let initial_count = self.size / 7 + u32::from(self.size % 7 != 0);

        for i in self.unique_pieces_remaining as usize..7 {
            self.piece_counts[i] = initial_count;
        }
        self.curr_pieces = self.size;
        self.unique_pieces_remaining = 7;
    }

    #[must_use]
    /// Returns `true` if the bag is empty, `false` otherwise
    pub fn empty(&self) -> bool {
        self.curr_pieces == 0
    }

    #[must_use]
    /// Returns `self.size`, the bag size
    pub fn size(&self) -> u32 {
        self.size
    }
}

#[cfg(test)]
mod tests {
    use super::Bag;
    use std::collections::HashMap;

    #[test]
    fn bag_emptying() {
        let mut my_bag = Bag::new(7);
        assert!(my_bag.empty(), "bag should start out empty");
        for _ in 0..7 {
            my_bag.draw();
        }
        assert!(
            my_bag.empty(),
            "Bag should be empty after drawing size times"
        );
    }

    #[test]
    fn bag_drawing() {
        let mut my_bag = Bag::new(21);
        let mut piece_counts = HashMap::new();
        for _ in 0..21 {
            *(piece_counts.entry(my_bag.draw()).or_insert(0)) += 1;
        }

        for (_, count) in piece_counts {
            assert!(
                count == 3,
                "Every piece should be seen exactly thrice when drawing 21 pieces from a 21 sized bag"
            );
        }
    }
}
