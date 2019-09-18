use crate::piece::Piece;
use crate::color::Color;
use crate::name::Name;
use crate::square::Square;
use crate::file::File;
use crate::rank::Rank;

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
		board[0][0] = Some(Piece::new(
			Color::White, Name::Rook, Square::new(File(0), Rank(0))
		));
		board[0][1] = Some(Piece::new(
			Color::White, Name::Knight, Square::new(File(1), Rank(0))
		));
		board[0][2] = Some(Piece::new(
			Color::White, Name::Bishop, Square::new(File(2), Rank(0))
		));
		board[0][3] = Some(Piece::new(
			Color::White, Name::Queen, Square::new(File(3), Rank(0))
		));
		board[0][4] = Some(Piece::new(
			Color::White, Name::King, Square::new(File(4), Rank(0))
		));
		board[0][5] = Some(Piece::new(
			Color::White, Name::Bishop, Square::new(File(5), Rank(0))
		));
		board[0][6] = Some(Piece::new(
			Color::White, Name::Knight, Square::new(File(6), Rank(0))
		));
		board[0][7] = Some(Piece::new(
			Color::White, Name::Rook, Square::new(File(7), Rank(0))
		));

		for i in 0..width {
			board[1][i] = Some(Piece::new(
				Color::White, Name::Pawn, Square::new(File(i), Rank(1))
			));
			board[6][i] = Some(Piece::new(
				Color::Black, Name::Pawn, Square::new(File(i), Rank(6))
			));
		}

		board[7][0] = Some(Piece::new(
			Color::Black, Name::Rook, Square::new(File(0), Rank(7))
		));
		board[7][1] = Some(Piece::new(
			Color::Black, Name::Knight, Square::new(File(1), Rank(7))
		));
		board[7][2] = Some(Piece::new(
			Color::Black, Name::Bishop, Square::new(File(2), Rank(7))
		));
		board[7][3] = Some(Piece::new(
			Color::Black, Name::Queen, Square::new(File(3), Rank(7))
		));
		board[7][4] = Some(Piece::new(
			Color::Black, Name::King, Square::new(File(4), Rank(7))
		));
		board[7][5] = Some(Piece::new(
			Color::Black, Name::Bishop, Square::new(File(5), Rank(7))
		));
		board[7][6] = Some(Piece::new(
			Color::Black, Name::Knight, Square::new(File(6), Rank(7))
		));
		board[7][7] = Some(Piece::new(
			Color::Black, Name::Rook, Square::new(File(7), Rank(7))
		));
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

	pub fn can_capture(&self, square: Square, color: Color) -> Vec<Square> {
		let mut squares = Vec::new();

		let direction_change = match color {
			Color::White => 1,
			Color::Black => -1,
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

						let square = Square::new(file, rank);

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
