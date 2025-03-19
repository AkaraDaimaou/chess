use chess::{Board, ChessMove, Square, MoveGen};
use std::str::FromStr;

pub struct Game {
    board: Board,
    turn: chess::Color,
}

impl Game {
    pub fn new() -> Self {
        Self {
            board: Board::default(),
            turn: chess::Color::White,
        }
    }

    pub fn make_move(&mut self, move_str: &str) -> Result<(), String> {
        let mv = ChessMove::from_str(move_str)
            .map_err(|_| format!("Invalid move: {}", move_str))?;
        
        let mut legal_moves = MoveGen::new_legal(&self.board);
        if legal_moves.any(|m| m == mv) {
            self.board = self.board.make_move_new(mv);
            self.turn = self.turn.flip();
            Ok(())
        } else {
            Err("Illegal move!".to_string())
        }
    }

    pub fn get_board(&self) -> String {
        format!("{}", self.board)
    }
}
