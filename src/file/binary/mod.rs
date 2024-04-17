/*
 * Copyright (c) Kia Shakiba
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

mod reader;
mod writer;

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
///     fn size() -> usize { 10 }
/// }
/// ```
pub trait SizedChunk {
	fn size() -> usize;
}

pub use crate::file::binary::{
	reader::{BinaryReader, ReadChunk, Iter, IntoIter},
	writer::{BinaryWriter, WriteChunk},
};
