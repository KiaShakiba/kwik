/*
 * Copyright (c) Kia Shakiba
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

use std::{
	io::Write,
	fmt::Display,
};

use crate::table::cell::{Cell, Align, Style};

#[derive(Default)]
pub struct Row {
	cells: Vec<Cell>,
	max_len: usize,
}

impl Row {
	/// Returns true if there are no columns in the row.
	///
	/// # Examples
	/// ```
	/// use kwik::{TableRow, TableRowAlign, TableRowStyle};
	///
	/// let mut row = TableRow::default();
	///
	/// assert!(row.is_empty());
	///
	/// row = row.push("Row 1", TableRowAlign::Left, TableRowStyle::Normal);
	///
	/// assert!(!row.is_empty());
	/// ```
	pub fn is_empty(&self) -> bool {
		self.cells.is_empty()
	}

	/// Returns the number of columns in the row.
	///
	/// # Examples
	/// ```
	/// use kwik::{TableRow, TableRowAlign, TableRowStyle};
	///
	/// let mut row = TableRow::default()
	///     .push("Row 1", TableRowAlign::Left, TableRowStyle::Normal);
	///
	/// assert_eq!(row.len(), 1);
	/// ```
	pub fn len(&self) -> usize {
		self.cells.len()
	}

	/// Adds a new column to the end of the row.
	///
	/// # Examples
	/// ```
	/// use kwik::{TableRow, TableRowAlign, TableRowStyle};
	///
	/// let mut row = TableRow::default()
	///     .push("Row 1", TableRowAlign::Left, TableRowStyle::Normal);
	/// ```
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

	/// Adds a blank column to the end of the row.
	///
	/// # Examples
	/// ```
	/// use kwik::{TableRow, TableRowAlign, TableRowStyle};
	///
	/// let mut row = TableRow::default()
	///     .blank();
	/// ```
	pub fn blank(self) -> Self {
		self.push("", Align::Left, Style::Normal)
	}

	/// Returns the printed size of the row.
	pub fn size(&self) -> usize {
		self.to_string(None, true).len()
	}

	/// Returns the printed size of the column at the supplied index.
	pub fn get_column_size(&self, index: usize) -> usize {
		if index >= self.cells.len() {
			panic!("Invalid column index.");
		}

		self.cells[index].size()
	}

	/// Prints the column to the supplied stream.
	pub fn print(
		&self,
		stdout: &mut impl Write,
		sizes: &Vec<usize>,
		spaced: bool
	) {
		writeln!(stdout, "{}", self.to_string(Some(sizes), spaced)).unwrap();
	}

	/// Returns the string value of the row.
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
