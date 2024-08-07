/*
 * Copyright (c) Kia Shakiba
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

use std::{
	path::Path,
	fs::File,
	io::{self, BufReader, BufRead},
};

use crate::file::FileReader;

/// Reads a text file line-by-line
pub struct TextReader {
	file: BufReader<File>,
	buf: String,
	count: u64,
}

pub struct Iter<'a>
{
	reader: &'a mut TextReader,
}

pub struct IntoIter {
	reader: TextReader,
}

impl FileReader for TextReader {
	/// Opens the file at the supplied path. If the file could not be
	/// opened, returns an error result.
	fn new<P>(path: P) -> io::Result<Self>
	where
		Self: Sized,
		P: AsRef<Path>,
	{
		let opened_file = File::open(path)?;

		let reader = TextReader {
			file: BufReader::new(opened_file),
			buf: String::new(),
			count: 0,
		};

		Ok(reader)
	}

	/// Returns the number of bytes in the opened file.
	#[inline]
	fn size(&self) -> u64 {
		let metadata = self.file
			.get_ref()
			.metadata()
			.expect("Could not get text file's size");

		metadata.len()
	}
}

impl TextReader {
	/// Reads one line of the text file and returns an option containing
	/// the line. If the end of the file is reached, `None` is returned.
	///
	/// # Examples
	/// ```no_run
	/// use std::io;
	///
	/// use kwik::file::{
	///     FileReader,
	///     text::TextReader,
	/// };
	///
	/// let mut reader = TextReader::new("/path/to/file").unwrap();
	///
	/// while let Some(line) = reader.read_line() {
	///     // do something with the line
	/// }
	/// ```
	#[inline]
	pub fn read_line(&mut self) -> Option<String> {
		self.buf.clear();

		match self.file.read_line(&mut self.buf) {
			Ok(buf_size) => {
				if buf_size == 0 {
					return None;
				}

				self.count += 1;

				if self.buf.ends_with('\n') {
					self.buf.pop();

					if self.buf.ends_with('\r') {
						self.buf.pop();
					}
				}

				Some(self.buf.clone())
			},

			Err(ref err) if err.kind() == io::ErrorKind::UnexpectedEof => None,
			Err(_) => panic!("An error occurred on line {} when reading text file", self.count + 1),
		}
	}

	/// Returns an iterator over the text file. The iterator takes a mutable
	/// reference to `self` as it is iterating over a stream. This means performing
	/// the iteration modifies the reader's position in the file.
	///
	/// # Examples
	/// ```no_run
	/// use std::io;
	///
	/// use kwik::file::{
	///     FileReader,
	///     text::TextReader,
	/// };
	///
	/// let mut reader = TextReader::new("/path/to/file").unwrap();
	///
	/// for line in reader.iter() {
	///     // do something with the line
	/// }
	/// ```
	#[inline]
	pub fn iter(&mut self) -> Iter {
		Iter {
			reader: self
		}
	}
}

impl<'a> Iterator for Iter<'a> {
	type Item = String;

	fn next(&mut self) -> Option<Self::Item> {
		self.reader.read_line()
	}
}

impl IntoIterator for TextReader {
	type Item = String;
	type IntoIter = IntoIter;

	fn into_iter(self) -> Self::IntoIter {
		IntoIter {
			reader: self
		}
	}
}

impl Iterator for IntoIter {
	type Item = String;

	fn next(&mut self) -> Option<Self::Item> {
		self.reader.read_line()
	}
}
