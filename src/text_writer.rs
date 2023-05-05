/*
 * Copyright (c) Kia Shakiba
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

use std::fs::File;
use std::io::{LineWriter, Write, Error, ErrorKind};
pub use crate::file_writer::FileWriter;

pub struct TextWriter {
	file: LineWriter<File>,
	count: u64,
}

impl FileWriter for TextWriter {
	fn new(path: &str) -> Result<Self, Error> where Self: Sized {
		let Ok(opened_file) = File::create(path) else {
			return Err(Error::new(
				ErrorKind::PermissionDenied,
				"Could not create text file."
			));
		};

		let writer = TextWriter {
			file: LineWriter::new(opened_file),
			count: 0,
		};

		Ok(writer)
	}
}

impl TextWriter {
	pub fn write_line(&mut self, line: &[u8]) {
		self.count += 1;

		if let Err(_) = self.file.write_all(&[line, b"\n"].concat()) {
			panic!("Could not write to text file at line {}.", self.count);
		}
	}
}
