/*
 * Copyright (c) Kia Shakiba
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

use thiserror::Error;
use sysinfo::{System, Pid as SysPid, ProcessesToUpdate};
use crate::sys::Pid;

#[derive(Debug, Error)]
pub enum CpuError {
	#[error("process with id `{0}` not found")]
	InvalidPid(u32),
}

/// Returns the CPU usage of the supplied pid between [0, 1], normalized
/// to the number of CPUs of the system.
///
/// # Examples
/// ```
/// use kwik::sys::cpu;
///
/// // returns the CPU usage of the current process
/// match cpu::usage(None) {
///     Ok(value) => {
///         // process CPU usage
///     },
///
///     Err(err) => {
///         // handle error
///     },
/// }
/// ```
///
/// # Errors
///
/// This function will return an error if an invalid pid is supplied.
pub fn usage(pid: Option<Pid>) -> Result<f64, CpuError> {
	let pid = pid.unwrap_or(std::process::id());
	let sys_pid = SysPid::from_u32(pid);
	let refresh_pid = &[sys_pid];

	let mut sys = System::new_all();
	let process = ProcessesToUpdate::Some(refresh_pid);

	sys.refresh_processes(process, true);

	match sys.process(sys_pid) {
		Some(process) => {
			let usage = process.cpu_usage() as f64
				/ sys.cpus().len() as f64
				/ 100.0;

			Ok(usage)
		},

		None => Err(CpuError::InvalidPid(pid)),
	}
}
