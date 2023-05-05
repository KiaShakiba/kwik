/*
 * Copyright (c) Kia Shakiba
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

use std::fs::File;
use std::io::{BufReader, BufRead, Error, ErrorKind};
pub use crate::file_reader::FileReader;

pub struct TextReader {
	file: BufReader<File>,
	buf: String,
	count: u64,
}

impl FileReader for TextReader {
	fn new(path: &str) -> Result<Self, Error> {
		let Ok(opened_file) = File::open(path) else {
			return Err(Error::new(
				ErrorKind::NotFound,
				"Could not open text file."
			));
		};

		let reader = TextReader {
			file: BufReader::new(opened_file),
			buf: String::new(),
			count: 0,
		};

		Ok(reader)
	}

	fn size(&self) -> u64 {
		let Ok(metadata) = self.file.get_ref().metadata() else {
			panic!("Could not get text file's size.");
		};

		metadata.len()
	}
}

impl TextReader {
	pub fn read_line(&mut self) -> Option<&String> {
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

				Some(&self.buf)
			},

			Err(ref err) if err.kind() ==  ErrorKind::UnexpectedEof => None,

			Err(_) => {
				panic!("An error occurred on line {} when reading text file.", self.count);
			},
		}
	}
}
