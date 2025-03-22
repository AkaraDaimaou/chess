use chess::{Board, ChessMove, Square, MoveGen, Color};
use std::str::FromStr;

pub struct Game {
    board: Board,
    turn: Color,
}

impl Game {
    pub fn new() -> Self {
        Self {
            board: Board::default(),
            turn: Color::White,
        }
    }

    pub fn make_move(&mut self, move_str: &str) -> Result<(), String> {
        let mv = ChessMove::from_str(move_str)
            .map_err(|_| format!("Invalid move: {}", move_str))?;
        
        let legal_moves = MoveGen::new_legal(&self.board);
        if legal_moves.any(|m| m == mv) {
            self.board = self.board.make_move(mv);
            
            // Toggle turn color properly
            self.turn = match self.turn {
                Color::White => Color::Black,
                Color::Black => Color::White,
            };
            
            Ok(())
        } else {
            Err("Illegal move!".to_string())
        }
    }

    pub fn get_board(&self) -> String {
        format!("{}", self.board)
    }
}

fn main() {
    println!("Chess Backend Starting...");
    
    let mut game = Game::new();
    println!("New game created");
    println!("Initial board state:\n{}", game.get_board());
    
    // Here you would set up your Axum server and endpoints
    // For example, exposing endpoints to make moves and get the board state
    
    println!("Server should be configured here...");
}