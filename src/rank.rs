use std::convert::TryFrom;

#[derive(Copy, Clone, PartialEq)]
pub struct Rank(pub usize);

impl TryFrom<usize> for Rank {
	type Error = ();

	fn try_from(value: usize) -> Result<Self, Self::Error> {
		if value >= 0 && value < 8 {
			return Ok(Rank(value));
		}
		Err(())
	}
}

impl TryFrom<isize> for Rank {
	type Error = ();

	fn try_from(value: isize) -> Result<Self, Self::Error> {
		Rank::try_from(value as usize)
	}
}

impl TryFrom<char> for Rank {
	type Error = ();

	fn try_from(value: char) -> Result<Self, Self::Error> {
		match value {
			'1' => Ok(Rank(0)),
			'2' => Ok(Rank(1)),
			'3' => Ok(Rank(2)),
			'4' => Ok(Rank(3)),
			'5' => Ok(Rank(4)),
			'6' => Ok(Rank(5)),
			'7' => Ok(Rank(6)),
			'8' => Ok(Rank(7)),
			_ => Err(()),
		}
	}
}

impl From<Rank> for usize {
	fn from(rank: Rank) -> Self {
		rank.0
	}
}

impl From<Rank> for isize {
	fn from(rank: Rank) -> Self {
		rank.0 as isize
	}
}

impl From<Rank> for char {
	fn from(rank: Rank) -> Self {
		match rank.0 {
			0 => '1',
			1 => '2',
			2 => '3',
			3 => '4',
			4 => '5',
			5 => '6',
			6 => '7',
			7 => '8',
			_ => ' ',
		}
	}
}

impl PartialEq<usize> for Rank {
	fn eq(&self, other: &usize) -> bool {
		let Rank(num) = self;
		num == other
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_from_char_valid() {
		let from_char = Rank::try_from('1');

		assert!(from_char.is_ok());

		let Rank(num) = from_char.unwrap();

		assert_eq!(num, 0);
	}

	#[test]
	fn test_from_char_invalid() {
		let from_char = Rank::try_from('9');

		assert!(from_char.is_err());
	}
}
