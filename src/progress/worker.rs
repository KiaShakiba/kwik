/*
 * Copyright (c) Kia Shakiba
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

use std::{
	cmp::Ordering,
	io::{self, StdoutLock, Write},
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

const DRAW_DELAY: Duration = Duration::from_millis(150);

pub struct ProgressWorker {
	pub thread: Option<thread::JoinHandle<()>>,
}

impl ProgressWorker {
	pub fn new(state: Arc<ProgressState>) -> Self {
		let mut local_state = LocalProgressState::new(&state);

		let thread = Some(thread::spawn(move || {
			loop {
				if local_state.is_complete() {
					draw_final(&local_state);
					break;
				} else {
					draw(&local_state);
				}

				thread::sleep(DRAW_DELAY);
				local_state.update(&state);
			}
		}));

		ProgressWorker {
			thread,
		}
	}
}

fn draw(state: &LocalProgressState) {
	let curr_amount = state.get_curr_progress_amount() as u8;
	let position = state.get_progress_position(curr_amount);
	let maybe_rate = state.get_rate();
	let eta = state.get_eta();
	let elapsed = state.get_total_duration();

	let mut lock = io::stdout().lock();
	write!(lock, "\x1B[2K\r[").unwrap();

	for i in 0..state.width {
		let character = match i.cmp(&position) {
			Ordering::Less => state.filled_character,
			Ordering::Greater => state.remaining_character,
			Ordering::Equal => state.curr_character,
		};

		write!(lock, "\x1B[33m{character}\x1B[0m").unwrap();
	}

	write!(lock, "] \x1B[33m{curr_amount} %\x1B[0m").unwrap();

	for tag in &state.tags {
		match tag {
			ProgressTag::Tps => {
				if let Some(rate) = maybe_rate {
					print_tps(&mut lock, rate);
				}
			},

			ProgressTag::Dps => {
				if let Some(rate) = maybe_rate {
					print_dps(&mut lock, rate);
				}
			},

			ProgressTag::Eta => {
				if eta.is_some_and(|eta| !eta.is_zero()) {
					print_eta(&mut lock, eta.unwrap());
				}
			},

			ProgressTag::Time => {
				if !elapsed.is_zero() {
					print_time(&mut lock, elapsed);
				}
			},
		}
	}

	write!(lock, "\r").unwrap();
	lock.flush().unwrap();
}

fn draw_final(state: &LocalProgressState) {
	let curr_amount = state.get_curr_progress_amount() as u8;
	let position = state.get_progress_position(curr_amount);

	let mut lock = io::stdout().lock();
	write!(lock, "\x1B[2K[").unwrap();

	for i in 0..state.width {
		let character = match i.cmp(&position) {
			Ordering::Less => state.filled_character,
			Ordering::Greater => state.remaining_character,
			Ordering::Equal => state.curr_character,
		};

		if curr_amount < 100 {
			write!(lock, "\x1B[31m{character}\x1B[0m").unwrap();
		} else {
			write!(lock, "\x1B[32m{character}\x1B[0m").unwrap();
		}
	}

	if curr_amount < 100 {
		write!(lock, "] \x1B[31m{curr_amount} %\x1B[0m").unwrap();
	} else {
		write!(lock, "] \x1B[32m{curr_amount} %\x1B[0m").unwrap();
	}

	if state.tags.contains(&ProgressTag::Time) {
		print_time(&mut lock, state.get_total_duration());
	}

	writeln!(lock).unwrap();
	lock.flush().unwrap();
}

fn print_tps(lock: &mut StdoutLock, rate: u64) {
	write!(lock, " ({} tps)", fmt::number(rate)).unwrap();
}

fn print_dps(lock: &mut StdoutLock, rate: u64) {
	write!(lock, " ({}/s)", fmt::memory(rate, Some(2))).unwrap();
}

fn print_eta(lock: &mut StdoutLock, eta: Duration) {
	write!(lock, " (eta {})", fmt::timespan(eta.as_millis())).unwrap();
}

fn print_time(lock: &mut StdoutLock, elapsed: Duration) {
	write!(lock, " (time {})", fmt::timespan(elapsed.as_millis())).unwrap();
}
