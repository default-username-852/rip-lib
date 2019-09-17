#[derive(Copy, Clone, PartialEq)]
pub struct Rank(pub usize);

impl Rank {
	pub fn as_char(&self) -> char {
		let Rank(num) = self;
		match num {
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

	pub fn as_usize(self) -> usize {
		let Rank(num) = self;
		num
	}

	pub fn as_isize(self) -> isize {
		let Rank(num) = self;
		num as isize
	}

	pub fn from_char(c: char) -> Result<Rank, ()> {
		match c {
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

	pub fn from_usize(u: usize) -> Result<Rank, ()> {
		if u >= 0 && u < 8 {
			return Ok(Rank(u));
		} else {
			return Err(());
		}
	}

	pub fn from_isize(i: isize) -> Result<Rank, ()> {
		let u = i as usize;
		return Rank::from_usize(u);
	}
}

impl PartialEq<usize> for Rank {
	fn eq(&self, other: &usize) -> bool {
		let Rank(num) = self;
		num == other
	}
}
