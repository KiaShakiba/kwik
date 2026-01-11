/*
 * Copyright (c) Kia Shakiba
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

use std::{
	fs::File,
	io::{self, BufReader, Read, Seek, SeekFrom},
	marker::PhantomData,
	path::Path,
};

use crate::file::{FileReader, binary::SizedChunk};

/// Reads a binary file in chunks.
pub struct BinaryReader<T>
where
	T: ReadChunk,
{
	file:  BufReader<File>,
	buf:   Box<[u8]>,
	count: u64,

	_marker: PhantomData<T>,
}

pub struct Iter<'a, T>
where
	T: ReadChunk,
{
	reader: &'a mut BinaryReader<T>,
}

pub struct IntoIter<T>
where
	T: ReadChunk,
{
	reader: BinaryReader<T>,
}

/// Implementing this trait allows the binary reader to parse chunks
/// of the binary file into the specified type.
pub trait ReadChunk: SizedChunk {
	/// Returns an instance of the implemented struct, given a chunk
	/// of the binary file. If the chunk could not be parsed, an
	/// error result is returned.
	///
	/// # Examples
	/// ```
	/// use std::io;
	/// use kwik::file::binary::{ReadChunk, SizedChunk};
	///
	/// struct MyStruct {
	///     // data fields
	/// }
	///
	/// impl ReadChunk for MyStruct {
	///     fn from_chunk(chunk: &[u8]) -> io::Result<Self>
	///     where
	///         Self: Sized,
	///     {
	///         // parse the chunk and return an instance of `Self` on success
	///         Ok(MyStruct {})
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
	/// This function will return an error if the chunk could not be parsed.
	fn from_chunk(buf: &[u8]) -> io::Result<Self>
	where
		Self: Sized;
}

impl<T> FileReader for BinaryReader<T>
where
	T: ReadChunk,
{
	fn from_path<P>(path: P) -> io::Result<Self>
	where
		Self: Sized,
		P: AsRef<Path>,
	{
		BinaryReader::from_file(File::open(path)?)
	}

	fn from_file(file: File) -> io::Result<Self>
	where
		Self: Sized,
	{
		let reader = BinaryReader {
			file:  BufReader::new(file),
			buf:   vec![0; T::chunk_size()].into_boxed_slice(),
			count: 0,

			_marker: PhantomData,
		};

		Ok(reader)
	}

	#[inline]
	fn size(&self) -> u64 {
		let metadata = self
			.file
			.get_ref()
			.metadata()
			.expect("Could not get binary file's size");

		metadata.len()
	}
}

impl<T> BinaryReader<T>
where
	T: ReadChunk,
{
	/// Reads one chunk of the binary file, as specified by the chunk size,
	/// and returns a `Result` containing the parsed chunk. If the end of the
	/// file is reached, an `io::Error` is returned.
	///
	/// # Examples
	/// ```no_run
	/// use std::io;
	///
	/// use kwik::file::{
	///     FileReader,
	///     binary::{BinaryReader, ReadChunk, SizedChunk},
	/// };
	///
	/// let mut reader = BinaryReader::<MyStruct>::from_path("/path/to/file").unwrap();
	///
	/// while let Ok(object) = reader.read_chunk() {
	///     // do something with the object
	/// }
	///
	/// struct MyStruct {
	///     // data fields
	///     data: u32,
	/// }
	///
	/// impl ReadChunk for MyStruct {
	///     fn from_chunk(chunk: &[u8]) -> io::Result<Self>
	///     where
	///         Self: Sized,
	///     {
	///         // parse the chunk and return an instance of `Self` on success
	///         Ok(MyStruct { data: 0 })
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
	/// This function will return an error if the chunk could not be read.
	#[inline]
	pub fn read_chunk(&mut self) -> io::Result<T> {
		self.file.read_exact(&mut self.buf).and_then(|_| {
			self.count += 1;

			let object = T::from_chunk(&self.buf)?;
			Ok(object)
		})
	}

	/// Returns an iterator over the binary file. The iterator takes a mutable
	/// reference to `self` as it is iterating over a stream. This means
	/// performing the iteration modifies the reader's position in the file.
	///
	/// # Examples
	/// ```no_run
	/// use std::io;
	///
	/// use kwik::file::{
	///     FileReader,
	///     binary::{BinaryReader, ReadChunk, SizedChunk},
	/// };
	///
	/// let mut reader = BinaryReader::<MyStruct>::from_path("/path/to/file").unwrap();
	///
	/// for chunk in reader.iter() {
	///     // do something with the object
	/// }
	///
	/// struct MyStruct {
	///     // data fields
	///     data: u32,
	/// }
	///
	/// impl ReadChunk for MyStruct {
	///     fn from_chunk(chunk: &[u8]) -> io::Result<Self>
	///     where
	///         Self: Sized,
	///     {
	///         // parse the chunk and return an instance of `Self` on success
	///         Ok(MyStruct { data: 0 })
	///     }
	/// }
	///
	/// impl SizedChunk for MyStruct {
	///     fn chunk_size() -> usize { 4 }
	/// }
	/// ```
	#[inline]
	pub fn iter(&mut self) -> Iter<'_, T> {
		Iter {
			reader: self
		}
	}
}

impl<T> Seek for BinaryReader<T>
where
	T: ReadChunk,
{
	fn seek(&mut self, pos: SeekFrom) -> io::Result<u64> {
		self.file.seek(pos)
	}
}

impl<T> Iterator for Iter<'_, T>
where
	T: ReadChunk,
{
	type Item = T;

	fn next(&mut self) -> Option<Self::Item> {
		match self.reader.read_chunk() {
			Ok(chunk) => Some(chunk),
			Err(err) if err.kind() == io::ErrorKind::UnexpectedEof => None,

			Err(_) => panic!(
				"An error occurred on chunk {} when reading binary file",
				self.reader.count + 1,
			),
		}
	}
}

impl<T> IntoIterator for BinaryReader<T>
where
	T: ReadChunk,
{
	type Item = T;
	type IntoIter = IntoIter<T>;

	fn into_iter(self) -> Self::IntoIter {
		IntoIter {
			reader: self
		}
	}
}

impl<T> Iterator for IntoIter<T>
where
	T: ReadChunk,
{
	type Item = T;

	fn next(&mut self) -> Option<Self::Item> {
		match self.reader.read_chunk() {
			Ok(chunk) => Some(chunk),
			Err(err) if err.kind() == io::ErrorKind::UnexpectedEof => None,

			Err(_) => panic!(
				"An error occurred on chunk {} when reading binary file",
				self.reader.count + 1,
			),
		}
	}
}

impl<T> ReadChunk for Option<T>
where
	T: ReadChunk,
{
	fn from_chunk(buf: &[u8]) -> io::Result<Self>
	where
		Self: Sized,
	{
		if buf[0] != 0 {
			let value = T::from_chunk(&buf[1..])?;
			Ok(Some(value))
		} else {
			Ok(None)
		}
	}
}

impl<T, E> ReadChunk for Result<T, E>
where
	T: ReadChunk,
	E: ReadChunk,
{
	fn from_chunk(buf: &[u8]) -> io::Result<Self>
	where
		Self: Sized,
	{
		if buf[0] != 0 {
			let value = T::from_chunk(&buf[1..T::chunk_size() + 1])?;
			Ok(Ok(value))
		} else {
			let err = E::from_chunk(&buf[1..E::chunk_size() + 1])?;
			Ok(Err(err))
		}
	}
}

macro_rules! impl_read_chunk_primitive {
	(char) => {
		impl ReadChunk for char {
			#[inline]
			fn from_chunk(buf: &[u8]) -> io::Result<Self>
			where
				Self: Sized,
			{
				Ok(buf[0] as char)
			}
		}
	};

	(bool) => {
		impl ReadChunk for bool {
			#[inline]
			fn from_chunk(buf: &[u8]) -> io::Result<Self>
			where
				Self: Sized,
			{
				Ok(buf[0] != 0)
			}
		}
	};

	($T:ty) => {
		impl ReadChunk for $T {
			#[inline]
			fn from_chunk(buf: &[u8]) -> io::Result<Self>
			where
				Self: Sized,
			{
				let (buf, _) = buf.split_at(<$T>::chunk_size());
				let value = <$T>::from_le_bytes(buf.try_into().unwrap());

				Ok(value)
			}
		}
	};
}

impl_read_chunk_primitive!(u8);
impl_read_chunk_primitive!(i8);
impl_read_chunk_primitive!(u16);
impl_read_chunk_primitive!(i16);
impl_read_chunk_primitive!(u32);
impl_read_chunk_primitive!(i32);
impl_read_chunk_primitive!(u64);
impl_read_chunk_primitive!(i64);
impl_read_chunk_primitive!(u128);
impl_read_chunk_primitive!(i128);
impl_read_chunk_primitive!(usize);
impl_read_chunk_primitive!(isize);
impl_read_chunk_primitive!(f32);
impl_read_chunk_primitive!(f64);
impl_read_chunk_primitive!(char);
impl_read_chunk_primitive!(bool);
