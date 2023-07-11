mod row;
mod cell;

use std::collections::HashSet;
pub use crate::table::row::Row;
pub use crate::table::cell::{Direction, Style};

#[derive(Default)]
pub struct Table {
	header: Option<Row>,
	rows: Vec<Row>,
	spacers: HashSet<usize>,

	row_len: usize,
}

impl Table {
	pub fn set_header(&mut self, header: Row) {
		if !self.rows.is_empty() && header.len() != self.row_len {
			panic!("Invalid number of columns in row.");
		}

		self.row_len = header.len();
		self.header = Some(header);
		self.spacers.insert(1);
	}

	pub fn add_row(&mut self, row: Row) {
		if !self.rows.is_empty() && row.len() != self.row_len {
			panic!("Invalid number of columns in row.");
		}

		self.row_len = row.len();
		self.rows.push(row);
	}

	pub fn add_spacer(&mut self) {
		let mut index = self.rows.len();

		if self.header.is_some() {
			index += 1;
		}

		self.spacers.insert(index);
	}

	pub fn print(&self) {
		let mut index: usize = 0;
		let column_lens = self.max_column_lens();

		if self.spacers.contains(&index) {
			self.print_spacer_row(&column_lens);
		}

		if let Some(header) = &self.header {
			index += 1;

			header.print(&column_lens, true);

			if self.spacers.contains(&index) {
				self.print_spacer_row(&column_lens);
			}
		}

		for row in &self.rows {
			index += 1;

			row.print(&column_lens, true);

			if self.spacers.contains(&index) {
				self.print_spacer_row(&column_lens);
			}
		}
	}

	fn print_spacer_row(&self, sizes: &Vec<usize>) {
		let mut row = Row::default();

		for size in sizes {
			let value = vec!["-"; *size + 2].join("");
			row = row.push(value, Direction::Left, Style::Normal);
		}

		row.print(sizes, false);
	}

	pub fn max_column_lens(&self) -> Vec<usize> {
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
