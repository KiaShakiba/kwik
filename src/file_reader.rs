/*
 * Copyright (c) Kia Shakiba
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

use std::{
	path::Path,
	io::Error,
};

pub trait FileReader {
	fn new<P>(_: P) -> Result<Self, Error>
	where
		Self: Sized,
		P: AsRef<Path>,
	;

	fn size(&self) -> u64;
}
