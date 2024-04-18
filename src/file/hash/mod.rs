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
/// use kwik::file::hash::sha256sum;
///
/// match sha256sum("/path/to/file") {
///     Ok(digest) => println!("{digest}"),
///     Err(err) => println!("{:?}", err),
/// }
/// ```
pub fn sha256sum<P>(path: P) -> Result<String, Error>
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
/// use kwik::file::hash::sha512sum;
///
/// match sha512sum("/path/to/file") {
///     Ok(digest) => println!("{digest}"),
///     Err(err) => println!("{:?}", err),
/// }
/// ```
pub fn sha512sum<P>(path: P) -> Result<String, Error>
where
	P: AsRef<Path>,
{
	let mut hasher = Sha512::new();
	let mut file = File::open(path)?;

	io::copy(&mut file, &mut hasher)?;

	Ok(format!("{:x}", hasher.finalize()))
}
