/*
 * Copyright (c) Kia Shakiba
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

use std::{
	path::Path,
	fs::File,
	io::{Error, ErrorKind},
	marker::PhantomData,
};

use csv::{Reader, ReaderBuilder, StringRecord};
pub use crate::file_reader::FileReader;

pub struct CsvReader<T>
where
	T: Row,
{
	file: Reader<File>,
	buf: RowData,
	count: u64,

	_marker: PhantomData<T>,
}

pub struct RowData {
	data: StringRecord,
}

pub trait Row {
	fn new(csv_row: &RowData) -> Result<Self, Error>
	where
		Self: Sized,
	;
}

pub struct Iter<'a, T>
where
	T: Row,
{
	reader: &'a mut CsvReader<T>,
}

pub struct IntoIter<T>
where
	T: Row,
{
	reader: CsvReader<T>,
}

impl<T> FileReader for CsvReader<T>
where
	T: Row,
{
	fn new<P>(path: P) -> Result<Self, Error>
	where
		Self: Sized,
		P: AsRef<Path>,
	{
		let reader = ReaderBuilder::new()
			.has_headers(false)
			.from_path(path)?;

		let reader = CsvReader {
			file: reader,
			buf: RowData::new(),
			count: 0,

			_marker: PhantomData,
		};

		Ok(reader)
	}

	#[inline]
	fn size(&self) -> u64 {
		let metadata = self.file
			.get_ref()
			.metadata()
			.expect("Could not get binary file's size.");

		metadata.len()
	}
}

impl<T> CsvReader<T>
where
	T: Row,
{
	#[inline]
	pub fn read_row(&mut self) -> Option<T> {
		self.buf.data.clear();

		let Ok(result) = self.file.read_record(&mut self.buf.data) else {
			panic!("An error occurred on CSV row {}.", self.count + 1);
		};

		if !result {
			return None;
		}

		self.count += 1;

		let row = match T::new(&self.buf) {
			Ok(row) => row,
			Err(err) => panic!("Parse error on CSV row {}: {:?}", self.count, err),
		};

		Some(row)
	}

	#[inline]
	pub fn iter(&mut self) -> Iter<T> {
		Iter {
			reader: self
		}
	}
}

impl RowData {
	fn new() -> Self {
		RowData {
			data: StringRecord::new(),
		}
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

impl<'a, T> Iterator for Iter<'a, T>
where
	T: Row,
{
	type Item = T;

	fn next(&mut self) -> Option<Self::Item> {
		self.reader.read_row()
	}
}

impl<T> IntoIterator for CsvReader<T>
where
	T: Row,
{
	type Item = T;
	type IntoIter = IntoIter<T>;

	fn into_iter(self) -> Self::IntoIter {
		IntoIter {
			reader: self
		}
	}
}

impl<T> Iterator for IntoIter<T>
where
	T: Row,
{
	type Item = T;

	fn next(&mut self) -> Option<Self::Item> {
		self.reader.read_row()
	}
}
