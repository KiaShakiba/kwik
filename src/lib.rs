/*
 * Copyright (c) Kia Shakiba
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

#![allow(incomplete_features)]
#![feature(generic_const_exprs)]
#![feature(associated_type_bounds)]

pub mod utils;
pub mod fmt;
pub mod math;
pub mod mem;

pub mod file_reader;
pub use crate::file_reader::FileReader;

pub mod text_reader;
pub use crate::text_reader::TextReader;

pub mod csv_reader;
pub use crate::csv_reader::{
	CsvReader,
	Row as CsvReaderRow,
};

pub mod binary_reader;
pub use crate::binary_reader::{
	BinaryReader,
	SizedChunk,
	Chunk as BinaryReaderChunk,
};

pub mod file_writer;
pub use crate::file_writer::FileWriter;

pub mod text_writer;
pub use crate::text_writer::TextWriter;

pub mod csv_writer;
pub use crate::csv_writer::{
	CsvWriter,
	Row as CsvWriterRow,
};

pub mod binary_writer;
pub use crate::binary_writer::{
	BinaryWriter,
	Chunk as BinaryWriterChunk,
};

pub mod progress;
pub use crate::progress::{Progress, Tag as ProgressTag};

pub mod genetic;
pub use crate::genetic::Genetic;

pub mod table;
pub use crate::table::{
	Table,
	Row as TableRow,
	Align as TableRowAlign,
	Style as TableRowStyle,
};
