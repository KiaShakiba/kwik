/*
 * Copyright (c) Kia Shakiba
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

use std::{
	path::Path,
	fs::File,
	io::{LineWriter, Write, Error},
};

use crate::file::FileWriter;

pub struct TextWriter {
	file: LineWriter<File>,
	count: u64,
}

impl FileWriter for TextWriter {
	fn new<P>(path: P) -> Result<Self, Error>
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
	#[inline]
	pub fn write_line(&mut self, line: &[u8]) {
		self.count += 1;

		if self.file.write_all(&[line, b"\n"].concat()).is_err() {
			panic!("Could not write to text file at line {}.", self.count);
		}
	}
}
