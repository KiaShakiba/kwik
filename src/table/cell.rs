pub struct Cell {
	value: String,
	direction: Direction,
	style: Style,
}

pub enum Direction {
	Left,
	Right,
}

pub enum Style {
	Bold,
	Normal,
}

impl Cell {
	pub fn new(
		value: String,
		direction: Direction,
		style: Style,
	) -> Self {
		Cell {
			value,
			direction,
			style,
		}
	}

	pub fn size(&self) -> usize {
		self.value.len()
	}

	pub fn to_sized_string(&self, size: usize) -> String {
		let string = match &self.direction {
			Direction::Left => format!("{:<size$}", self.value),
			Direction::Right => format!("{:>size$}", self.value),
		};

		match &self.style {
			Style::Bold => format!("\x1B[1m{}\x1B[0m", string),
			Style::Normal => string,
		}
	}
}
