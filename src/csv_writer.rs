/*
 * Copyright (c) Kia Shakiba
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

use std::fs::File;
use std::io::{BufWriter, Write, Error, ErrorKind};
use std::marker::PhantomData;
use csv::Writer;
use crate::file_writer::FileWriter;

pub use csv::StringRecord as StringRow;

pub struct CsvWriter<T: Row> {
	file: Writer<File>,
	buf: StringRow,
	count: u64,

	_marker: PhantomData<T>,
}

pub trait Row {
	fn as_row(&self, _: &mut StringRow) -> Result<(), Error>;
}

impl<T: Row> FileWriter for CsvWriter<T> {
	fn new(path: &str) -> Result<Self, Error> where Self: Sized {
		let Ok(file) = Writer::from_path(path) else {
			return Err(Error::new(
				ErrorKind::NotFound,
				"Could not create CSV file."
			));
		};

		let writer = CsvWriter {
			file,
			buf: StringRow::new(),
			count: 0,

			_marker: PhantomData,
		};

		Ok(writer)
	}
}

impl<T: Row> CsvWriter<T> {
	pub fn write_row(&mut self, object: &T) {
		self.buf.clear();
		self.count += 1;

		if let Err(_) = object.as_row(&mut self.buf) {
			panic!("Error converting object {} to row", self.count);
		}

		if let Err(_) = self.file.write_record(&self.buf) {
			panic!("Could not write to CSV file at row {}.", self.count);
		}
	}
}
