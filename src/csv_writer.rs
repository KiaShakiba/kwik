/*
 * Copyright (c) Kia Shakiba
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

use std::{
	path::Path,
	fs::File,
	io::Error,
	marker::PhantomData,
};

use csv::{Writer, StringRecord};
pub use crate::file_writer::FileWriter;

pub struct CsvWriter<T>
where
	T: Row,
{
	file: Writer<File>,
	buf: RowData,
	count: u64,

	_marker: PhantomData<T>,
}

pub struct RowData {
	data: StringRecord,
}

pub trait Row {
	fn as_row(&self, _: &mut RowData) -> Result<(), Error>;
}

impl<T> FileWriter for CsvWriter<T>
where
	T: Row,
{
	fn new<P>(path: P) -> Result<Self, Error>
	where
		Self: Sized,
		P: AsRef<Path>,
	{
		let file = Writer::from_path(path)?;

		let writer = CsvWriter {
			file,
			buf: RowData::new(),
			count: 0,

			_marker: PhantomData,
		};

		Ok(writer)
	}
}

impl<T> CsvWriter<T>
where
	T: Row,
{
	pub fn write_row(&mut self, object: &T) {
		self.buf.data.clear();
		self.count += 1;

		if object.as_row(&mut self.buf).is_err() {
			panic!("Error converting object {} to row", self.count);
		}

		if self.file.write_record(&self.buf.data).is_err() {
			panic!("Could not write to CSV file at row {}.", self.count);
		}
	}
}

impl RowData {
	fn new() -> Self {
		RowData {
			data: StringRecord::new(),
		}
	}

	pub fn push(&mut self, value: &str) {
		self.data.push_field(value);
	}

	pub fn size(&self) -> usize {
		let items_size = self.data
			.iter()
			.map(|item| item.as_bytes().len())
			.sum::<usize>();

		items_size + self.data.len()
	}
}
