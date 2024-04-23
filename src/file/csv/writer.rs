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

use csv::Writer;

use crate::file::{
	FileWriter,
	csv::RowData,
};

pub struct CsvWriter<T>
where
	T: WriteRow,
{
	file: Writer<File>,
	buf: RowData,
	count: u64,

	_marker: PhantomData<T>,
}

pub trait WriteRow {
	fn as_row(&self, row_data: &mut RowData) -> Result<(), Error>;
}

impl<T> FileWriter for CsvWriter<T>
where
	T: WriteRow,
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
	T: WriteRow,
{
	#[inline]
	pub fn write_row(&mut self, object: &T) {
		self.buf.data.clear();
		self.count += 1;

		assert!(
			object.as_row(&mut self.buf).is_ok(),
			"Error converting object {} to row",
			self.count,
		);

		assert!(
			self.file.write_record(&self.buf.data).is_ok(),
			"Could not write to CSV file at row {}",
			self.count,
		);
	}
}
