/*
 * Copyright (c) Kia Shakiba
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

use std::{
	mem,
	str::FromStr,
	process::Command,
};

use thiserror::Error;
use sysinfo::System;

use crate::{
	file::{
		FileReader,
		text::TextReader,
	},
	sys::Pid,
};

#[derive(Debug, Error)]
pub enum MemError {
	#[error("stat for `{0}` not found")]
	InvalidStat(String),

	#[error("could not find system memory information")]
	MemInfo,

	#[error("could not clear memory HWM")]
	Clear,

	#[error("an internal error occurred")]
	Internal,
}

/// Returns a parsed status member from the process status file.
/// If a pid is supplied, the status member of that process is returned;
/// otherwise, the status member of the current process is returned.
///
/// # Examples
/// ```
/// use kwik::sys::mem;
///
/// // returns the "VmHWM" status member of the current process
/// match mem::stat::<u64>("VmHWM", None) {
///     Ok(value) => {
///         // process value
///     },
///
///     Err(err) => {
///         // handle error
///     }
/// }
/// ```
///
/// # Errors
///
/// This function returns an error if the status member could not be found.
pub fn stat<T>(key: &str, pid: Option<Pid>) -> Result<T, MemError>
where
	T: FromStr + Copy,
{
	let path = match pid {
		Some(pid) => format!("/proc/{pid}/status"),
		None => String::from("/proc/self/status"),
	};

	let reader = TextReader::new(path)
		.map_err(|_| MemError::Internal)?;

	for line in reader {
		if line.starts_with(key) {
			let parsed = &line[key.len() + 1..line.len() - 2]
				.trim()
				.parse::<T>();

			return match parsed {
				Ok(value) => Ok(*value),
				Err(_) => Err(MemError::InvalidStat(key.to_string())),
			}
		}
	}

	Err(MemError::InvalidStat(key.to_string()))
}

/// Returns the high water mark of the supplied pid in bytes. If no pid
/// is supplied, the high water mark of the current process is returned.
///
/// # Examples
/// ```
/// use kwik::sys::mem;
///
/// // returns the high water mark of the current process
/// match mem::hwm(None) {
///     Ok(value) => {
///         // process high water mark
///     },
///
///     Err(err) => {
///         // handle error
///     }
/// }
/// ```
///
/// # Errors
///
/// This function returns an error if the high water mark could not
/// be determined.
#[inline]
pub fn hwm(pid: Option<Pid>) -> Result<u64, MemError> {
	stat::<u64>("VmHWM", pid).map(|value| value * 1024)
}

/// Returns the resident set size of the supplied pid in bytes. If no pid
/// is supplied, the resident set size of the current process is returned.
///
/// # Examples
/// ```
/// use kwik::sys::mem;
///
/// // returns the resident set size of the current process
/// match mem::rss(None) {
///     Ok(value) => {
///         // process high water mark
///     },
///
///     Err(err) => {
///         // handle error
///     }
/// }
/// ```
///
/// # Errors
///
/// This function returns an error if the resident set size could not
/// be determined.
#[inline]
pub fn rss(pid: Option<Pid>) -> Result<u64, MemError> {
	stat::<u64>("VmRSS", pid).map(|value| value * 1024)
}

/// Returns the total physical memory of the system in bytes.
///
/// # Examples
/// ```
/// use kwik::sys::mem;
///
/// assert!(mem::total() > 0);
/// ```
///
/// # Errors
///
/// This function will return an error if the memory size
/// could not be determined.
#[inline]
#[must_use]
pub fn total() -> u64 {
	let mut sys = System::new();

	sys.refresh_memory();
	sys.total_memory()
}

/// Clears the memory refs of the supplied pid. If no pid is supplied,
/// clears the memory refs of the current process.
///
/// # Examples
/// ```
/// use kwik::sys::mem;
///
/// // clears the memory refs of the current process
/// if let Err(err) = mem::clear(None) {
///     // handle error
/// }
/// ```
///
/// # Errors
///
/// This function returns an error if the memory refs could not be cleared.
pub fn clear(pid: Option<Pid>) -> Result<(), MemError> {
	let command = match pid {
		Some(pid) => format!("echo 1 > /proc/{pid}/clear_refs"),
		None => String::from("echo 1 > /proc/self/clear_refs"),
	};

	let status = Command::new("sh")
		.arg("-c")
		.arg(command)
		.status()
		.map_err(|_| MemError::Internal)?;

	match status.success() {
		true => Ok(()),
		false => Err(MemError::Clear),
	}
}

/// Returns the size of the supplied value in bytes.
///
/// # Examples
/// ```
/// use kwik::sys::mem;
///
/// let num: u32 = 5;
/// let size = mem::size_of(&num);
///
/// assert_eq!(size, 4);
/// ```
#[inline]
#[must_use]
pub fn size_of<T>(value: &T) -> usize {
	mem::size_of_val(value)
}

/// Returns the size of the supplied vector in bytes.
///
/// # Examples
/// ```
/// use kwik::sys::mem;
///
/// let values = vec![0u32, 1, 2, 3];
/// let size = mem::size_of_vec(&values);
///
/// assert_eq!(size, 40);
/// ```
#[inline]
#[must_use]
pub fn size_of_vec<T>(value: &Vec<T>) -> usize {
	let container_size = size_of(value);

	if value.is_empty() {
		return container_size;
	}

	container_size + value.len() * size_of(&value[0])
}
