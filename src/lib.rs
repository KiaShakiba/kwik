/*
 * Copyright (c) Kia Shakiba
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

pub mod utils;
pub mod fmt;
pub mod math;
pub mod mem;

pub mod file_reader;
pub use file_reader::FileReader;

pub mod text_reader;
pub use text_reader::TextReader;

pub mod csv_reader;
pub use csv_reader::{
	CsvReader,
	Row as CsvReaderRow,
};

pub mod binary_reader;
pub use binary_reader::{
	BinaryReader,
	SizedChunk,
	Chunk as BinaryReaderChunk,
};

pub mod file_writer;
pub use file_writer::FileWriter;

pub mod text_writer;
pub use text_writer::TextWriter;

pub mod csv_writer;
pub use csv_writer::{
	CsvWriter,
	Row as CsvWriterRow,
};

pub mod binary_writer;
pub use binary_writer::{
	BinaryWriter,
	Chunk as BinaryWriterChunk,
};

pub mod progress;
pub use progress::{Progress, Tag as ProgressTag};
