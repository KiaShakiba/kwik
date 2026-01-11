/*
 * Copyright (c) Kia Shakiba
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

use std::{
	fs::File,
	io::{self, BufWriter, Seek, SeekFrom, Write},
	marker::PhantomData,
	path::Path,
};

use crate::file::{FileWriter, binary::SizedChunk};

/// Writes a binary file in chunks.
pub struct BinaryWriter<T>
where
	T: WriteChunk,
{
	file:  BufWriter<File>,
	buf:   Vec<u8>,
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
	/// use std::io;
	/// use kwik::file::binary::{WriteChunk, SizedChunk};
	///
	/// struct MyStruct {
	///     // data fields
	/// }
	///
	/// impl WriteChunk for MyStruct {
	///     fn as_chunk(&self, buf: &mut Vec<u8>) -> io::Result<()>
	///     where
	///         Self: Sized,
	///     {
	///         // modify `buf`
	///         Ok(())
	///     }
	/// }
	///
	/// impl SizedChunk for MyStruct {
	///     fn chunk_size() -> usize { 0 }
	/// }
	/// ```
	///
	/// # Errors
	///
	/// This function will return an error if the chunk could not be created.
	fn as_chunk(&self, buf: &mut Vec<u8>) -> io::Result<()>;
}

impl<T> FileWriter for BinaryWriter<T>
where
	T: WriteChunk,
{
	fn from_path<P>(path: P) -> io::Result<Self>
	where
		Self: Sized,
		P: AsRef<Path>,
	{
		BinaryWriter::from_file(File::create(path)?)
	}

	fn from_file(file: File) -> io::Result<Self>
	where
		Self: Sized,
	{
		let writer = BinaryWriter {
			file:  BufWriter::new(file),
			buf:   Vec::<u8>::with_capacity(T::chunk_size()),
			count: 0,

			_marker: PhantomData,
		};

		Ok(writer)
	}

	fn flush(&mut self) -> io::Result<()> {
		self.file.flush()
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
	/// use std::io;
	///
	/// use kwik::file::{
	///     FileWriter,
	///     binary::{BinaryWriter, WriteChunk, SizedChunk},
	/// };
	///
	/// let mut reader = BinaryWriter::<MyStruct>::from_path("/path/to/file").unwrap();
	///
	/// reader.write_chunk(&MyStruct { data: 0 }).unwrap();
	///
	/// struct MyStruct {
	///     // data fields
	///     data: u32,
	/// }
	///
	/// impl WriteChunk for MyStruct {
	///     fn as_chunk(&self, buf: &mut Vec<u8>) -> io::Result<()>
	///     where
	///         Self: Sized,
	///     {
	///         // modify `buf`
	///         Ok(())
	///     }
	/// }
	///
	/// impl SizedChunk for MyStruct {
	///     fn chunk_size() -> usize { 4 }
	/// }
	/// ```
	///
	/// # Errors
	///
	/// This function will return an error if the chunk could not be written.
	#[inline]
	pub fn write_chunk(&mut self, object: &T) -> io::Result<()> {
		self.buf.clear();
		self.count += 1;

		object.as_chunk(&mut self.buf)?;

		if self.buf.len() != T::chunk_size() {
			let message = format!("Invalid chunk size at chunk {}", self.count);
			return Err(io::Error::new(io::ErrorKind::InvalidData, message));
		}

		self.file.write_all(&self.buf)
	}
}

impl<T> Seek for BinaryWriter<T>
where
	T: WriteChunk,
{
	fn seek(&mut self, pos: SeekFrom) -> io::Result<u64> {
		self.file.seek(pos)
	}
}

impl<T> WriteChunk for Option<T>
where
	T: WriteChunk,
{
	fn as_chunk(&self, buf: &mut Vec<u8>) -> io::Result<()> {
		self.is_some().as_chunk(buf)?;

		match self {
			Some(value) => value.as_chunk(buf)?,

			None => {
				let size = T::chunk_size();
				let zeros = std::iter::repeat_n(0, size);

				buf.extend(zeros);
			},
		};

		Ok(())
	}
}

impl<T, E> WriteChunk for Result<T, E>
where
	T: WriteChunk,
	E: WriteChunk,
{
	fn as_chunk(&self, buf: &mut Vec<u8>) -> io::Result<()> {
		self.is_ok().as_chunk(buf)?;

		match self {
			Ok(value) => {
				value.as_chunk(buf)?;

				if T::chunk_size() > E::chunk_size() {
					let zeros = std::iter::repeat_n(0, T::chunk_size() - E::chunk_size());
					buf.extend(zeros);
				}
			},

			Err(err) => {
				err.as_chunk(buf)?;

				if E::chunk_size() > T::chunk_size() {
					let zeros = std::iter::repeat_n(0, E::chunk_size() - T::chunk_size());
					buf.extend(zeros);
				}
			},
		}

		Ok(())
	}
}

macro_rules! impl_write_chunk_primitive {
	(char) => {
		impl WriteChunk for char {
			#[inline]
			fn as_chunk(&self, buf: &mut Vec<u8>) -> io::Result<()> {
				let byte = *self as u8;
				byte.as_chunk(buf)
			}
		}
	};

	(bool) => {
		impl WriteChunk for bool {
			#[inline]
			fn as_chunk(&self, buf: &mut Vec<u8>) -> io::Result<()> {
				let byte: u8 = if *self { 1 } else { 0 };
				byte.as_chunk(buf)
			}
		}
	};

	($T:ty) => {
		impl WriteChunk for $T {
			#[inline]
			fn as_chunk(&self, buf: &mut Vec<u8>) -> io::Result<()> {
				buf.extend_from_slice(&self.to_le_bytes());
				Ok(())
			}
		}
	};
}

impl_write_chunk_primitive!(u8);
impl_write_chunk_primitive!(i8);
impl_write_chunk_primitive!(u16);
impl_write_chunk_primitive!(i16);
impl_write_chunk_primitive!(u32);
impl_write_chunk_primitive!(i32);
impl_write_chunk_primitive!(u64);
impl_write_chunk_primitive!(i64);
impl_write_chunk_primitive!(u128);
impl_write_chunk_primitive!(i128);
impl_write_chunk_primitive!(usize);
impl_write_chunk_primitive!(isize);
impl_write_chunk_primitive!(f32);
impl_write_chunk_primitive!(f64);
impl_write_chunk_primitive!(char);
impl_write_chunk_primitive!(bool);
