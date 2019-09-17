#[derive(Copy, Clone, PartialEq)]
pub enum Name {
	King,
	Queen,
	Rook,
	Bishop,
	Knight,
	Pawn,
}

impl Name {
	pub fn all() -> Vec<Name> {
		return vec![
			Name::King,
			Name::Queen,
			Name::Rook,
			Name::Bishop,
			Name::Knight,
			Name::Pawn
		];
	}
}
