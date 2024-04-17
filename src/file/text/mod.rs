/*
 * Copyright (c) Kia Shakiba
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

mod reader;
mod writer;

pub use crate::file::text::{
	reader::{TextReader, Iter, IntoIter},
	writer::TextWriter,
};
