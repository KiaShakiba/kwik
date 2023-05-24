/*
 * Copyright (c) Kia Shakiba
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

use std::str::FromStr;
use std::process::Command;
use std::mem;
use sys_info::mem_info;
use crate::file_reader::FileReader;
use crate::text_reader::TextReader;

pub type Pid = u32;

pub enum Error {
	InvalidStat(String),
	MemInfo,
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
/// // returns the "VmHWM" status member of the current process
/// match stat::<u64>("VmHWM", None) {
/// 	Ok(value) => {
///			// process value
///		},
///
///		Err(err) => {
///			// handle error
///		}
/// }
/// ```
pub fn stat<'a, T>(key: &'a str, pid: Option<&Pid>) -> Result<T, Error>
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
				Err(_) => Err(Error::InvalidStat(key.to_string())),
			}
		}
	}

	Err(Error::InvalidStat(key.to_string()))
}

/// Returns the high water mark of the supplied pid in bytes. If no pid
/// is supplied, the high water mark of the current process is returned.
///
/// If the high water mark could not be determined, an error result
/// is returned.
///
/// # Examples
/// ```
/// // returns the high water mark of the current process
/// match hwm(None) {
/// 	Ok(value) => {
///			// process high water mark
///		},
///
///		Err(err) => {
///			// handle error
///		}
/// }
/// ```
pub fn hwm(pid: Option<&Pid>) -> Result<u64, Error> {
	match stat::<u64>("VmHWM", pid) {
		Ok(value) => Ok(value * 1024),
		Err(error) => Err(error),
	}
}

/// Returns the resident set size of the supplied pid in bytes. If no pid
/// is supplied, the resident set size of the current process is returned.
///
/// If the resident set size could not be determined, an error result
/// is returned.
///
/// # Examples
/// ```
/// // returns the resident set size of the current process
/// match rss(None) {
/// 	Ok(value) => {
///			// process high water mark
///		},
///
///		Err(err) => {
///			// handle error
///		}
/// }
/// ```
pub fn rss(pid: Option<&Pid>) -> Result<u64, Error> {
	match stat::<u64>("VmRSS", pid) {
		Ok(value) => Ok(value * 1024),
		Err(error) => Err(error),
	}
}

/// Returns the total physical memory of the system in bytes.
///
/// If the memory size could not be determined, an error result
/// is returned.
///
/// # Examples
/// ```
/// match sys() {
/// 	Ok(value) => {
///			// process system memory size
///		},
///
///		Err(err) => {
///			// handle error
///		}
/// }
/// ```
pub fn sys() -> Result<u64, Error> {
	match mem_info() {
		Ok(info) => Ok(info.total * 1024),
		Err(_) => Err(Error::MemInfo)
	}
}

/// Clears the memory refs of the supplied pid. If no pid is supplied,
/// clears the memory refs of the current process.
///
/// If the memory refs could not be cleared, an error result is returned.
///
/// # Examples
/// ```
/// // clears the memory refs of the current process
/// if let Err(err) = clear(None) {
/// 	// handle error
/// }
/// ```
pub fn clear(pid: Option<&Pid>) -> Result<(), Error> {
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
		false => Err(Error::Clear),
	}
}

/// Returns the size of the supplied value in bytes.
///
/// # Examples
/// ```
/// let num: u32 = 5;
/// let size = size_of(&num);
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
/// let values = vec![0u32, 1, 2, 3];
/// let size = size_of_vec(&num);
///
/// assert_eq!(size, 16);
/// ```
pub fn size_of_vec<T>(value: &Vec<T>) -> usize {
	let container_size = size_of(value);

	if value.is_empty() {
		return container_size;
	}

	container_size + value.len() * size_of(&value[0])
}
