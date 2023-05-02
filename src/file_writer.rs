/*
 * Copyright (c) Kia Shakiba
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

use std::io::Error;

pub trait FileWriter {
	fn new(_: &str) -> Result<Self, Error> where Self: Sized;
}
