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

pub fn hwm(pid: Option<&Pid>) -> Result<u64, Error> {
	match stat::<u64>("VmHWM", pid) {
		Ok(value) => Ok(value * 1024),
		Err(error) => Err(error),
	}
}

pub fn rss(pid: Option<&Pid>) -> Result<u64, Error> {
	match stat::<u64>("VmRSS", pid) {
		Ok(value) => Ok(value * 1024),
		Err(error) => Err(error),
	}
}

pub fn sys() -> Result<u64, Error> {
	match mem_info() {
		Ok(info) => Ok(info.total * 1024),
		Err(_) => Err(Error::MemInfo)
	}
}

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

pub fn size_of<T>(value: &T) -> usize {
	mem::size_of_val(value)
}

pub fn size_of_vec<T>(value: &Vec<T>) -> usize {
	let container_size = size_of(value);

	if value.is_empty() {
		return container_size;
	}

	container_size + value.len() * size_of(&value[0])
}
