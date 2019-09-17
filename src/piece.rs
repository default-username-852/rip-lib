use crate::color::Color;
use crate::direction::Direction;
use crate::name::Name;
use crate::square::Square;

use std::fmt;

#[derive(Copy, Clone, PartialEq)]
pub struct Piece {
	pub(crate) color: Color,
	pub(crate) name: Name,
	pub(crate) moved: bool,
	pub(crate) prev_square: Square,
	pub(crate) curr_square: Square,
}

impl Piece {
	pub fn new(color: Color, name: Name, square: Square) -> Piece {
		Piece {
			color: color,
			name: name,
			moved: false,
			prev_square: square,
			curr_square: square,
		}
	}

	pub fn simple(name: Name) -> Piece {
		let square = Square::from_str("A1").unwrap();
		Piece {
			color: Color::White,
			name: name,
			moved: false,
			prev_square: square,
			curr_square: square,
		}
	}

	pub fn moves(&self) -> Vec<Vec<Direction>> {
		match self.name {
			Name::King | Name::Queen =>
				vec![
					vec![Direction::Up],
					vec![Direction::Down],
					vec![Direction::Left],
					vec![Direction::Right],
					vec![Direction::UpLeft],
					vec![Direction::UpRight],
					vec![Direction::DownLeft],
					vec![Direction::DownRight]
				],
			Name::Rook =>
				vec![
					vec![Direction::Up],
					vec![Direction::Down],
					vec![Direction::Left],
					vec![Direction::Right]
				],
			Name::Bishop =>
				vec![
					vec![Direction::UpLeft],
					vec![Direction::UpRight],
					vec![Direction::DownLeft],
					vec![Direction::DownRight]
				],
			Name::Knight =>
				vec![
					vec![Direction::UpLeft, Direction::Up],
					vec![Direction::UpLeft, Direction::Left],
					vec![Direction::UpRight, Direction::Up],
					vec![Direction::UpRight, Direction::Right],
					vec![Direction::DownLeft, Direction::Down],
					vec![Direction::DownLeft, Direction::Left],
					vec![Direction::DownRight, Direction::Down],
					vec![Direction::DownRight, Direction::Right]
				],
			Name::Pawn => {
				if self.moved {
					vec![
						vec![Direction::Up]
					]
				} else {
					vec![
						vec![Direction::Up],
						vec![Direction::Up, Direction::Up]
					]
				}
			},
		}
	}

	pub fn capture_moves(&self) -> Vec<Vec<Direction>> {
		match self.name {
			Name::Pawn => vec![vec![Direction::UpLeft], vec![Direction::UpRight]],
			_  => self.moves(),
		}
	}

	pub fn special_capture_move(&self) -> bool {
		match self.name {
			Name::Pawn => true,
			_ => false,
		}
	}

	pub fn repetetive_moves(&self) -> bool {
		match self.name {
			Name::King | Name::Knight | Name::Pawn => false,
			Name::Queen | Name::Rook | Name::Bishop => true,
		}
	}

	pub fn can_jump(&self) -> bool {
		match self.name {
			Name::Knight => true,
			_ => false,
		}
	}
}

impl fmt::Display for Piece {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self.name {
			Name::King => write!(f, "K"),
			Name::Queen => write!(f, "Q"),
			Name::Rook => write!(f, "R"),
			Name::Bishop => write!(f, "B"),
			Name::Knight => write!(f, "N"),
			Name::Pawn => write!(f, "P"),
		}
	}
}
