/*
 * Copyright (c) Kia Shakiba
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

use std::{
	path::Path,
	fs::File,
	io::{BufWriter, Write, Error},
	marker::PhantomData,
};

use crate::file::{
	FileWriter,
	binary::SizedChunk,
};

/// Writes a binary file in chunks
pub struct BinaryWriter<T>
where
	T: WriteChunk,
{
	file: BufWriter<File>,
	buf: Vec<u8>,
	count: u64,

	_marker: PhantomData<T>,
}

/// Implementing this trait allows the binary writer to convert the
/// struct into writable chunks.
pub trait WriteChunk: SizedChunk {
	/// Fills the supplied buffer with binary data to be written
	/// to the file as a chunk.
	///
	/// # Examples
	/// ```
	/// use std::io::Error;
	/// use kwik::file::binary::{WriteChunk, SizedChunk};
	///
	/// struct MyStruct {
	///     // data fields
	/// }
	///
	/// impl WriteChunk for MyStruct {
	///     fn as_chunk(&self, buf: &mut Vec<u8>) -> Result<(), Error>
	///     where
	///         Self: Sized,
	///     {
	///         // modify `buf`
	///         Ok(())
	///     }
	/// }
	///
	/// impl SizedChunk for MyStruct {
	///     fn size() -> usize { 0 }
	/// }
	/// ```
	///
	/// # Errors
	///
	/// This function will return an error if the chunk could not be created.
	fn as_chunk(&self, buf: &mut Vec<u8>) -> Result<(), Error>;
}

impl<T> FileWriter for BinaryWriter<T>
where
	T: WriteChunk,
{
	/// Opens the file at the supplied path. If the file could not be
	/// opened, returns an error result.
	fn new<P>(path: P) -> Result<Self, Error>
	where
		Self: Sized,
		P: AsRef<Path>,
	{
		let opened_file = File::create(path)?;

		let writer = BinaryWriter {
			file: BufWriter::new(opened_file),
			buf: Vec::<u8>::with_capacity(T::size()),
			count: 0,

			_marker: PhantomData,
		};

		Ok(writer)
	}
}

impl<T> BinaryWriter<T>
where
	T: WriteChunk,
{
	/// Writes one chunk to the binary file, as specified by the chunk size.
	///
	/// # Examples
	/// ```no_run
	/// use std::io::Error;
	///
	/// use kwik::file::{
	///     FileWriter,
	///     binary::{BinaryWriter, WriteChunk, SizedChunk},
	/// };
	///
	/// let mut reader = BinaryWriter::<MyStruct>::new("/path/to/file").unwrap();
	///
	/// reader.write_chunk(&MyStruct { data: 0 });
	///
	/// struct MyStruct {
	///     // data fields
	///     data: u32,
	/// }
	///
	/// impl WriteChunk for MyStruct {
	///     fn as_chunk(&self, buf: &mut Vec<u8>) -> Result<(), Error>
	///     where
	///         Self: Sized,
	///     {
	///         // modify `buf`
	///         Ok(())
	///     }
	/// }
	///
	/// impl SizedChunk for MyStruct {
	///     fn size() -> usize { 4 }
	/// }
	/// ```
	#[inline]
	pub fn write_chunk(&mut self, object: &T) {
		self.buf.clear();
		self.count += 1;

		if object.as_chunk(&mut self.buf).is_err() {
			panic!("Error converting object {} to chunk", self.count);
		}

		if self.buf.len() != T::size() {
			panic!("Invalid chunk size at chunk {}", self.count);
		}

		if self.file.write_all(&self.buf).is_err() {
			panic!("Could not write to binary file at chunk {}", self.count);
		}
	}
}
