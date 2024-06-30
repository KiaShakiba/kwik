/*
 * Copyright (c) Kia Shakiba
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

use std::{
	io::{self, Write, StdoutLock},
	fmt::Debug,
	cmp::Ordering,
	time::{Instant, Duration},
};

use crate::{fmt, math};

const DEFAULT_WIDTH: u64 = 70;

const DEFAULT_FILLED_CHARACTER: char = '=';
const DEFAULT_CURRENT_CHARACTER: char = '>';
const DEFAULT_REMAINING_CHARACTER: char = ' ';

const PULSE_INTERVAL: Duration = Duration::from_secs(1);

/// Displays a progress bar in terminal
pub struct Progress {
	width: u64,

	filled_character: char,
	current_character: char,
	remaining_character: char,

	total: u64,
	current: u64,

	stopped: bool,

	tags: Vec<Tag>,

	rate_count: u64,
	previous_rate: u64,

	instants: [Option<Instant>; 101],
	pulse_instant: Instant,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Tag {
	/// Ticks per second
	Tps,

	/// Estimated remaining time
	Eta,

	/// Elapsed time
	Time,
}

impl Progress {
	/// Initializes and prints a new progress bar
	///
	/// # Examples
	/// ```
	/// use kwik::progress::Progress;
	///
	/// let progress = Progress::new(100);
	/// ```
	///
	/// # Panics
	///
	/// Panics if the total is zero.
	#[must_use]
	pub fn new(total: u64) -> Self {
		assert_ne!(total, 0, "Total cannot be zero.");

		let now = Instant::now();
		let mut instants = [None; 101];

		instants[0] = Some(now);

		let progress = Progress {
			width: DEFAULT_WIDTH,

			filled_character: DEFAULT_FILLED_CHARACTER,
			current_character: DEFAULT_CURRENT_CHARACTER,
			remaining_character: DEFAULT_REMAINING_CHARACTER,

			total,
			current: 0,

			stopped: false,

			tags: Vec::new(),

			rate_count: 0,
			previous_rate: 0,

			instants,
			pulse_instant: now,
		};

		progress.draw(0, 0, None, Duration::ZERO);

		progress
	}

	/// Sets the progress bar's width. The default is 70.
	///
	/// # Panics
	///
	/// Panics if the width is zero.
	#[inline]
	pub fn set_width(&mut self, width: u64) {
		assert_ne!(width, 0, "Width cannot be zero.");
		self.width = width;
	}

	/// Sets the progress bar's width. The default is 70.
	///
	/// # Panics
	///
	/// Panics if the width is zero.
	#[inline]
	#[must_use]
	pub fn with_width(mut self, width: u64) -> Self {
		self.set_width(width);
		self
	}

	/// Sets the progress bar's filled character. The default is '='.
	#[inline]
	pub fn set_filled_character(&mut self, filled_character: char) {
		self.filled_character = filled_character;
	}

	/// Sets the progress bar's filled character. The default is '='.
	#[inline]
	#[must_use]
	pub fn with_filled_character(mut self, filled_character: char) -> Self {
		self.set_filled_character(filled_character);
		self
	}

	/// Sets the progress bar's current character. The default is '>'.
	#[inline]
	pub fn set_current_character(&mut self, current_character: char) {
		self.current_character = current_character;
	}

	/// Sets the progress bar's current character. The default is '>'.
	#[inline]
	#[must_use]
	pub fn with_current_character(mut self, current_character: char) -> Self {
		self.set_current_character(current_character);
		self
	}

	/// Sets the progress bar's remaining character. The default is ' '.
	#[inline]
	pub fn set_remaining_character(&mut self, remaining_character: char) {
		self.remaining_character = remaining_character;
	}

	/// Sets the progress bar's remaining character. The default is ' '.
	#[inline]
	#[must_use]
	pub fn with_remaining_character(mut self, remaining_character: char) -> Self {
		self.set_remaining_character(remaining_character);
		self
	}

	/// Adds the supplied tag to the enabled tags.
	///
	/// # Examples
	/// ```
	/// use kwik::progress::{Progress, Tag};
	///
	/// let mut progress = Progress::new(100);
	///
	/// progress.set_tag(Tag::Tps);
	/// progress.set_tag(Tag::Eta);
	/// progress.set_tag(Tag::Time);
	/// ```
	///
	/// # Panics
	///
	/// Panics if the tag is already enabled.
	#[inline]
	pub fn set_tag(&mut self, tag: Tag) {
		assert!(
			!self.tags.contains(&tag),
			"Progress tag {tag:?} is already enabled.",
		);

		self.tags.push(tag);
	}

	/// Adds the supplied tag to the enabled tags.
	///
	/// # Examples
	/// ```
	/// use kwik::progress::{Progress, Tag};
	///
	/// let progress = Progress::new(100)
	///     .with_tag(Tag::Tps)
	///     .with_tag(Tag::Eta)
	///     .with_tag(Tag::Time);
	/// ```
	///
	/// # Panics
	///
	/// Panics if the tag is already enabled.
	#[inline]
	#[must_use]
	pub fn with_tag(mut self, tag: Tag) -> Self {
		self.set_tag(tag);
		self
	}

	/// Checks if the progress is complete.
	#[inline]
	#[must_use]
	pub fn is_complete(&self) -> bool {
		self.current == self.total
	}

	/// Ticks the progress bar by the supplied amount.
	///
	/// # Panics
	///
	/// Panics if the tick amount is greater than the total.
	#[inline]
	pub fn tick<T>(&mut self, value: T)
	where
		T: TryInto<u64> + Copy,
		<T as TryInto<u64>>::Error: Debug,
	{
		self.set(self.current + value.try_into().unwrap());
	}

	fn set(&mut self, value: u64) {
		assert!(!self.stopped, "Progress bar has been stopped.");

		assert!(
			value <= self.total,
			"Progress value ({value}) larger than total ({}).",
			self.total,
		);

		let previous = self.current;
		self.current = value;

		let amount = self.get_progress_amount(self.current) as u8;
		let previous_amount = self.get_progress_amount(previous) as u8;

		let now = Instant::now();

		let pulse_duration = self.pulse(&now);
		let rate = self.get_rate(pulse_duration);

		if amount == previous_amount && amount != 100 && pulse_duration.is_none() {
			return;
		}

		if amount != previous_amount {
			self.instants[amount as usize] = Some(now);
		}

		self.draw(
			amount,
			rate,
			self.get_eta(&now),
			now - self.instants[0].unwrap(),
		);

		if amount == 100 {
			self.stopped = true;
			println!();
		}
	}

	/// Stops the progress bar and moves the cursor to a new line.
	///
	/// # Examples
	/// ```
	/// use kwik::progress::{Progress, Tag};
	///
	/// let mut progress = Progress::new(100);
	///
	/// progress.tick(50);
	/// progress.stop(); // the progress bar will stop at 50%
	/// ```
	///
	#[inline]
	pub fn stop(&mut self) {
		if self.stopped {
			return;
		}

		self.stopped = true;

		let now = Instant::now();
		let amount = self.get_progress_amount(self.current) as u8;

		self.draw_final(amount, now - self.instants[0].unwrap());
	}

	#[inline]
	#[must_use]
	fn pulse(&mut self, now: &Instant) -> Option<Duration> {
		let duration = now.duration_since(self.pulse_instant);

		if duration >= PULSE_INTERVAL {
			self.pulse_instant = *now;
			return Some(duration);
		}

		None
	}

	#[must_use]
	fn get_progress_amount(&self, current: u64) -> f64 {
		100.0 * current as f64 / self.total as f64
	}

	#[must_use]
	fn get_progress_position(&self, amount: u8) -> u64 {
		(self.width as f64 * amount as f64 / 100.0) as u64
	}

	#[must_use]
	fn get_rate(&mut self, pulse_duration: Option<Duration>) -> u64 {
		self.rate_count += 1;

		if let Some(pulse_duration) = pulse_duration {
			let ms = pulse_duration.as_millis() as f64;
			let rate = self.rate_count as f64 / (ms / 1000.0);

			self.previous_rate = rate as u64;
			self.rate_count = 0;

			return rate.round() as u64;
		}

		self.previous_rate
	}

	#[must_use]
	fn get_eta(&self, now: &Instant) -> Option<Duration> {
		let amount = self.get_progress_amount(self.current);
		let elapsed = now.duration_since(self.instants[0].unwrap());

		if amount as u8 == 100 || elapsed.is_zero() {
			return None;
		}

		let x = amount * 2.0 - 100.0;
		let x1 = math::min(&[x, 98.0]) as i64;
		let x2 = x1 as usize + 1;

		if x1 <= 0 || self.instants[x1 as usize].is_none() {
			let rate = self.current as f64 / elapsed.as_millis() as f64;

			if rate == 0.0 {
				return None;
			}

			let duration_ms = ((self.total - self.current) as f64 / rate) as u64;
			let duration = Duration::from_millis(duration_ms);

			return Some(duration);
		}

		let y1 = self.instants[x1 as usize].unwrap();
		let y2 = self.instants[x2].unwrap();

		let m = y2 - y1;
		let b = y1 - m * x1 as u32;

		Some(*now - (b + Duration::from_millis((m.as_millis() as f64 * x) as u64)))
	}

	fn draw(
		&self,
		amount: u8,
		rate: u64,
		eta: Option<Duration>,
		elapsed: Duration,
	) {
		if amount == 100 {
			return self.draw_final(amount, elapsed);
		}

		let mut lock = io::stdout().lock();
		let position = self.get_progress_position(amount);

		write!(lock, "\x1B[2K\r[").unwrap();

		for i in 0..self.width {
			let character = match i.cmp(&position) {
				Ordering::Less => self.filled_character,
				Ordering::Greater => self.remaining_character,
				Ordering::Equal => self.current_character,
			};

			write!(lock, "\x1B[33m{character}\x1B[0m").unwrap();
		}

		write!(lock, "] \x1B[33m{amount} %\x1B[0m").unwrap();

		for tag in &self.tags {
			match tag {
				Tag::Tps => if rate > 0 {
					print_rate(&mut lock, rate);
				},

				Tag::Eta => if eta.is_some_and(|eta| !eta.is_zero()) {
					print_eta(&mut lock, eta.unwrap());
				},

				Tag::Time => if !elapsed.is_zero() {
					print_time(&mut lock, elapsed);
				},
			}
		}

		write!(lock, "\r").unwrap();
		lock.flush().unwrap();
	}

	fn draw_final(&self, amount: u8, elapsed: Duration) {
		let mut lock = io::stdout().lock();
		let position = self.get_progress_position(amount);

		write!(lock, "\x1B[2K[").unwrap();

		for i in 0..self.width {
			let character = match i.cmp(&position) {
				Ordering::Less => self.filled_character,
				Ordering::Greater => self.remaining_character,
				Ordering::Equal => self.current_character,
			};

			if amount < 100 {
				write!(lock, "\x1B[31m{character}\x1B[0m").unwrap();
			} else {
				write!(lock, "\x1B[32m{character}\x1B[0m").unwrap();
			}
		}

		if amount < 100 {
			write!(lock, "] \x1B[31m{amount} %\x1B[0m").unwrap();
		} else {
			write!(lock, "] \x1B[32m{amount} %\x1B[0m").unwrap();
		}

		if self.tags.contains(&Tag::Time) {
			print_time(&mut lock, elapsed);
		}

		writeln!(lock).unwrap();
		lock.flush().unwrap();
	}
}

fn print_rate(lock: &mut StdoutLock, rate: u64) {
	write!(
		lock,
		" ({} tps)",
		fmt::number(rate),
	).unwrap();
}

fn print_eta(lock: &mut StdoutLock, eta: Duration) {
	write!(
		lock,
		" (eta {})",
		fmt::timespan(eta.as_millis()),
	).unwrap();
}

fn print_time(lock: &mut StdoutLock, elapsed: Duration) {
	write!(
		lock,
		" (time {})",
		fmt::timespan(elapsed.as_millis()),
	).unwrap();
}
