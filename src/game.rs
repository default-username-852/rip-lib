use crate::board::Board;
use crate::color::Color;
use crate::name::Name;
use crate::piece::Piece;

pub struct Game {
	board: Board,
	turn: Color,
	moved_last: Option<Piece>,
}

impl Game {
	pub fn new() -> Game {
		let board = Board::new();

		Game {
			board: board,
			turn: Color::White,
			moved_last: None,
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

		// check that the piece is allowed to go there
		match self.board.board[r1][c1].unwrap().name {
			Name::King => {
				if
					(r1 as isize - r2 as isize).abs() != 1 &&
					(c1 as isize - c2 as isize).abs() != 1
				{
					return Err("The king cannot move to that position".to_string());
				}
			},
			Name::Queen => {
				if
					r1 != r2 && c1 != c2 &&
					(r1 as isize - r2 as isize).abs() !=
						(c1 as isize - c2 as isize).abs()
				{
					return Err("The queen cannot move to that position".to_string());
				}
			},
			Name::Rook => {
				if r1 != r2 && c1 != c2 {
					return Err("The rook cannot move to that position".to_string());
				}
			},
			Name::Bishop => {
				if
					(r1 as isize - r2 as isize).abs() !=
						(c1 as isize - c2 as isize).abs()
				{
					return Err("The bishop cannot move to that position".to_string());
				}
			},
			Name::Knight => {
				// the Pythagorean theorem tells us that the knight can only move
				// to positions within sqrt(5) of its original position
				if
						(r1 as isize - r2 as isize).pow(2) +
						(c1 as isize - c2 as isize).pow(2) != 5
				{
					println!("{}", (
						(r1 as isize - r2 as isize).pow(2) +
						(c1 as isize - c2 as isize).pow(2)
					).pow(2));
					return Err("The knight cannot move to that position".to_string());
				}
			},
			Name::Pawn => {
				// check if the pawn is trying to move backwards
				if self.turn == Color::White {
					if (r1 as isize - r2 as isize) > 0 {
						return Err("The pawn cannot move to that position".to_string());
					}
				} else {
					if (r1 as isize - r2 as isize) < 0 {
						return Err("The pawn cannot move to that position".to_string());
					}
				}

				if c1 != c2 {
					if
						!(
							(r1 as isize - r2 as isize).abs() == 1 &&
							(c1 as isize - c2 as isize).abs() == 1 &&
							self.board.board[r2][c2].is_some()
						)
					{
						// the pawn is only allowed to move between columns
						// to capture another piece
						return Err("The pawn cannot move to that position".to_string());
					}
				} else {
					if
						(r1 as isize - r2 as isize).abs() != 1 &&
						!(
							(r1 as isize - r2 as isize).abs() == 2 &&
							self.board.board[r1][c1].unwrap().moved == false
						)
					{
						return Err("The pawn cannot move to that position".to_string());
					}
				}
			},
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

		self.moved_last = self.board.board[r2][c2];

		self.turn = match self.turn {
			Color::White => Color::Black,
			Color::Black => Color::White,
		};

		Ok(())
	}

	pub fn en_passant(
		&mut self,
		r1: usize, c1: usize,
		r2: usize, c2: usize
	) -> Result<(), String> {
		let pawn = self.board.board[r1][c1].unwrap();
		if pawn.name != Name::Pawn {
			return Err("Only pawns are allowed to perform en passant".to_string());
		}

		if pawn.color != self.turn {
			return Err("You can't move the opponent's piece".to_string());
		}

		if (r1 as isize - r2 as isize).abs() != 1 {
			return Err("Not a valid en passant".to_string());
		}

		let captured;

		if pawn.color == Color::White {
			if (r1 as isize - r2 as isize) > 0 {
				return Err("Not a valid en passant".to_string());
			}

			if (c1 as isize - c2 as isize) == 1 {
				// move to the right
				captured = self.board.board[r1][c1 - 1];
			} else if (c1 as isize - c2 as isize) == -1 {
				// move to the left
				captured = self.board.board[r1][c1 + 1];
			} else {
				return Err("Not a valid en passant".to_string());
			}
		} else {
			if (r1 as isize - r2 as isize) < 0 {
				return Err("Not a valid en passant".to_string());
			}

			if (c1 as isize - c2 as isize) == 1 {
				// move to the left
				captured = self.board.board[r1][c1 - 1];
			} else if (c1 as isize - c2 as isize) == -1 {
				// move to the right
				captured = self.board.board[r1][c1 + 1];
			} else {
				return Err("Not a valid en passant".to_string());
			}
		}

		if captured.is_none() {
			return Err("Not a valid en passant".to_string());
		} else if captured.unwrap() != self.moved_last.unwrap() {
			return Err("Not a valid en passant".to_string());
		} else {
			if
				captured.unwrap().color == self.turn ||
				captured.unwrap().name != Name::Pawn
			{
				return Err("Not a valid en passant".to_string());
			}

			if captured.unwrap().color == Color::White {
				if captured.unwrap().prev_square.r != 1 {
					return Err("Not a valid en passant".to_string());
				}
			} else {
				if captured.unwrap().prev_square.r != 6 {
					return Err("Not a valid en passant".to_string());
				}
			}
		}

		// there is no need to check that the square that the pawn is moving to
		// is empty, since the captured pawn wouldn't have been able to move
		// if it wasn't

		self.board.move_piece(r1, c1, r2, c2);

		self.board.capture_piece(captured.unwrap().curr_square.r, captured.unwrap().curr_square.c);

		self.moved_last = self.board.board[r2][c2];

		self.turn = match self.turn {
			Color::White => Color::Black,
			Color::Black => Color::White,
		};

		Ok(())
	}
}
