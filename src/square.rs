use crate::rank::Rank;
use crate::file::File;
use crate::piece::Piece;
use std::convert::TryFrom;

#[derive(Copy, Clone, PartialEq)]
pub struct Square {
	pub(crate) rank: Rank,
	pub(crate) file: File,
	pub(crate) piece: Option<Piece>,
}

impl Square {
	pub fn new(rank: Rank, file: File) -> Square {
		Square {
			rank: rank,
			file: file,
			piece: None,
		}
	}

	pub fn from_str(square: &str) -> Result<Square, ()> {
		if square.len() > 2 {
			return Err(());
		}

		let rank = Rank::try_from(square.chars().nth(1).unwrap())?;
		let file = File::try_from(square.chars().nth(0).unwrap())?;

		Ok(Square {
			rank: rank,
			file: file,
			piece: None,
		})
	}

	pub fn is_empty(&self) -> bool {
		self.piece.is_none()
	}
}

impl std::fmt::Debug for Square {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}{}", char::from(self.file), char::from(self.rank))
	}
}
