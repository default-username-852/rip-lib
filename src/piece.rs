use crate::color::Color;
use crate::name::Name;
use crate::square::Square;

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
