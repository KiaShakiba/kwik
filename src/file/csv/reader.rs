/*
 * Copyright (c) Kia Shakiba
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

use std::{
	io,
	path::Path,
	fs::File,
	marker::PhantomData,
};

use csv::{Reader, ReaderBuilder};

use crate::file::{
	FileReader,
	csv::RowData,
};

/// Reads a CSV file in rows.
pub struct CsvReader<T>
where
	T: ReadRow,
{
	file: Reader<File>,
	buf: RowData,
	count: u64,

	_marker: PhantomData<T>,
}

/// Implementing this trait allows the CSV reader to parse rows
/// of the CSV file into the specified type.
pub trait ReadRow {
	/// Returns an instance of the implemented struct, given a row
	/// of the CSV file. If the row could not be parsed, an
	/// error result is returned.
	///
	/// # Examples
	/// ```
	/// use std::io;
	/// use kwik::file::csv::{ReadRow, RowData};
	///
	/// struct MyStruct {
	///     // data fields
	/// }
	///
	/// impl ReadRow for MyStruct {
	///     fn new(row_data: &RowData) -> io::Result<Self>
	///     where
	///         Self: Sized,
	///     {
	///         // parse the row and return an instance of `Self` on success
	///         Ok(MyStruct {})
	///     }
	/// }
	/// ```
	///
	/// # Errors
	///
	/// This function will return an error if the row could not be parsed.
	fn new(row_data: &RowData) -> io::Result<Self>
	where
		Self: Sized,
	;
}

pub struct Iter<'a, T>
where
	T: ReadRow,
{
	reader: &'a mut CsvReader<T>,
}

pub struct IntoIter<T>
where
	T: ReadRow,
{
	reader: CsvReader<T>,
}

impl<T> FileReader for CsvReader<T>
where
	T: ReadRow,
{
	/// Opens the file at the supplied path. If the file could not be
	/// opened, returns an error result.
	fn new<P>(path: P) -> io::Result<Self>
	where
		Self: Sized,
		P: AsRef<Path>,
	{
		let reader = ReaderBuilder::new()
			.has_headers(false)
			.from_path(path)?;

		let reader = CsvReader {
			file: reader,
			buf: RowData::default(),
			count: 0,

			_marker: PhantomData,
		};

		Ok(reader)
	}

	/// Returns the number of bytes in the opened file.
	#[inline]
	fn size(&self) -> u64 {
		let metadata = self.file
			.get_ref()
			.metadata()
			.expect("Could not get CSV file's size");

		metadata.len()
	}
}

impl<T> CsvReader<T>
where
	T: ReadRow,
{
	/// Reads one row of the CSV file and returns an option containing
	/// the parsed row. If the end of the file is reached, `None` is returned.
	///
	/// # Examples
	/// ```no_run
	/// use std::io;
	///
	/// use kwik::file::{
	///     FileReader,
	///     csv::{CsvReader, ReadRow, RowData},
	/// };
	///
	/// let mut reader = CsvReader::<MyStruct>::new("/path/to/file").unwrap();
	///
	/// while let Ok(object) = reader.read_row() {
	///     // do something with the object
	/// }
	///
	/// struct MyStruct {
	///     // data fields
	///     data: u32,
	/// }
	///
	/// impl ReadRow for MyStruct {
	///     fn new(row_data: &RowData) -> io::Result<Self>
	///     where
	///         Self: Sized,
	///     {
	///         // parse the row and return an instance of `Self` on success
	///         Ok(MyStruct { data: 0 })
	///     }
	/// }
	/// ```
	///
	/// # Errors
	///
	/// This function will return an error if the row could not be read.
	#[inline]
	pub fn read_row(&mut self) -> io::Result<T> {
		self.buf.data.clear();

		let result = self.file
			.read_record(&mut self.buf.data)
			.map_err(|_| {
				let message = format!(
					"An error occurred on row {} when reading CSV file",
					self.count + 1,
				);

				io::Error::new(io::ErrorKind::InvalidData, message)
			})?;

		if !result {
			return Err(io::Error::new(
				io::ErrorKind::UnexpectedEof,
				"The end of the file has been reached",
			));
		}

		self.count += 1;

		let row = T::new(&self.buf)?;
		Ok(row)
	}

	/// Returns an iterator over the CSV file. The iterator takes a mutable
	/// reference to `self` as it is iterating over a stream. This means performing
	/// the iteration modifies the reader's position in the file.
	///
	/// # Examples
	/// ```no_run
	/// use std::io;
	///
	/// use kwik::file::{
	///     FileReader,
	///     csv::{CsvReader, ReadRow, RowData},
	/// };
	///
	/// let mut reader = CsvReader::<MyStruct>::new("/path/to/file").unwrap();
	///
	/// for row in reader.iter() {
	///     // do something with the object
	/// }
	///
	/// struct MyStruct {
	///     // data fields
	///     data: u32,
	/// }
	///
	/// impl ReadRow for MyStruct {
	///     fn new(row: &RowData) -> io::Result<Self>
	///     where
	///         Self: Sized,
	///     {
	///         // parse the row and return an instance of `Self` on success
	///         Ok(MyStruct { data: 0 })
	///     }
	/// }
	/// ```
	#[inline]
	pub fn iter(&mut self) -> Iter<T> {
		Iter {
			reader: self
		}
	}
}

impl<'a, T> Iterator for Iter<'a, T>
where
	T: ReadRow,
{
	type Item = T;

	fn next(&mut self) -> Option<Self::Item> {
		match self.reader.read_row() {
			Ok(line) => Some(line),
			Err(err) if err.kind() == io::ErrorKind::UnexpectedEof => None,

			Err(_) => panic!(
				"An error occurred on row {} when reading CSV file",
				self.reader.count + 1,
			),
		}
	}
}

impl<T> IntoIterator for CsvReader<T>
where
	T: ReadRow,
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
	T: ReadRow,
{
	type Item = T;

	fn next(&mut self) -> Option<Self::Item> {
		match self.reader.read_row() {
			Ok(line) => Some(line),
			Err(err) if err.kind() == io::ErrorKind::UnexpectedEof => None,

			Err(_) => panic!(
				"An error occurred on row {} when reading CSV file",
				self.reader.count + 1,
			),
		}
	}
}
