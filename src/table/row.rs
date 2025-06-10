/*
 * Copyright (c) Kia Shakiba
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

use std::{
	io::{self, Write},
	fmt::Display,
};

use crate::{
	file::csv::{WriteRow, RowData},
	table::cell::{
		Cell,
		Align,
		Style,
	},
};

#[derive(Default)]
pub struct Row {
	cells: Vec<Cell>,
	max_len: usize,
}

#[derive(PartialEq)]
pub enum ColumnJoinType {
	Normal,
	Spaced,
	Plus,
}

impl Row {
	/// Returns `true` if there are no columns in the row.
	///
	/// # Examples
	/// ```
	/// use kwik::table::{Row, Align, Style};
	///
	/// let mut row = Row::default();
	///
	/// assert!(row.is_empty());
	///
	/// row = row.push("Row 1", Align::Left, Style::Normal);
	///
	/// assert!(!row.is_empty());
	/// ```
	#[inline]
	#[must_use]
	pub fn is_empty(&self) -> bool {
		self.cells.is_empty()
	}

	/// Returns the number of columns in the row.
	///
	/// # Examples
	/// ```
	/// use kwik::table::{Row, Align, Style};
	///
	/// let mut row = Row::default()
	///     .push("Row 1", Align::Left, Style::Normal);
	///
	/// assert_eq!(row.len(), 1);
	/// ```
	#[inline]
	#[must_use]
	pub fn len(&self) -> usize {
		self.cells.len()
	}

	/// Adds a new column to the end of the row.
	///
	/// # Examples
	/// ```
	/// use kwik::table::{Row, Align, Style};
	///
	/// let mut row = Row::default()
	///     .push("Row 1", Align::Left, Style::Normal);
	/// ```
	#[inline]
	#[must_use]
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
	/// use kwik::table::{Row, Align, Style};
	///
	/// let mut row = Row::default()
	///     .blank();
	/// ```
	#[inline]
	#[must_use]
	pub fn blank(self) -> Self {
		self.push("", Align::Left, Style::Normal)
	}

	/// Returns the printed size of the row.
	#[inline]
	#[must_use]
	pub fn size(&self) -> usize {
		self.to_string(None, ColumnJoinType::Spaced).len()
	}

	/// Returns the printed size of the column at the supplied index.
	///
	/// # Panics
	///
	/// Panics if the column index is out of the bounds of the columns.
	#[inline]
	#[must_use]
	pub fn get_column_size(&self, index: usize) -> usize {
		assert!(index < self.cells.len(), "Invalid column index.");
		self.cells[index].size()
	}

	/// Prints the column to the supplied stream.
	#[inline]
	pub fn print(
		&self,
		stdout: &mut impl Write,
		sizes: &Vec<usize>,
		join_type: ColumnJoinType,
	) {
		writeln!(
			stdout,
			"{}",
			self.to_string(Some(sizes), join_type)
		).unwrap();
	}

	/// Returns the string value of the row.
	#[must_use]
	fn to_string(
		&self,
		sizes: Option<&Vec<usize>>,
		join_type: ColumnJoinType,
	) -> String {
		let join_str = match join_type {
			ColumnJoinType::Normal => "|",
			ColumnJoinType::Spaced => " | ",
			ColumnJoinType::Plus => "+",
		};

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

		if join_type == ColumnJoinType::Spaced {
			format!("| {line} |")
		} else {
			format!("|{line}|")
		}
	}
}

impl WriteRow for Row {
	fn as_row(&self, row_data: &mut RowData) -> io::Result<()> {
		for cell in &self.cells {
			row_data.push(cell.value());
		}

		Ok(())
	}
}
