use crate::color::Color;
use crate::name::Name;

#[derive(Copy, Clone)]
pub struct Piece {
	color: Color,
	pub(crate) name: Name,
	moved: bool,
}

impl Piece {
	pub fn new(color: Color, name: Name) -> Piece {
		Piece {
			color: color,
			name: name,
			moved: false,
		}
	}
}
