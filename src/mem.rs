/*
 * Copyright (c) Kia Shakiba
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

use std::str::FromStr;
use std::process::Command;
use sys_info::mem_info;
use crate::file_reader::FileReader;
use crate::text_reader::TextReader;

pub type Pid = u32;

pub fn stat<'a, T>(key: &'a str, pid: Option<&Pid>) -> Option<T>
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
				Ok(value) => Some(*value),
				Err(_) => None,
			}
		}
	}

	None
}

pub fn hwm(pid: Option<&Pid>) -> Option<u64> {
	match stat::<u64>("VmHWM", pid) {
		Some(value) => Some(value * 1024),
		None => None,
	}
}

pub fn rss(pid: Option<&Pid>) -> Option<u64> {
	match stat::<u64>("VmRSS", pid) {
		Some(value) => Some(value * 1024),
		None => None,
	}
}

pub fn sys() -> u64 {
	match mem_info() {
		Ok(info) => info.total * 1024,

		Err(_) => {
			panic!("Could not get system memory.");
		}
	}
}

pub fn clear(pid: Option<&Pid>) {
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

	if !status.success() {
		panic!("{}", error);
	}
}
