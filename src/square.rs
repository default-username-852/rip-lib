use crate::rank::Rank;
use crate::file::File;

#[derive(Copy, Clone, PartialEq)]
pub struct Square {
	pub(crate) file: File,
	pub(crate) rank: Rank,
}

impl Square {
	pub fn new(file: File, rank: Rank) -> Square {
		Square {
			file: file,
			rank: rank,
		}
	}
}

impl std::fmt::Debug for Square {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}{}", self.file.as_char(), self.rank.as_char())
	}
}
