use crate::color::Color;
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
	pub fn new(color: Color, name: Name, r: usize, c: usize) -> Piece {
		Piece {
			color: color,
			name: name,
			moved: false,
			prev_square: Square::new(r, c),
			curr_square: Square::new(r, c),
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
