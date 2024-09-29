/*
 * Copyright (c) Kia Shakiba
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

use std::{
	path::Path,
	fs::File,
	io::{self, LineWriter, Write},
};

use crate::file::FileWriter;

/// Writes a text file line-by-line.
pub struct TextWriter {
	file: LineWriter<File>,
	count: u64,
}

impl FileWriter for TextWriter {
	fn new<P>(path: P) -> io::Result<Self>
	where
		Self: Sized,
		P: AsRef<Path>,
	{
		let opened_file = File::create(path)?;

		let writer = TextWriter {
			file: LineWriter::new(opened_file),
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
	/// let mut reader = TextWriter::new("/path/to/file").unwrap();
	///
	/// reader.write_line(b"data").unwrap();
	/// ```
	///
	/// # Errors
	///
	/// This function will return an error if the line could not be written.
	#[inline]
	pub fn write_line(&mut self, line: &[u8]) -> io::Result<()> {
		self.count += 1;
		self.file.write_all(&[line, b"\n"].concat())
	}
}
