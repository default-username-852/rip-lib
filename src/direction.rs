pub enum Direction {
	Up,
	Down,
	Left,
	Right,
	UpLeft,
	UpRight,
	DownLeft,
	DownRight,
}

impl Direction {
	pub fn delta_x(&self) -> isize {
		match self {
			Direction::Up => 0,
			Direction::Down => 0,
			Direction::Left => -1,
			Direction::Right => 1,
			Direction::UpLeft => -1,
			Direction::UpRight => 1,
			Direction::DownLeft => -1,
			Direction::DownRight => 1,
		}
	}

	pub fn delta_y(&self) -> isize {
		match self {
			Direction::Up => 1,
			Direction::Down => -1,
			Direction::Left => 0,
			Direction::Right => 0,
			Direction::UpLeft => 1,
			Direction::UpRight => 1,
			Direction::DownLeft => -1,
			Direction::DownRight => -1,
		}
	}

	pub fn backwards(&self) -> Direction {
		match self {
			Direction::Up => Direction::Down,
			Direction::Down => Direction::Up,
			Direction::Left => Direction::Right,
			Direction::Right => Direction::Left,
			Direction::UpLeft => Direction::DownRight,
			Direction::UpRight => Direction::DownLeft,
			Direction::DownLeft => Direction::UpRight,
			Direction::DownRight => Direction::UpLeft,
		}
	}
}
