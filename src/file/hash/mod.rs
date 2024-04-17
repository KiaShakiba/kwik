/*
 * Copyright (c) Kia Shakiba
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

use std::{
	io::{self, Error},
	path::Path,
	fs::File,
};

use sha2::{Digest, Sha256, Sha512};

/// Computes the SHA256 hash of the file at the provided path.
///
/// # Examples
/// ```
/// use kwik::file::hash::sha256;
///
/// match sha256("/path/to/file") {
///     Ok(digest) => println!("{digest}"),
///     Err(err) => println!("{:?}", err),
/// }
/// ```
pub fn sha256<P>(path: P) -> Result<String, Error>
where
	P: AsRef<Path>,
{
	let mut hasher = Sha256::new();
	let mut file = File::open(path)?;

	io::copy(&mut file, &mut hasher)?;

	Ok(format!("{:x}", hasher.finalize()))
}

/// Computes the SHA512 hash of the file at the provided path.
///
/// # Examples
/// ```
/// use kwik::file::hash::sha512;
///
/// match sha512("/path/to/file") {
///     Ok(digest) => println!("{digest}"),
///     Err(err) => println!("{:?}", err),
/// }
/// ```
pub fn sha512<P>(path: P) -> Result<String, Error>
where
	P: AsRef<Path>,
{
	let mut hasher = Sha512::new();
	let mut file = File::open(path)?;

	io::copy(&mut file, &mut hasher)?;

	Ok(format!("{:x}", hasher.finalize()))
}
