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
		BufReader,
		BufRead,
		Seek,
		SeekFrom,
	},
};

use crate::file::FileReader;

/// Reads a text file line-by-line.
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
	fn from_path<P>(path: P) -> io::Result<Self>
	where
		Self: Sized,
		P: AsRef<Path>,
	{
		TextReader::from_file(File::open(path)?)
	}

	fn from_file(file: File) -> io::Result<Self>
	where
		Self: Sized,
	{
		let reader = TextReader {
			file: BufReader::new(file),
			buf: String::new(),
			count: 0,
		};

		Ok(reader)
	}

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
	/// Reads one line of the text file and returns a `Result` containing
	/// the line. If the end of the file is reached, an `io::Error` is returned.
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
	/// let mut reader = TextReader::from_path("/path/to/file").unwrap();
	///
	/// while let Ok(line) = reader.read_line() {
	///     // do something with the line
	/// }
	/// ```
	///
	/// # Errors
	///
	/// This function will return an error if the line could not be read.
	#[inline]
	pub fn read_line(&mut self) -> io::Result<String> {
		self.buf.clear();

		self.file
			.read_line(&mut self.buf)
			.and_then(|buf_size| {
				if buf_size == 0 {
					return Err(io::Error::new(
						io::ErrorKind::UnexpectedEof,
						"The end of the file has been reached",
					));
				}

				self.count += 1;

				if self.buf.ends_with('\n') {
					self.buf.pop();

					if self.buf.ends_with('\r') {
						self.buf.pop();
					}
				}

				Ok(self.buf.clone())
			})
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
	/// let mut reader = TextReader::from_path("/path/to/file").unwrap();
	///
	/// for line in reader.iter() {
	///     // do something with the line
	/// }
	/// ```
	#[inline]
	pub fn iter(&mut self) -> Iter<'_> {
		Iter {
			reader: self
		}
	}
}

impl Seek for TextReader {
	fn seek(&mut self, pos: SeekFrom) -> io::Result<u64> {
		self.file.seek(pos)
	}
}

impl Iterator for Iter<'_> {
	type Item = String;

	fn next(&mut self) -> Option<Self::Item> {
		match self.reader.read_line() {
			Ok(line) => Some(line),
			Err(err) if err.kind() == io::ErrorKind::UnexpectedEof => None,

			Err(_) => panic!(
				"An error occurred on line {} when reading text file",
				self.reader.count + 1,
			),
		}
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
		match self.reader.read_line() {
			Ok(line) => Some(line),
			Err(err) if err.kind() == io::ErrorKind::UnexpectedEof => None,

			Err(_) => panic!(
				"An error occurred on line {} when reading text file",
				self.reader.count + 1,
			),
		}
	}
}
