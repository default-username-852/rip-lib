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
