use crate::piece::Piece;
use crate::color::Color;
use crate::name::Name;

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
		board[0][0] = Some(Piece::new(Color::White, Name::Rook));
		board[0][1] = Some(Piece::new(Color::White, Name::Knight));
		board[0][2] = Some(Piece::new(Color::White, Name::Bishop));
		board[0][3] = Some(Piece::new(Color::White, Name::King));
		board[0][4] = Some(Piece::new(Color::White, Name::Queen));
		board[0][5] = Some(Piece::new(Color::White, Name::Bishop));
		board[0][6] = Some(Piece::new(Color::White, Name::Knight));
		board[0][7] = Some(Piece::new(Color::White, Name::Rook));

		for i in 0..width {
			board[1][i] = Some(Piece::new(Color::White, Name::Pawn));
			board[6][i] = Some(Piece::new(Color::Black, Name::Pawn));
		}

		board[7][0] = Some(Piece::new(Color::Black, Name::Rook));
		board[7][1] = Some(Piece::new(Color::Black, Name::Knight));
		board[7][2] = Some(Piece::new(Color::Black, Name::Bishop));
		board[7][3] = Some(Piece::new(Color::Black, Name::King));
		board[7][4] = Some(Piece::new(Color::Black, Name::Queen));
		board[7][5] = Some(Piece::new(Color::Black, Name::Bishop));
		board[7][6] = Some(Piece::new(Color::Black, Name::Knight));
		board[7][7] = Some(Piece::new(Color::Black, Name::Rook));
	}

	pub fn move_piece(&mut self, r1: usize, c1: usize, r2: usize, c2: usize) {
		let piece = self.board[r1][c1];
		self.board[r1][c1] = None;
		self.board[r2][c2] = piece;
	}

	pub fn print(&self) {
		println!(" ABCDEFGH");
		for i in 0..self.height {
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
