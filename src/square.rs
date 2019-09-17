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
