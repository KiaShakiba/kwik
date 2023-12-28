/*
 * Copyright (c) Kia Shakiba
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

mod row;
mod cell;

use std::{
	path::Path,
	io::{Write, Error},
	collections::HashSet,
};

pub use crate::{
	csv_writer::{FileWriter, CsvWriter},
	table::{
		row::{Row, ColumnJoinType},
		cell::{Align, Style},
	},
};

#[derive(Default)]
pub struct Table {
	header: Option<Row>,
	rows: Vec<Row>,
	spacers: HashSet<usize>,

	row_len: usize,
}

/// Prints a table to a stream.
impl Table {
	/// Sets the table's header row. The header row is followed by a spacer
	/// row by default.
	///
	/// # Examples
	/// ```
	/// use kwik::table::{Table, Row, Align, Style};
	///
	/// let mut table = Table::default();
	///
	/// let header = Row::default()
	///     .push("Header 1", Align::Center, Style::Bold);
	///
	/// table.set_header(header);
	///
	/// let mut stdout = Vec::new();
	/// table.print(&mut stdout);
	///
	/// assert_eq!(stdout, b"| \x1B[1mHeader 1\x1B[0m |\n|----------|\n");
	/// ```
	pub fn set_header(&mut self, header: Row) {
		if !self.rows.is_empty() && header.len() != self.row_len {
			panic!("Invalid number of columns in row.");
		}

		self.row_len = header.len();
		self.header = Some(header);
		self.spacers.insert(1);
	}

	/// Adds a row to the table;
	///
	/// # Examples
	/// ```
	/// use kwik::table::{Table, Row, Align, Style};
	///
	/// let mut table = Table::default();
	///
	/// let row = Row::default()
	///     .push("Row 1", Align::Left, Style::Normal);
	///
	/// table.add_row(row);
	///
	/// let mut stdout = Vec::new();
	/// table.print(&mut stdout);
	///
	/// assert_eq!(stdout, b"| Row 1 |\n");
	/// ```
	pub fn add_row(&mut self, row: Row) {
		if !self.rows.is_empty() && row.len() != self.row_len {
			panic!("Invalid number of columns in row.");
		}

		self.row_len = row.len();
		self.rows.push(row);
	}

	/// Adds a spacer row to the table.
	///
	/// # Examples
	/// ```
	/// use kwik::table::{Table, Row, Align, Style};
	///
	/// let mut table = Table::default();
	///
	/// let row1 = Row::default()
	///     .push("Row 1", Align::Left, Style::Normal);
	///
	/// let row2 = Row::default()
	///     .push("Row 2", Align::Left, Style::Normal);
	///
	/// table.add_row(row1);
	/// table.add_spacer();
	/// table.add_row(row2);
	///
	/// let mut stdout = Vec::new();
	/// table.print(&mut stdout);
	///
	/// assert_eq!(stdout, b"| Row 1 |\n|-------|\n| Row 2 |\n");
	/// ```
	pub fn add_spacer(&mut self) {
		let mut index = self.rows.len();

		if self.header.is_some() {
			index += 1;
		}

		self.spacers.insert(index);
	}

	/// Prints the table to the supplied stream.
	///
	/// # Examples
	/// ```
	/// use kwik::table::{Table, Row, Align, Style};
	///
	/// let mut table = Table::default();
	///
	/// let header = Row::default()
	///     .push("Header 1", Align::Center, Style::Bold);
	///
	/// let row = Row::default()
	///     .push("Longer row 1", Align::Left, Style::Normal);
	///
	/// table.set_header(header);
	/// table.add_row(row);
	///
	/// let mut stdout = Vec::new();
	/// table.print(&mut stdout);
	///
	/// assert_eq!(stdout, b"| \x1B[1m  Header 1  \x1B[0m |\n|--------------|\n| Longer row 1 |\n");
	/// ```
	pub fn print(&self, stdout: &mut impl Write) {
		let mut index: usize = 0;
		let column_lens = self.max_column_lens();

		if self.spacers.contains(&index) {
			self.print_spacer_row(stdout, &column_lens);
		}

		if let Some(header) = &self.header {
			index += 1;

			header.print(stdout, &column_lens, ColumnJoinType::Spaced);

			if self.spacers.contains(&index) {
				self.print_spacer_row(stdout, &column_lens);
			}
		}

		for row in &self.rows {
			index += 1;

			row.print(stdout, &column_lens, ColumnJoinType::Spaced);

			if self.spacers.contains(&index) {
				self.print_spacer_row(stdout, &column_lens);
			}
		}
	}

	/// Writes the table to the file at the supplied path.
	///
	/// # Examples
	/// ```
	/// use std::env;
	/// use kwik::table::{Table, Row, Align, Style};
	///
	/// let mut table = Table::default();
	///
	/// let header = Row::default()
	///     .push("Header 1", Align::Center, Style::Bold);
	///
	/// let row = Row::default()
	///     .push("Longer row 1", Align::Left, Style::Normal);
	///
	/// table.set_header(header);
	/// table.add_row(row);
	///
	/// let mut path = env::var("CARGO_MANIFEST_DIR").unwrap();
	/// path.push_str("/target/table.csv");
	///
	/// table.to_file(path).expect("Could not write table to file.");
	/// ```
	pub fn to_file<P>(&self, path: P) -> Result<(), Error>
	where
		P: AsRef<Path>,
	{
		let mut writer = CsvWriter::<Row>::new(path)?;

		if let Some(header) = &self.header {
			writer.write_row(header);
		}

		for row in &self.rows {
			writer.write_row(row);
		}

		Ok(())
	}

	fn print_spacer_row(
		&self,
		stdout: &mut impl Write,
		sizes: &Vec<usize>
	) {
		let mut row = Row::default();

		for size in sizes {
			let value = vec!["-"; *size + 2].join("");
			row = row.push(value, Align::Left, Style::Normal);
		}

		row.print(stdout, sizes, ColumnJoinType::Plus);
	}

	fn max_column_lens(&self) -> Vec<usize> {
		let mut sizes: Vec<usize> = vec![0; self.row_len];

		if let Some(header) = &self.header {
			for (index, size) in sizes.iter_mut().enumerate() {
				let row_column_size = header.get_column_size(index);

				if row_column_size > *size {
					*size = row_column_size;
				}
			}
		}

		for row in &self.rows {
			for (index, size) in sizes.iter_mut().enumerate() {
				let row_column_size = row.get_column_size(index);

				if row_column_size > *size {
					*size = row_column_size;
				}
			}
		}

		sizes
	}
}
