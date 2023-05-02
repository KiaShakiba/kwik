/*
 * Copyright (c) Kia Shakiba
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

use std::fs::File;
use std::io::{Error, ErrorKind};
use std::marker::PhantomData;
use csv::Reader;
use crate::file_reader::FileReader;

pub use csv::StringRecord as StringRow;

pub struct CsvReader<T: Row> {
	file: Reader<File>,
	buf: StringRow,
	count: u64,

	_marker: PhantomData<T>,
}

pub trait Row {
	fn new(_: &StringRow) -> Result<Self, Error> where Self: Sized;
}

impl<T: Row> FileReader for CsvReader<T> {
	fn new(path: &str) -> Result<Self, Error> {
		let Ok(file) = Reader::from_path(path) else {
			return Err(Error::new(
				ErrorKind::NotFound,
				"Could not open CSV file."
			));
		};

		let reader = CsvReader {
			file,
			buf: StringRow::new(),
			count: 0,

			_marker: PhantomData,
		};

		Ok(reader)
	}

	fn size(&self) -> u64 {
		let Ok(metadata) = self.file.get_ref().metadata() else {
			panic!("Could not get CSV file's size.");
		};

		metadata.len()
	}
}

impl<T: Row> CsvReader<T> {
	pub fn read_row(&mut self) -> Option<T> {
		match self.file.read_record(&mut self.buf) {
			Ok(result) => {
				if !result {
					return None;
				}

				self.count += 1;

				let row = match T::new(&self.buf) {
					Ok(row) => row,
					Err(err) => panic!("Parse error in row {}: {:?}", self.count, err),
				};

				Some(row)
			},

			Err(_) => {
				panic!("An error occurred on line {} when reading CSV file.", self.count);
			},
		}
	}
}
