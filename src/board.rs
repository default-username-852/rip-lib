use crate::piece::Piece;
use crate::color::Color;
use crate::name::Name;
use crate::square::Square;

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
		board[0][0] = Some(Piece::new(Color::White, Name::Rook, 0, 0));
		board[0][1] = Some(Piece::new(Color::White, Name::Knight, 0, 1));
		board[0][2] = Some(Piece::new(Color::White, Name::Bishop, 0, 2));
		board[0][3] = Some(Piece::new(Color::White, Name::Queen, 0, 3));
		board[0][4] = Some(Piece::new(Color::White, Name::King, 0, 4));
		board[0][5] = Some(Piece::new(Color::White, Name::Bishop, 0, 5));
		board[0][6] = Some(Piece::new(Color::White, Name::Knight, 0, 6));
		board[0][7] = Some(Piece::new(Color::White, Name::Rook, 0, 7));

		for i in 0..width {
			board[1][i] = Some(Piece::new(Color::White, Name::Pawn, 1, i));
			board[6][i] = Some(Piece::new(Color::Black, Name::Pawn, 6, i));
		}

		board[7][0] = Some(Piece::new(Color::Black, Name::Rook, 7, 0));
		board[7][1] = Some(Piece::new(Color::Black, Name::Knight, 7, 1));
		board[7][2] = Some(Piece::new(Color::Black, Name::Bishop, 7, 2));
		board[7][3] = Some(Piece::new(Color::Black, Name::Queen, 7, 3));
		board[7][4] = Some(Piece::new(Color::Black, Name::King, 7, 4));
		board[7][5] = Some(Piece::new(Color::Black, Name::Bishop, 7, 5));
		board[7][6] = Some(Piece::new(Color::Black, Name::Knight, 7, 6));
		board[7][7] = Some(Piece::new(Color::Black, Name::Rook, 7, 7));
	}

	pub fn move_piece(&mut self, r1: usize, c1: usize, r2: usize, c2: usize) {
		let mut piece = self.board[r1][c1].unwrap();
		self.board[r1][c1] = None;
		piece.prev_square = Square::new(r1, c1);
		piece.curr_square = Square::new(r2, c2);
		self.board[r2][c2] = Some(piece);
	}

	pub fn capture_piece(&mut self, r: usize, c: usize) {
		self.board[r][c] = None;
	}

	pub fn print(&self) {
		println!(" ABCDEFGH");
		for i in (0..self.height).rev() {
			print!("{}", i + 1);
			for j in 0..self.width {
				match self.board[i][j] {
					Some(piece) => {
						match piece.name {
							Name::King => print!("K"),
							Name::Queen => print!("Q"),
							Name::Rook => print!("R"),
							Name::Bishop => print!("B"),
							Name::Knight => print!("N"),
							Name::Pawn => print!("P"),
						}
					},
					None => print!("."),
				}
			}
			println!();
		}
	}
}
