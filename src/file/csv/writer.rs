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
	fmt::Display,
	marker::PhantomData,
};

use csv::Writer;

use crate::file::{
	FileWriter,
	csv::RowData,
};

/// Writes a CSV file in rows.
pub struct CsvWriter<T>
where
	T: WriteRow,
{
	file: Writer<File>,
	buf: RowData,
	count: u64,

	_marker: PhantomData<T>,
}

/// Implementing this trait allows the CSV writer to convert the
/// struct into writable rows.
pub trait WriteRow {
	/// Fills the supplied row with data to be written to the file.
	///
	/// # Examples
	/// ```
	/// use std::io;
	/// use kwik::file::csv::{WriteRow, RowData};
	///
	/// struct MyStruct {
	///     // data fields
	/// }
	///
	/// impl WriteRow for MyStruct {
	///     fn as_row(&self, row: &mut RowData) -> io::Result<()>
	///     where
	///         Self: Sized,
	///     {
	///         // modify `row`
	///         Ok(())
	///     }
	/// }
	/// ```
	///
	/// # Errors
	///
	/// This function will return an error if the row could not be created.
	fn as_row(&self, row: &mut RowData) -> io::Result<()>;
}

impl<T> FileWriter for CsvWriter<T>
where
	T: WriteRow,
{
	fn from_path<P>(path: P) -> io::Result<Self>
	where
		Self: Sized,
		P: AsRef<Path>,
	{
		CsvWriter::from_file(File::create(path)?)
	}

	fn from_file(file: File) -> io::Result<Self>
	where
		Self: Sized,
	{
		let file = Writer::from_writer(file);

		let writer = CsvWriter {
			file,
			buf: RowData::default(),
			count: 0,

			_marker: PhantomData,
		};

		Ok(writer)
	}

	fn flush(&mut self) -> io::Result<()> {
		self.file.flush()
	}
}

impl<T> CsvWriter<T>
where
	T: WriteRow,
{
	/// Adds a header row to the CSV file.
	///
	/// # Examples
	/// ```no_run
	/// use std::io;
	///
	/// use kwik::file::{
	///     FileWriter,
	///     csv::{CsvWriter, WriteRow, RowData},
	/// };
	///
	/// let mut reader = CsvWriter::<MyStruct>::from_path("/path/to/file").unwrap();
	///
	/// reader.set_headers(&["Row 1", "Row 2"]).unwrap();
	///
	/// struct MyStruct {
	///     // data fields
	///     data: u32,
	/// }
	///
	/// impl WriteRow for MyStruct {
	///     fn as_row(&self, row: &mut RowData) -> io::Result<()>
	///     where
	///         Self: Sized,
	///     {
	///         // modify `row`
	///         Ok(())
	///     }
	/// }
	/// ```
	///
	/// # Errors
	///
	/// This function will return an error if the header row could not be written.
	pub fn set_headers<H>(&mut self, headers: &[H]) -> io::Result<()>
	where
		H: Display,
	{
		if self.count > 0 {
			return Err(io::Error::new(
				io::ErrorKind::InvalidData,
				"CSV header can only be set on the first row",
			));
		}

		self.buf.data.clear();
		self.count += 1;

		for header in headers {
			self.buf.data.push_field(&header.to_string());
		}

		self.file
			.write_record(&self.buf.data)
			.map_err(|_| io::Error::new(
				io::ErrorKind::InvalidData,
				"An error occurred when writing CSV file header",
			))
	}

	/// Adds a header row to the CSV file.
	///
	/// # Examples
	/// ```no_run
	/// use std::io;
	///
	/// use kwik::file::{
	///     FileWriter,
	///     csv::{CsvWriter, WriteRow, RowData},
	/// };
	///
	/// let reader = CsvWriter::<MyStruct>::from_path("/path/to/file").unwrap()
	///     .with_headers(&["Row 1", "Row 2"]).unwrap();
	///
	/// struct MyStruct {
	///     // data fields
	///     data: u32,
	/// }
	///
	/// impl WriteRow for MyStruct {
	///     fn as_row(&self, row: &mut RowData) -> io::Result<()>
	///     where
	///         Self: Sized,
	///     {
	///         // modify `row`
	///         Ok(())
	///     }
	/// }
	/// ```
	///
	/// # Errors
	///
	/// This function will return an error if the header row could not be written.
	pub fn with_headers<H>(mut self, headers: &[H]) -> io::Result<Self>
	where
		H: Display,
	{
		self.set_headers(headers)?;
		Ok(self)
	}

	/// Writes one row to the CSV file.
	///
	/// # Examples
	/// ```no_run
	/// use std::io;
	///
	/// use kwik::file::{
	///     FileWriter,
	///     csv::{CsvWriter, WriteRow, RowData},
	/// };
	///
	/// let mut reader = CsvWriter::<MyStruct>::from_path("/path/to/file").unwrap();
	///
	/// reader.write_row(&MyStruct { data: 0 }).unwrap();
	///
	/// struct MyStruct {
	///     // data fields
	///     data: u32,
	/// }
	///
	/// impl WriteRow for MyStruct {
	///     fn as_row(&self, row: &mut RowData) -> io::Result<()>
	///     where
	///         Self: Sized,
	///     {
	///         // modify `row`
	///         Ok(())
	///     }
	/// }
	/// ```
	///
	/// # Errors
	///
	/// This function will return an error if the row could not be written.
	#[inline]
	pub fn write_row(&mut self, object: &T) -> io::Result<()> {
		self.buf.data.clear();
		self.count += 1;

		object.as_row(&mut self.buf)?;

		self.file
			.write_record(&self.buf.data)
			.map_err(|_| {
				let message = format!(
					"An error occurred on row {} when writing CSV file",
					self.count,
				);

				io::Error::new(io::ErrorKind::InvalidData, message)
			})
	}
}
