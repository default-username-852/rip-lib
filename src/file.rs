use std::convert::TryFrom;

#[derive(Copy, Clone, PartialEq)]
pub struct File(pub usize);

impl TryFrom<usize> for File {
	type Error = ();

	fn try_from(value: usize) -> Result<Self, Self::Error> {
		if value >= 0 && value < 8 {
			return Ok(File(value));
		}
		Err(())
	}
}

impl TryFrom<isize> for File {
	type Error = ();

	fn try_from(value: isize) -> Result<Self, Self::Error> {
		File::try_from(value as usize)
	}
}

impl TryFrom<char> for File {
	type Error = ();

	fn try_from(value: char) -> Result<Self, Self::Error> {
		match value {
			'A' => Ok(File(0)),
			'B' => Ok(File(1)),
			'C' => Ok(File(2)),
			'D' => Ok(File(3)),
			'E' => Ok(File(4)),
			'F' => Ok(File(5)),
			'G' => Ok(File(6)),
			'H' => Ok(File(7)),
			_ => Err(()),
		}
	}
}

impl From<File> for usize {
	fn from(file: File) -> Self {
		file.0
	}
}

impl From<File> for isize {
	fn from(file: File) -> Self {
		file.0 as isize
	}
}

impl From<File> for char {
	fn from(file: File) -> Self {
		match file.0 {
			0 => 'A',
			1 => 'B',
			2 => 'C',
			3 => 'D',
			4 => 'E',
			5 => 'F',
			6 => 'G',
			7 => 'H',
			_ => ' ',
		}
	}
}

impl PartialEq<usize> for File {
	fn eq(&self, other: &usize) -> bool {
		let File(num) = self;
		num == other
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_from_char_valid() {
		let from_char = File::try_from('A');

		assert!(from_char.is_ok());

		let File(num) = from_char.unwrap();

		assert_eq!(num, 0);
	}

	#[test]
	fn test_from_char_invalid() {
		let from_char = File::try_from('I');

		assert!(from_char.is_err());
	}

	#[test]
	fn test_from_char_lowercase_invalid() {
		let from_char = File::try_from('a');

		assert!(from_char.is_err());
	}
}
