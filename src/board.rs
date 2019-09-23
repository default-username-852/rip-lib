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
	pub(crate) board: Vec<Vec<Option<Piece>>>,
}

impl Board {
	pub fn new() -> Board {

		let width = 8;
		let height = 8;

		let mut board = Vec::with_capacity(height);
		for i in 0..height {
			board.push(Vec::with_capacity(width));
			for j in 0..width {
				board[i].push(None);
			}
		}

		Board::initialize(&mut board, height, width);

		Board {
			width: width,
			height: height,
			board: board,
		}
	}

	fn initialize(
		board: &mut Vec<Vec<Option<Piece>>>,
		height: usize,
		width: usize
	) {	
		let names = vec![Name::Rook, Name::Knight, Name::Bishop, Name::Queen,
			Name::King, Name::Bishop, Name::Knight, Name::Rook];

		for i in 0..width {
			board[1][i] = Some(Piece::new(
				Color::White, Name::Pawn, Square::new(Rank(1), File(i))
			));
			board[6][i] = Some(Piece::new(
				Color::Black, Name::Pawn, Square::new(Rank(6), File(i))
			));

			board[0][i] = Some(Piece::new(
				Color::White, names[i], Square::new(Rank(0), File(i))
			));
			board[7][i] = Some(Piece::new(
				Color::Black, names[i], Square::new(Rank(7), File(i))
			));
		}
	}

	pub fn move_piece(&mut self, from_square: Square, to_square: Square) {
		let mut piece = self.board
			[from_square.rank.as_usize()][from_square.file.as_usize()].unwrap();
		self.board
			[from_square.rank.as_usize()][from_square.file.as_usize()] = None;
		piece.prev_square = from_square;
		piece.curr_square = to_square;
		self.board
			[to_square.rank.as_usize()][to_square.file.as_usize()] = Some(piece);
	}

	pub fn capture_piece(&mut self, square: Square) {
		self.board[square.rank.as_usize()][square.file.as_usize()] = None;
	}

	pub fn print(&self) {
		println!(" ABCDEFGH");
		for i in (0..self.height).rev() {
			print!("{}", i + 1);
			for j in 0..self.width {
				if self.board[i][j].is_some() {
					print!("{}", self.board[i][j].unwrap());
				} else {
					print!(".");
				}
			}
			println!();
		}
	}

	pub fn legal_moves(&self, square: Square) -> Vec<Square> {
		let mut legal_moves = Vec::new();

		let piece = self.board
			[square.rank.as_usize()][square.file.as_usize()].unwrap();

		if piece.special_capture_move() {
			legal_moves.extend_from_slice(
				&self.calculate_legal_moves(square,	piece.moves(), false)
			);

			legal_moves.extend_from_slice(
				&self.calculate_legal_moves(square, piece.capture_moves(), true)
			);
		} else {
			legal_moves.extend_from_slice(
				&self.calculate_legal_moves(square, piece.moves(), false)
			);
		}

		return legal_moves;
	}

	// if the piece chosen piece doesn't have special capture moves,
	// capture_moves should be set to false
	fn calculate_legal_moves(
		&self,
		square: Square,
		moves: Vec<Vec<Direction>>,
		capture_moves: bool
	) -> Vec<Square> {
		let mut legal_moves = Vec::new();
		let piece = self.board
			[square.rank.as_usize()][square.file.as_usize()].unwrap();
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

					let square = Square::new(rank, file);

					let curr_piece =
						self.board[curr_rank as usize][curr_file as usize];

					if j != moves[i].len() - 1 {
						if piece.can_jump() || curr_piece.is_none() {
							continue;
						} else {
							break;
						}
					} else {
						let mut capture = false;

						if curr_piece.is_none() {
							if capture_moves {
								break 'repetetive;
							}
						} else {
							if !capture_moves && piece.special_capture_move() {
								break;
							}

							let captured_piece = curr_piece.unwrap();

							if captured_piece.color == piece.color {
								break 'repetetive;
							} else {
								capture = true;
							}
						}

						legal_moves.push(square);

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
						curr_rank += capture_moves[i][j].backwards().delta_y()
							* direction_change;
						curr_file += capture_moves[i][j].backwards().delta_x()
							* direction_change;

						let file = match File::from_isize(curr_file) {
							Ok(f) => f,
							Err(_) => break 'repetetive,
						};

						let rank = match Rank::from_isize(curr_rank) {
							Ok(r) => r,
							Err(_) => break 'repetetive,
						};

						let square = Square::new(rank, file);

						let curr_piece =
							self.board[curr_rank as usize][curr_file as usize];

						if j != capture_moves[i].len() - 1 {
							if
								piece.can_jump() ||
								curr_piece.is_none()
							{
								continue;
							} else {
								break;
							}
						} else {
							if curr_piece.is_some() {
								let capturing_piece = curr_piece.unwrap();
								if
									capturing_piece.color != color &&
									capturing_piece.name == *name
								{
									squares.push(square);
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
