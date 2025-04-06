/*
 * Copyright (c) Kia Shakiba
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

mod reader;
mod writer;

use std::mem;

/// Implementing this trait specifies the number of bytes each
/// chunk occupies in the binary file. The file will be read in chunks
/// of that size.
///
/// # Examples
/// ```
/// use kwik::file::binary::SizedChunk;
///
/// struct MyStruct {
///     // data fields
/// }
///
/// impl SizedChunk for MyStruct {
///     fn chunk_size() -> usize { 10 }
/// }
/// ```
pub trait SizedChunk {
	fn chunk_size() -> usize;
}

pub use crate::file::binary::{
	reader::{BinaryReader, ReadChunk, Iter, IntoIter},
	writer::{BinaryWriter, WriteChunk},
};

impl<T> SizedChunk for Option<T>
where
	T: SizedChunk,
{
	fn chunk_size() -> usize {
		T::chunk_size() + 1
	}
}

macro_rules! impl_sized_chunk_primitive {
	($T:ty) => {
		impl SizedChunk for $T {
			#[inline]
			fn chunk_size() -> usize {
				mem::size_of::<$T>()
			}
		}
	}
}

impl_sized_chunk_primitive!(u8);
impl_sized_chunk_primitive!(i8);
impl_sized_chunk_primitive!(u16);
impl_sized_chunk_primitive!(i16);
impl_sized_chunk_primitive!(u32);
impl_sized_chunk_primitive!(i32);
impl_sized_chunk_primitive!(u64);
impl_sized_chunk_primitive!(i64);
impl_sized_chunk_primitive!(u128);
impl_sized_chunk_primitive!(i128);
impl_sized_chunk_primitive!(usize);
impl_sized_chunk_primitive!(isize);
impl_sized_chunk_primitive!(f32);
impl_sized_chunk_primitive!(f64);
impl_sized_chunk_primitive!(char);
impl_sized_chunk_primitive!(bool);
