#[derive(Copy, Clone, PartialEq)]
pub struct Square {
	pub(crate) r: usize,
	pub(crate) c: usize,
}

impl Square {
	pub fn new(r: usize, c: usize) -> Square {
		Square {
			r: r,
			c: c,
		}
	}
}
