/*
 * Copyright (c) Kia Shakiba
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

pub struct Cell {
	value: String,
	align: Align,
	style: Style,
}

pub enum Align {
	Left,
	Right,
	Center,
}

pub enum Style {
	Bold,
	Normal,
}

impl Cell {
	pub fn new(
		value: String,
		align: Align,
		style: Style,
	) -> Self {
		Cell {
			value,
			align,
			style,
		}
	}

	pub fn size(&self) -> usize {
		self.value.len()
	}

	pub fn to_sized_string(&self, size: usize) -> String {
		let string = match &self.align {
			Align::Left => format!("{:<size$}", self.value),
			Align::Right => format!("{:>size$}", self.value),

			Align::Center => {
				let before = (size as f64 - self.value.len() as f64) / 2.0;
				let after = (size as f64 - self.value.len() as f64) / 2.0;

				format!(
					"{:before$}{}{:after$}", "", self.value, "",
					before = before.floor() as usize,
					after = after.ceil() as usize,
				)
			},
		};

		match &self.style {
			Style::Bold => format!("\x1B[1m{}\x1B[0m", string),
			Style::Normal => string,
		}
	}
}
