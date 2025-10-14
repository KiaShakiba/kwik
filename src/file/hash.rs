/*
 * Copyright (c) Kia Shakiba
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

use std::{fs::File, io, path::Path};

use md5::Md5;
use sha2::{Digest, Sha256, Sha512};

/// Computes the SHA256 hash of the file at the provided path.
///
/// # Examples
/// ```
/// use kwik::file::hash::sha256sum;
///
/// match sha256sum("/path/to/file") {
///     Ok(digest) => println!("{digest}"),
///     Err(err) => println!("{err:?}"),
/// }
/// ```
pub fn sha256sum<P>(path: P) -> io::Result<String>
where
	P: AsRef<Path>,
{
	let mut file = File::open(path)?;
	let mut hasher = Sha256::new();

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
///     Err(err) => println!("{err:?}"),
/// }
/// ```
pub fn sha512sum<P>(path: P) -> io::Result<String>
where
	P: AsRef<Path>,
{
	let mut file = File::open(path)?;
	let mut hasher = Sha512::new();

	io::copy(&mut file, &mut hasher)?;

	Ok(format!("{:x}", hasher.finalize()))
}

/// Computes the MD5 hash of the file at the provided path.
///
/// # Examples
/// ```
/// use kwik::file::hash::md5sum;
///
/// match md5sum("/path/to/file") {
///     Ok(digest) => println!("{digest}"),
///     Err(err) => println!("{err:?}"),
/// }
/// ```
pub fn md5sum<P>(path: P) -> io::Result<String>
where
	P: AsRef<Path>,
{
	let mut file = File::open(path)?;
	let mut hasher = Md5::new();

	io::copy(&mut file, &mut hasher)?;

	Ok(format!("{:x}", hasher.finalize()))
}
