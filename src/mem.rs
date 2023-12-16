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
use sys_info::mem_info;

use crate::{
	file_reader::FileReader,
	text_reader::TextReader,
};

pub type Pid = u32;

#[derive(Debug, Error)]
pub enum MemError {
	#[error("stat for `{0}` not found")]
	InvalidStat(String),

	#[error("could not find system memory information")]
	MemInfo,

	#[error("could not clear memory HWM")]
	Clear,
}

/// Returns a parsed status member from the process status file.
/// If a pid is supplied, the status member of that process is returned;
/// otherwise, the status member of the current process is returned.
///
/// If the status member could not be found, an error result is returned.
///
/// # Examples
/// ```
/// use kwik::mem;
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
pub fn stat<T>(key: &str, pid: Option<&Pid>) -> Result<T, MemError>
where
	T: FromStr + Copy
{
	let path = match pid {
		Some(pid) => format!("/proc/{pid}/status"),
		None => String::from("/proc/self/status"),
	};

	let mut reader = TextReader::new(&path)
		.expect("Could not open process status file.");

	while let Some(line) = reader.read_line() {
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
/// If the high water mark could not be determined, an error result
/// is returned.
///
/// # Examples
/// ```
/// use kwik::mem;
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
pub fn hwm(pid: Option<&Pid>) -> Result<u64, MemError> {
	stat::<u64>("VmHWM", pid).map(|value| value * 1024)
}

/// Returns the resident set size of the supplied pid in bytes. If no pid
/// is supplied, the resident set size of the current process is returned.
///
/// If the resident set size could not be determined, an error result
/// is returned.
///
/// # Examples
/// ```
/// use kwik::mem;
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
pub fn rss(pid: Option<&Pid>) -> Result<u64, MemError> {
	stat::<u64>("VmRSS", pid).map(|value| value * 1024)
}

/// Returns the total physical memory of the system in bytes.
///
/// If the memory size could not be determined, an error result
/// is returned.
///
/// # Examples
/// ```
/// use kwik::mem;
///
/// match mem::sys() {
///     Ok(value) => {
///         // process system memory size
///     },
///
///     Err(err) => {
///         // handle error
///     }
/// }
/// ```
pub fn sys() -> Result<u64, MemError> {
	match mem_info() {
		Ok(info) => Ok(info.total * 1024),
		Err(_) => Err(MemError::MemInfo)
	}
}

/// Clears the memory refs of the supplied pid. If no pid is supplied,
/// clears the memory refs of the current process.
///
/// If the memory refs could not be cleared, an error result is returned.
///
/// # Examples
/// ```
/// use kwik::mem;
///
/// // clears the memory refs of the current process
/// if let Err(err) = mem::clear(None) {
///     // handle error
/// }
/// ```
pub fn clear(pid: Option<&Pid>) -> Result<(), MemError> {
	let command = match pid {
		Some(pid) => format!("echo 1 > /proc/{pid}/clear_refs"),
		None => String::from("echo 1 > /proc/self/clear_refs"),
	};

	let error = "Could not clear memory refs.";

	let status = Command::new("sh")
		.arg("-c")
		.arg(command)
		.status()
		.expect(error);

	match status.success() {
		true => Ok(()),
		false => Err(MemError::Clear),
	}
}

/// Returns the size of the supplied value in bytes.
///
/// # Examples
/// ```
/// use kwik::mem;
///
/// let num: u32 = 5;
/// let size = mem::size_of(&num);
///
/// assert_eq!(size, 4);
/// ```
pub fn size_of<T>(value: &T) -> usize {
	mem::size_of_val(value)
}

/// Returns the size of the supplied vector in bytes.
///
/// # Examples
/// ```
/// use kwik::mem;
///
/// let values = vec![0u32, 1, 2, 3];
/// let size = mem::size_of_vec(&values);
///
/// assert_eq!(size, 40);
/// ```
pub fn size_of_vec<T>(value: &Vec<T>) -> usize {
	let container_size = size_of(value);

	if value.is_empty() {
		return container_size;
	}

	container_size + value.len() * size_of(&value[0])
}
