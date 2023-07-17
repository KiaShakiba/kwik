/*
 * Copyright (c) Kia Shakiba
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

use std::fmt::Display;
use crate::table::cell::{Cell, Align, Style};

#[derive(Default)]
pub struct Row {
	cells: Vec<Cell>,
	max_len: usize,
}

impl Row {
	pub fn is_empty(&self) -> bool {
		self.cells.is_empty()
	}

	pub fn len(&self) -> usize {
		self.cells.len()
	}

	pub fn push<T>(
		mut self,
		value: T,
		align: Align,
		style: Style,
	) -> Self
	where
		T: 'static + Display,
	{
		let string = value.to_string();
		let len = string.len();
		let cell = Cell::new(string, align, style);

		if len > self.max_len {
			self.max_len = len;
		}

		self.cells.push(cell);
		self
	}

	pub fn blank(self) -> Self {
		self.push("", Align::Left, Style::Normal)
	}

	pub fn size(&self) -> usize {
		self.to_string(None, true).len()
	}

	pub fn get_column_size(&self, index: usize) -> usize {
		if index >= self.cells.len() {
			panic!("Invalid column index.");
		}

		self.cells[index].size()
	}

	pub fn print(&self, sizes: &Vec<usize>, spaced: bool) {
		println!("{}", self.to_string(Some(sizes), spaced));
	}

	fn to_string(
		&self,
		sizes: Option<&Vec<usize>>,
		spaced: bool
	) -> String {
		let join_str = if spaced { " | " } else { "|" };

		let line = self.cells
			.iter()
			.enumerate()
			.map(|(index, cell)| {
				let size = match sizes {
					Some(sizes) => sizes[index],
					None => cell.size(),
				};

				cell.to_sized_string(size)
			})
			.collect::<Vec<String>>()
			.join(join_str);

		if spaced {
			format!("| {line} |")
		} else {
			format!("|{line}|")
		}
	}
}
