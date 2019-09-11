use crate::board::Board;
use crate::color::Color;

pub struct Game {
	board: Board,
	turn: Color,
}

impl Game {
	pub fn new() -> Game {
		let board = Board::new();

		Game {
			board: board,
			turn: Color::White,
		}
	}

	pub fn print_board(&self) {
		self.board.print();
	}

	pub fn move_piece(
		&mut self,
		r1: usize, c1: usize,
		r2: usize, c2: usize
	) -> Result<(), String> {
		if self.board.board[r1][c1].is_none() {
			return Err("There is no piece to move".to_string());
		}

		if self.board.board[r1][c1].unwrap().color != self.turn {
			return Err("You can't move the opponent's piece".to_string());
		}

		if self.board.board[r2][c2].is_some() {
			if self.board.board[r2][c2].unwrap().color == self.turn {
				return Err("You can't take your own piece".to_string());
			} else {
				// take the opponent's piece
				self.board.board[r2][c2] = None;
			}
		}

		self.board.move_piece(r1, c1, r2, c2);

		self.turn = match self.turn {
			Color::White => Color::Black,
			Color::Black => Color::White,
		};

		Ok(())
	}
}
