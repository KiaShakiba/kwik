/*
 * Copyright (c) Kia Shakiba
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

use std::{
	path::Path,
	fs::File,
	io::{
		self,
		LineWriter,
		Write,
		Seek,
		SeekFrom,
	},
};

use crate::file::FileWriter;

/// Writes a text file line-by-line.
pub struct TextWriter {
	file: LineWriter<File>,
	count: u64,
}

impl FileWriter for TextWriter {
	fn from_path<P>(path: P) -> io::Result<Self>
	where
		Self: Sized,
		P: AsRef<Path>,
	{
		TextWriter::from_file(File::create(path)?)
	}

	fn from_file(file: File) -> io::Result<Self>
	where
		Self: Sized,
	{
		let writer = TextWriter {
			file: LineWriter::new(file),
			count: 0,
		};

		Ok(writer)
	}

	fn flush(&mut self) -> io::Result<()> {
		self.file.flush()
	}
}

impl TextWriter {
	/// Writes one line to the text file.
	///
	/// # Examples
	/// ```no_run
	/// use std::io;
	///
	/// use kwik::file::{
	///     FileWriter,
	///     text::TextWriter,
	/// };
	///
	/// let mut reader = TextWriter::from_path("/path/to/file").unwrap();
	///
	/// reader.write_line(b"data").unwrap();
	/// ```
	///
	/// # Errors
	///
	/// This function will return an error if the line could not be written.
	#[inline]
	pub fn write_line<T>(&mut self, line: T) -> io::Result<()>
	where
		T: AsRef<str>,
	{
		self.count += 1;
		self.file.write_all(&[line.as_ref().as_bytes(), b"\n"].concat())
	}
}

impl Seek for TextWriter {
	fn seek(&mut self, pos: SeekFrom) -> io::Result<u64> {
		self.file.get_ref().seek(pos)
	}
}
