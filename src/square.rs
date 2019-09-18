use crate::rank::Rank;
use crate::file::File;

#[derive(Copy, Clone, PartialEq)]
pub struct Square {
	pub(crate) rank: Rank,
	pub(crate) file: File,
}

impl Square {
	pub fn new(rank: Rank, file: File) -> Square {
		Square {
			rank: rank,
			file: file,
		}
	}

	pub fn from_str(square: &str) -> Result<Square, ()> {
		if square.len() > 2 {
			return Err(());
		}

		let rank = Rank::from_char(square.chars().nth(1).unwrap())?;
		let file = File::from_char(square.chars().nth(0).unwrap())?;

		Ok(Square {
			rank: rank,
			file: file,
		})
	}
}

impl std::fmt::Debug for Square {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}{}", self.file.as_char(), self.rank.as_char())
	}
}
