/*
 * Copyright (c) Kia Shakiba
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

use std::{
	fs::File,
	io::{BufWriter, Write, Error, ErrorKind},
	marker::PhantomData,
};

pub use crate::{
	file_writer::FileWriter,
	binary_reader::SizedChunk,
};

pub struct BinaryWriter<T>
where
	T: Chunk,
{
	file: BufWriter<File>,
	buf: Vec<u8>,
	count: u64,

	_marker: PhantomData<T>,
}

pub trait Chunk: SizedChunk {
	fn as_chunk(&self, _: &mut Vec<u8>) -> Result<(), Error>;
}

impl<T> FileWriter for BinaryWriter<T>
where
	T: Chunk,
{
	fn new(path: &str) -> Result<Self, Error> where Self: Sized {
		let Ok(opened_file) = File::create(path) else {
			return Err(Error::new(
				ErrorKind::PermissionDenied,
				"Could not create binary file."
			));
		};

		let writer = BinaryWriter {
			file: BufWriter::new(opened_file),
			buf: Vec::<u8>::with_capacity(T::SIZE),
			count: 0,

			_marker: PhantomData,
		};

		Ok(writer)
	}
}

impl<T> BinaryWriter<T>
where
	T: Chunk,
{
	pub fn write_chunk(&mut self, object: &T) {
		self.buf.clear();
		self.count += 1;

		if object.as_chunk(&mut self.buf).is_err() {
			panic!("Error converting object {} to chunk", self.count);
		}

		if self.file.write_all(&self.buf).is_err() {
			panic!("Could not write to binary file at chunk {}.", self.count);
		}
	}
}
