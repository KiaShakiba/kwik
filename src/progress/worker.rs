/*
 * Copyright (c) Kia Shakiba
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

use std::{
	cmp::Ordering,
	io::{self, Write},
	sync::Arc,
	thread,
	time::Duration,
};

use crate::{
	fmt,
	progress::{
		ProgressTag,
		state::{LocalProgressState, ProgressState},
	},
};

const DRAW_LONG_DELAY: Duration = Duration::from_millis(100);
const DRAW_SHORT_DELAY: Duration = Duration::from_millis(10);

pub struct ProgressWorker {
	pub thread: Option<thread::JoinHandle<()>>,
}

impl ProgressWorker {
	pub fn new(state: Arc<ProgressState>) -> Self {
		let mut local_state = LocalProgressState::new(&state);
		let mut prev_amount = local_state.get_curr_progress_amount() as u8;

		let thread = Some(thread::spawn(move || {
			loop {
				if local_state.is_complete() {
					draw_final(&local_state).unwrap();
					break;
				}

				let curr_amount = local_state.get_curr_progress_amount() as u8;

				let delay = if curr_amount - prev_amount > 1 {
					DRAW_SHORT_DELAY
				} else {
					DRAW_LONG_DELAY
				};

				draw(&local_state).unwrap();
				prev_amount = curr_amount;

				thread::sleep(delay);
				local_state.update(&state);
			}
		}));

		ProgressWorker {
			thread,
		}
	}
}

fn draw(state: &LocalProgressState) -> io::Result<()> {
	let curr_amount = state.get_curr_progress_amount() as u8;
	let position = state.get_progress_position(curr_amount);
	let maybe_rate = state.get_rate();
	let eta = state.get_eta();
	let elapsed = state.get_total_duration();

	let mut writer = state.writer.lock();
	write!(writer, "\x1B[2K\r[")?;

	for i in 0..state.width {
		let character = match i.cmp(&position) {
			Ordering::Less => state.filled_character,
			Ordering::Greater => state.remaining_character,
			Ordering::Equal => state.curr_character,
		};

		write!(writer, "\x1B[33m{character}\x1B[0m")?;
	}

	write!(writer, "] \x1B[33m{curr_amount} %\x1B[0m")?;

	for tag in &state.tags {
		match tag {
			ProgressTag::Tps => {
				if let Some(rate) = maybe_rate {
					print_tps(writer.as_mut(), rate)?;
				}
			},

			ProgressTag::Dps => {
				if let Some(rate) = maybe_rate {
					print_dps(writer.as_mut(), rate)?;
				}
			},

			ProgressTag::Eta => {
				if let Some(eta) = eta
					&& !eta.is_zero()
				{
					print_eta(writer.as_mut(), eta)?;
				}
			},

			ProgressTag::Time => {
				if !elapsed.is_zero() {
					print_time(writer.as_mut(), elapsed)?;
				}
			},
		}
	}

	write!(writer, "\r")?;
	writer.flush()?;

	Ok(())
}

fn draw_final(state: &LocalProgressState) -> io::Result<()> {
	let curr_amount = state.get_curr_progress_amount() as u8;
	let position = state.get_progress_position(curr_amount);

	let mut writer = state.writer.lock();
	write!(writer, "\x1B[2K[")?;

	for i in 0..state.width {
		let character = match i.cmp(&position) {
			Ordering::Less => state.filled_character,
			Ordering::Greater => state.remaining_character,
			Ordering::Equal => state.curr_character,
		};

		if curr_amount < 100 {
			write!(writer, "\x1B[31m{character}\x1B[0m")?;
		} else {
			write!(writer, "\x1B[32m{character}\x1B[0m")?;
		}
	}

	if curr_amount < 100 {
		write!(writer, "] \x1B[31m{curr_amount} %\x1B[0m")?;
	} else {
		write!(writer, "] \x1B[32m{curr_amount} %\x1B[0m")?;
	}

	if state.tags.contains(&ProgressTag::Time) {
		print_time(writer.as_mut(), state.get_total_duration())?;
	}

	writeln!(writer)?;
	writer.flush()?;

	Ok(())
}

fn print_tps(mut writer: impl Write, rate: u64) -> io::Result<()> {
	write!(writer, " ({} tps)", fmt::number(rate))
}

fn print_dps(mut writer: impl Write, rate: u64) -> io::Result<()> {
	write!(writer, " ({}/s)", fmt::memory(rate, Some(2)))
}

fn print_eta(mut writer: impl Write, eta: Duration) -> io::Result<()> {
	write!(writer, " (eta {})", fmt::timespan(eta.as_millis()))
}

fn print_time(mut writer: impl Write, elapsed: Duration) -> io::Result<()> {
	write!(writer, " (time {})", fmt::timespan(elapsed.as_millis()))
}
