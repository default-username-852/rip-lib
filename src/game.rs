use crate::board::Board;
use crate::color::Color;
use crate::name::Name;

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
				if c1 != c2 {
					return Err("The pawn cannot move to that position".to_string());
				}

				if self.turn == Color::White {
					if (r1 as isize - r2 as isize) > 0 {
						return Err("The pawn cannot move to that position".to_string());
					}
				} else {
					if (r1 as isize - r2 as isize) < 0 {
						return Err("The pawn cannot move to that position".to_string());
					}
				}

				// if we have come this far, the pawn isn't trying to move to the side
				// nor is it trying to move backwards

				if
					(r1 as isize - r2 as isize).abs() != 1 &&
					!(
						(r1 as isize - r2 as isize).abs() == 2 &&
						self.board.board[r1][c1].unwrap().moved == false
					)
				{
					return Err("The pawn cannot move to that position".to_string());
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

		self.turn = match self.turn {
			Color::White => Color::Black,
			Color::Black => Color::White,
		};

		Ok(())
	}
}
