use crate::board::Board;
use crate::color::Color;
use crate::name::Name;
use crate::piece::Piece;
use crate::square::Square;

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
		from_square: Square,
		to_square: Square,
	) -> Result<(), String> {
		let from_rank = from_square.rank.as_usize();
		let from_file = from_square.file.as_usize();

		let to_rank = to_square.rank.as_usize();
		let to_file = to_square.file.as_usize();

		if self.board.get(from_square).is_empty() {
			return Err("There is no piece to move".to_string());
		}

		if self.board.get(from_square).piece.unwrap().color != self.turn {
			return Err("You can't move the opponent's piece".to_string());
		}

		// check that the piece is allowed to go there
		match self.board.get(from_square).piece.unwrap().name {
			Name::King => {
				if (from_rank as isize - to_rank as isize).abs() != 1 &&
					(from_file as isize - to_file as isize).abs() != 1
				{
					return Err("The king cannot move to that position".to_string());
				}
			},
			Name::Queen => {
				if from_rank != to_rank &&
					from_file != to_file &&
					(from_rank as isize - to_rank as isize).abs() !=
						(from_file as isize - to_file as isize).abs()
				{
					return Err("The queen cannot move to that position".to_string());
				}
			},
			Name::Rook => {
				if from_rank != to_rank && from_file != to_file {
					return Err("The rook cannot move to that position".to_string());
				}
			},
			Name::Bishop => {
				if (from_rank as isize - to_rank as isize).abs() !=
					(from_file as isize - to_file as isize).abs()
				{
					return Err("The bishop cannot move to that position".to_string());
				}
			},
			Name::Knight => {
				// the Pythagorean theorem tells us that the knight can only move
				// to positions within sqrt(5) of its original position
				if (from_rank as isize - to_rank as isize).pow(2) +
					(from_file as isize - to_file as isize).pow(2) !=
					5
				{
					return Err("The knight cannot move to that position".to_string());
				}
			},
			Name::Pawn => {
				// check if the pawn is trying to move backwards
				if self.turn == Color::White {
					if (from_rank as isize - to_rank as isize) > 0 {
						return Err("The pawn cannot move to that position".to_string());
					}
				} else {
					if (from_rank as isize - to_rank as isize) < 0 {
						return Err("The pawn cannot move to that position".to_string());
					}
				}

				if from_file != to_file {
					if !((from_rank as isize - to_rank as isize).abs() == 1 &&
						(from_file as isize - to_file as isize).abs() == 1 &&
						!self.board.get(to_square).is_empty())
					{
						// the pawn is only allowed to move between columns
						// to capture another piece
						return Err("The pawn cannot move to that position".to_string());
					}
				} else {
					if (from_rank as isize - to_rank as isize).abs() != 1 &&
						!((from_rank as isize - to_rank as isize).abs() == 2 &&
							self.board.get(from_square).piece.unwrap().moved == false)
					{
						return Err("The pawn cannot move to that position".to_string());
					}
				}
			},
		}

		if !self.board.get(to_square).is_empty() {
			if self.board.get(to_square).piece.unwrap().color == self.turn {
				return Err("You can't take your own piece".to_string());
			} else {
				// take the opponent's piece
				self.board.get_mut(to_square).piece = None;
			}
		}

		self.board.move_piece(from_square, to_square);

		self.moved_last = self.board.get(to_square).piece;

		self.turn = match self.turn {
			Color::White => Color::Black,
			Color::Black => Color::White,
		};

		Ok(())
	}

	pub fn en_passant(
		&mut self,
		from_square: Square,
		to_square: Square,
	) -> Result<(), String> {
		let from_rank = from_square.rank.as_usize();
		let from_file = from_square.file.as_usize();

		let to_rank = to_square.rank.as_usize();
		let to_file = to_square.file.as_usize();

		let pawn = self.board.get(from_square).piece.unwrap();

		if pawn.name != Name::Pawn {
			return Err("Only pawns are allowed to perform en passant".to_string());
		}

		if pawn.color != self.turn {
			return Err("You can't move the opponent's piece".to_string());
		}

		if (from_rank as isize - to_rank as isize).abs() != 1 {
			return Err("Not a valid en passant".to_string());
		}

		let captured;

		if pawn.color == Color::White {
			if (from_rank as isize - to_rank as isize) > 0 {
				return Err("Not a valid en passant".to_string());
			}

			if (from_file as isize - to_file as isize) == 1 {
				// move to the right
				captured = self.board.board[from_rank][from_file - 1];
			} else if (from_file as isize - to_file as isize) == -1 {
				// move to the left
				captured = self.board.board[from_rank][from_file + 1];
			} else {
				return Err("Not a valid en passant".to_string());
			}
		} else {
			if (from_rank as isize - to_rank as isize) < 0 {
				return Err("Not a valid en passant".to_string());
			}

			if (from_file as isize - to_file as isize) == 1 {
				// move to the left
				captured = self.board.board[from_rank][from_file - 1];
			} else if (from_file as isize - to_file as isize) == -1 {
				// move to the right
				captured = self.board.board[from_rank][from_file + 1];
			} else {
				return Err("Not a valid en passant".to_string());
			}
		}

		if captured.is_empty() {
			return Err("Not a valid en passant".to_string());
		} else if captured.piece.unwrap() != self.moved_last.unwrap() {
			return Err("Not a valid en passant".to_string());
		} else {
			if captured.piece.unwrap().color == self.turn ||
				captured.piece.unwrap().name != Name::Pawn
			{
				return Err("Not a valid en passant".to_string());
			}
		}

		// there is no need to check that the square that the pawn is moving to
		// is empty, since the captured pawn wouldn't have been able to move
		// if it wasn't

		self.board.move_piece(from_square, to_square);

		self.board.capture_piece(captured);

		self.moved_last = self.board.get(to_square).piece;

		self.turn = match self.turn {
			Color::White => Color::Black,
			Color::Black => Color::White,
		};

		Ok(())
	}
}
