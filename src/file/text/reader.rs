/*
 * Copyright (c) Kia Shakiba
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

use std::{
	path::Path,
	fs::File,
	io::{BufReader, BufRead, Error, ErrorKind},
};

use crate::file::FileReader;

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
	fn new<P>(path: P) -> Result<Self, Error>
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

	#[inline]
	fn size(&self) -> u64 {
		let metadata = self.file
			.get_ref()
			.metadata()
			.expect("Could not get binary file's size");

		metadata.len()
	}
}

impl TextReader {
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

			Err(ref err) if err.kind() ==  ErrorKind::UnexpectedEof => None,
			Err(_) => panic!("An error occurred on line {} when reading text file", self.count + 1),
		}
	}

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
