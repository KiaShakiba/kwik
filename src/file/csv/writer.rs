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
	///     fn as_row(&self, row_data: &mut RowData) -> io::Result<()>
	///     where
	///         Self: Sized,
	///     {
	///         // modify `row_data`
	///         Ok(())
	///     }
	/// }
	/// ```
	///
	/// # Errors
	///
	/// This function will return an error if the row could not be created.
	fn as_row(&self, row_data: &mut RowData) -> io::Result<()>;
}

impl<T> FileWriter for CsvWriter<T>
where
	T: WriteRow,
{
	/// Opens the file at the supplied path. If the file could not be
	/// opened, returns an error result.
	fn new<P>(path: P) -> io::Result<Self>
	where
		Self: Sized,
		P: AsRef<Path>,
	{
		let file = Writer::from_path(path)?;

		let writer = CsvWriter {
			file,
			buf: RowData::default(),
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
	/// let mut reader = CsvWriter::<MyStruct>::new("/path/to/file").unwrap();
	///
	/// reader.write_row(&MyStruct { data: 0 }).unwrap();
	///
	/// struct MyStruct {
	///     // data fields
	///     data: u32,
	/// }
	///
	/// impl WriteRow for MyStruct {
	///     fn as_row(&self, row_data: &mut RowData) -> io::Result<()>
	///     where
	///         Self: Sized,
	///     {
	///         // modify `row_data`
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
