/*
 * Copyright (c) Kia Shakiba
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

use std::time::Instant;

use sysinfo::{Pid as SysPid, ProcessRefreshKind, ProcessesToUpdate, System};
use thiserror::Error;

use crate::sys::Pid;

#[derive(Debug, Error)]
pub enum CpuError {
	#[error("process with id `{0}` not found")]
	InvalidPid(u32),
}

// A per-process CPU usage monitor.
pub struct CpuUsage {
	pid: SysPid,
	system: System,

	cached_total_usage: Option<f64>,
	last_refresh: Instant,
}

/// Returns an instance of `CpuUsage` which can be polled periodically to
/// get the CPU usage of the supplied pid. If no pid is supplied, the CPU
/// usage of the current process is tracked.
///
/// # Examples
/// ```
/// use kwik::sys::cpu;
///
/// // returns the CPU usage of the current process
/// match cpu::usage(None) {
///     Ok(mut cpu_usage) => {
///         assert!(cpu_usage.poll_total().is_ok());
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
pub fn usage(pid: Option<Pid>) -> Result<CpuUsage, CpuError> {
	let pid = pid.unwrap_or(std::process::id());

	let mut cpu_usage = CpuUsage::new(pid);
	cpu_usage.refresh_cached_usage()?;

	Ok(cpu_usage)
}

impl CpuUsage {
	fn new(pid: Pid) -> Self {
		CpuUsage {
			pid: SysPid::from_u32(pid),
			system: System::new_all(),

			cached_total_usage: None,
			last_refresh: Instant::now(),
		}
	}

	/// Returns the total system CPU usage between [0, 1] of the process.
	///
	/// # Examples
	/// ```
	/// use kwik::sys::cpu;
	///
	/// let mut usage = cpu::usage(None).unwrap();
	/// assert!(usage.poll_total().is_ok());
	/// ```
	pub fn poll_total(&mut self) -> Result<f64, CpuError> {
		self.refresh_cached_usage()?;
		Ok(self.cached_total_usage.unwrap())
	}

	/// Returns the total system CPU usage between [0, 1] of the process,
	/// normalized to the number of CPU cores.
	///
	/// # Examples
	/// ```
	/// use kwik::sys::cpu;
	///
	/// let mut usage = cpu::usage(None).unwrap();
	/// assert!(usage.poll_per_core().is_ok());
	/// ```
	pub fn poll_per_core(&mut self) -> Result<f64, CpuError> {
		let total_usage = self.poll_total()?;
		Ok(total_usage / self.system.cpus().len() as f64)
	}

	fn refresh_cached_usage(&mut self) -> Result<(), CpuError> {
		if self.cached_total_usage.is_some()
			&& self.last_refresh.elapsed()
				< sysinfo::MINIMUM_CPU_UPDATE_INTERVAL
		{
			return Ok(());
		}

		self.system.refresh_processes_specifics(
			ProcessesToUpdate::Some(&[self.pid]),
			true,
			ProcessRefreshKind::nothing().with_cpu(),
		);

		let Some(process) = self.system.process(self.pid) else {
			return Err(CpuError::InvalidPid(self.pid.as_u32()));
		};

		self.cached_total_usage = Some(process.cpu_usage() as f64 / 100.0);
		self.last_refresh = Instant::now();

		Ok(())
	}
}
