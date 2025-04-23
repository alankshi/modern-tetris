mod tetris_game;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn board_state() {
        let game = tetris_game::SprintGame::new();

        println!("{}", game.board);
    }
}
