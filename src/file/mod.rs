/*
 * Copyright (c) Kia Shakiba
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

pub mod binary;
pub mod text;
pub mod csv;
pub mod hash;

use std::{
	io,
	path::Path,
	fs::File,
};

pub trait FileReader {
	/// Opens the file at the supplied path. If the file could not be
	/// opened, returns an error result.
	fn from_path<P>(path: P) -> io::Result<Self>
	where
		Self: Sized,
		P: AsRef<Path>,
	;

	/// Opens the reader with the supplied file.
	fn from_file(file: File) -> io::Result<Self>
	where
		Self: Sized,
	;

	/// Returns the number of bytes in the opened file.
	fn size(&self) -> u64;
}

pub trait FileWriter {
	/// Opens the file at the supplied path. If the file does not exist,
	/// a new file is created. If it does exist, the file is truncated.
	/// If the file could not be opened, returns an error result.
	fn from_path<P>(path: P) -> io::Result<Self>
	where
		Self: Sized,
		P: AsRef<Path>,
	;

	/// Opens the writer with the supplied file.
	fn from_file(file: File) -> io::Result<Self>
	where
		Self: Sized,
	;

	/// Flushes the current buffer to the file. If the buffer could not
	/// be flushed, returns an error result.
	fn flush(&mut self) -> io::Result<()>;
}
