/*
 * Copyright (c) Kia Shakiba
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

mod reader;
mod writer;

use std::io::{Error, ErrorKind};
use csv::StringRecord;

pub struct RowData {
	data: StringRecord,
}

impl RowData {
	fn new() -> Self {
		RowData {
			data: StringRecord::new(),
		}
	}

	#[inline]
	pub fn push(&mut self, value: &str) {
		self.data.push_field(value);
	}

	#[inline]
	pub fn get(&self, index: usize) -> Result<&str, Error> {
		self.data
			.get(index)
			.ok_or(Error::new(
				ErrorKind::InvalidData,
				format!("Invalid CSV column {}", index)
			))
	}

	#[inline]
	pub fn size(&self) -> usize {
		let items_size = self.data
			.iter()
			.map(|item| item.as_bytes().len())
			.sum::<usize>();

		items_size + self.data.len()
	}
}

pub use crate::file::csv::{
	reader::{CsvReader, ReadRow, Iter, IntoIter},
	writer::{CsvWriter, WriteRow},
};
