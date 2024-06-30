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

/// Writes a text file line-by-line
pub struct TextWriter {
	file: LineWriter<File>,
	count: u64,
}

impl FileWriter for TextWriter {
	/// Opens the file at the supplied path. If the file could not be
	/// opened, returns an error result.
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
	/// reader.write_line(b"data");
	/// ```
	#[inline]
	pub fn write_line(&mut self, line: &[u8]) {
		self.count += 1;

		if self.file.write_all(&[line, b"\n"].concat()).is_err() {
			panic!("Could not write to text file at line {}", self.count);
		}
	}
}
