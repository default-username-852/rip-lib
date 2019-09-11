use crate::board::Board;

pub struct Game {
	board: Board,
}

impl Game {
	pub fn new() -> Game {
		let board = Board::new();

		Game {
			board: board,
		}
	}

	pub fn print_board(&self) {
		self.board.print();
	}
}
