/*
 * Copyright (c) Kia Shakiba
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

mod reader;
mod writer;

use std::io;
use num_traits::AsPrimitive;
use csv::StringRecord;

/// CSV row data.
#[derive(Default)]
pub struct RowData {
	data: StringRecord,
}

impl RowData {
	/// Returns `true` if the row is empty (i.e., has no columns).
	#[inline]
	pub fn is_empty(&self) -> bool {
		self.data.is_empty()
	}

	/// Returns the number of columns in the row.
	#[inline]
	pub fn len(&self) -> usize {
		self.data.len()
	}

	/// Returns the size of the row in bytes, including commas
	/// and the new line character.
	#[inline]
	pub fn size(&self) -> usize {
		let items_size = self.data
			.iter()
			.map(|item| item.len())
			.sum::<usize>();

		items_size + self.data.len()
	}

	/// Returns the column data at the supplied index.
	///
	/// # Errors
	///
	/// This function returns an error if the column does not exist.
	#[inline]
	pub fn get(&self, index: impl AsPrimitive<usize>) -> io::Result<&str> {
		self.data
			.get(index.as_())
			.ok_or(io::Error::new(
				io::ErrorKind::InvalidData,
				format!("Invalid CSV column {}", index.as_()),
			))
	}

	/// Adds a new column to the end of the row.
	#[inline]
	pub fn push<T>(&mut self, value: T)
	where
		T: AsRef<str>,
	{
		self.data.push_field(value.as_ref());
	}
}

pub use crate::file::csv::{
	reader::{CsvReader, ReadRow, Iter, IntoIter},
	writer::{CsvWriter, WriteRow},
};
