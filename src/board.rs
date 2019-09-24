use crate::piece::Piece;
use crate::color::Color;
use crate::name::Name;
use crate::square::Square;
use crate::file::File;
use crate::rank::Rank;
use crate::direction::Direction;

pub struct Board {
	height: usize,
	width: usize,
	pub(crate) board: Vec<Vec<Square>>,
}

impl Board {
	pub fn new() -> Board {
		let width = 8;
		let height = 8;

		let mut board = Vec::with_capacity(height);
		for i in 0..height {
			board.push(Vec::with_capacity(width));
			for j in 0..width {
				board[i].push(Square::new(Rank(i), File(j)));
			}
		}

		Board::initialize(&mut board, height, width);

		Board {
			width: width,
			height: height,
			board: board,
		}
	}

	fn initialize(board: &mut Vec<Vec<Square>>, height: usize, width: usize) {
		let names = vec![
			Name::Rook,
			Name::Knight,
			Name::Bishop,
			Name::Queen,
			Name::King,
			Name::Bishop,
			Name::Knight,
			Name::Rook,
		];

		for i in 0..width {
			board[1][i].piece = Some(Piece::new(Color::White, Name::Pawn));
			board[6][i].piece = Some(Piece::new(Color::Black, Name::Pawn));

			board[0][i].piece = Some(Piece::new(Color::White, names[i]));
			board[7][i].piece = Some(Piece::new(Color::Black, names[i]));
		}
	}

	pub fn get(&self, square: Square) -> &Square {
		&self.board[square.rank.as_usize()][square.file.as_usize()]
	}

	pub fn get_mut(&mut self, square: Square) -> &mut Square {
		&mut self.board[square.rank.as_usize()][square.file.as_usize()]
	}

	pub fn move_piece(&mut self, from_square: Square, to_square: Square) {
		let from_square = &mut self.get_mut(from_square);
		let piece = from_square.piece.take();
		let to_square = &mut self.get_mut(to_square);

		to_square.piece = piece;
	}

	pub fn capture_piece(&mut self, square: Square) {
		let square = self.get_mut(square);
		square.piece = None;
	}

	pub fn print(&self) {
		println!(" ABCDEFGH");
		for i in (0..self.height).rev() {
			print!("{}", i + 1);
			for j in 0..self.width {
				let square = self.get(Square::new(Rank(i), File(j)));
				if !square.is_empty() {
					print!("{}", square.piece.unwrap());
				} else {
					print!(".");
				}
			}
			println!();
		}
	}

	pub fn legal_moves(&self, square: Square) -> Vec<Square> {
		let square = self.get(square);
		let mut legal_moves = Vec::new();

		let piece = square.piece.unwrap();

		if piece.special_capture_move() {
			legal_moves.extend_from_slice(&self.calculate_legal_moves(
				square,
				piece.moves(),
				false,
			));

			legal_moves.extend_from_slice(&self.calculate_legal_moves(
				square,
				piece.capture_moves(),
				true,
			));
		} else {
			legal_moves.extend_from_slice(&self.calculate_legal_moves(
				square,
				piece.moves(),
				false,
			));
		}

		return legal_moves;
	}

	// if the piece chosen piece doesn't have special capture moves,
	// capture_moves should be set to false
	fn calculate_legal_moves(
		&self,
		square: &Square,
		moves: Vec<Vec<Direction>>,
		capture_moves: bool,
	) -> Vec<Square> {
		let mut legal_moves = Vec::new();
		let piece = square.piece.unwrap();
		let direction_change = match piece.color {
			Color::White => 1,
			Color::Black => -1,
		};

		for i in 0..moves.len() {
			let mut curr_rank: isize = square.rank.as_isize();
			let mut curr_file: isize = square.file.as_isize();

			'repetetive: loop {
				for j in 0..moves[i].len() {
					curr_rank += moves[i][j].delta_y() * direction_change;
					curr_file += moves[i][j].delta_x() * direction_change;

					let file = match File::from_isize(curr_file) {
						Ok(f) => f,
						Err(_) => break 'repetetive,
					};

					let rank = match Rank::from_isize(curr_rank) {
						Ok(r) => r,
						Err(_) => break 'repetetive,
					};

					let square = self.get(Square::new(rank, file));

					if j != moves[i].len() - 1 {
						if square.is_empty() || piece.can_jump() {
							continue;
						} else {
							break;
						}
					} else {
						let mut capture = false;

						if square.is_empty() {
							if capture_moves {
								break 'repetetive;
							}
						} else {
							if !capture_moves && piece.special_capture_move() {
								break;
							}

							let captured_piece = square.piece.unwrap();

							if captured_piece.color == piece.color {
								break 'repetetive;
							} else {
								capture = true;
							}
						}

						legal_moves.push(*square);

						if capture {
							break 'repetetive;
						}
					}
				}

				if !piece.repetetive_moves() {
					break;
				}
			}
		}

		return legal_moves;
	}

	pub fn can_capture(&self, square: Square, color: Color) -> Vec<Square> {
		let mut squares = Vec::new();

		// since we will be checking the moves of the oposite color,
		// we have to change the sign of the direction
		let direction_change = match color {
			Color::White => -1,
			Color::Black => 1,
		};

		for name in &Name::all() {
			let piece = Piece::simple(*name);
			let capture_moves = piece.capture_moves();

			for i in 0..capture_moves.len() {
				let mut curr_rank: isize = square.rank.as_isize();
				let mut curr_file: isize = square.file.as_isize();

				'repetetive: loop {
					for j in 0..capture_moves[i].len() {
						curr_rank +=
							capture_moves[i][j].backwards().delta_y() * direction_change;
						curr_file +=
							capture_moves[i][j].backwards().delta_x() * direction_change;

						let file = match File::from_isize(curr_file) {
							Ok(f) => f,
							Err(_) => break 'repetetive,
						};

						let rank = match Rank::from_isize(curr_rank) {
							Ok(r) => r,
							Err(_) => break 'repetetive,
						};

						let square = self.get(Square::new(rank, file));

						if j != capture_moves[i].len() - 1 {
							if piece.can_jump() || square.is_empty() {
								continue;
							} else {
								break;
							}
						} else {
							if !square.is_empty() {
								let capturing_piece = square.piece.unwrap();
								if capturing_piece.color != color &&
									capturing_piece.name == *name
								{
									squares.push(*square);
								}

								break 'repetetive;
							}
						}
					}

					if !piece.repetetive_moves() {
						break;
					}
				}
			}
		}

		return squares;
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_move_piece() {
		let mut board = Board::new();

		let from = Square::from_str("A2").unwrap();
		let to = Square::from_str("A3").unwrap();

		board.move_piece(from, to);

		assert!(board.get(from).is_empty());
		assert!(!board.get(to).is_empty());
	}
}
