#[derive(Copy, Clone, PartialEq)]
pub struct File(pub usize);

impl File {
	pub fn as_char(&self) -> char {
		let File(num) = self;
		match num {
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

	pub fn as_usize(self) -> usize {
		let File(num) = self;
		num
	}

	pub fn as_isize(self) -> isize {
		let File(num) = self;
		num as isize
	}

	pub fn from_char(c: char) -> Result<File, ()> {
		match c {
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

	pub fn from_usize(u: usize) -> Result<File, ()> {
		if u >= 0 && u < 8 {
			return Ok(File(u));
		} else {
			return Err(());
		}
	}

	pub fn from_isize(i: isize) -> Result<File, ()> {
		let u = i as usize;
		return File::from_usize(u);
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
		let from_char = File::from_char('A');

		assert!(from_char.is_ok());

		let File(num) = from_char.unwrap();

		assert_eq!(num, 0);
	}

	#[test]
	fn test_from_char_invalid() {
		let from_char = File::from_char('I');

		assert!(from_char.is_err());
	}

	#[test]
	fn test_from_char_lowercase_invalid() {
		let from_char = File::from_char('a');

		assert!(from_char.is_err());
	}
}
