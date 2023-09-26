/*
 * Copyright (c) Kia Shakiba
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

use std::{
	io::Write,
	convert::TryInto,
	fmt::Debug,
	cmp::Ordering,
};

use crate::{
	fmt,
	math,
	utils::timestamp,
};

const WIDTH: u64 = 70;

const FILLED_CHARACTER: char = '=';
const CURRENT_CHARACTER: char = '>';
const REMAINING_CHARACTER: char = ' ';

const PULSE_INTERVAL: u64 = 1000;

pub struct Progress {
	total: u64,
	current: u64,

	tags: Vec<Tag>,

	rate_count: u64,
	previous_rate: u64,

	initial_time: u64,
	pulse_time: u64,

	amount_timestamps: [u64; 101],
}

#[derive(Clone)]
pub enum Tag {
	Tps,
	Eta,
	Time,
}

impl Progress {
	pub fn new(total: u64, tags: &[Tag]) -> Self {
		let now = timestamp();
		let mut amount_timestamps = [0; 101];

		amount_timestamps[0] = now;

		let progress = Progress {
			total,
			current: 0,

			tags: tags.to_vec(),

			rate_count: 0,
			previous_rate: 0,

			initial_time: now,
			pulse_time: now,

			amount_timestamps,
		};

		progress.draw(0, 0, 0, 0);

		progress
	}

	pub fn complete(&self) -> bool {
		self.current == self.total
	}

	pub fn tick<T>(&mut self, value: T)
	where
		T: TryInto<u64> + Copy,
		<T as TryInto<u64>>::Error: Debug,
	{
		self.set(self.current + value.try_into().unwrap());
	}

	fn set(&mut self, value: u64) {
		if value > self.total {
			panic!("Progress value ({}) larger than total ({}).", value, self.total);
		}

		let previous = self.current;
		self.current = value;

		let amount = (100.0 * self.current as f64 / self.total as f64) as u64;
		let previous_amount = (100.0 * previous as f64 / self.total as f64) as u64;

		let now = timestamp();

		let interval = self.pulse(now);
		let rate = self.get_rate(interval);

		if amount == previous_amount && amount != 100 && interval == 0 {
			return;
		}

		if amount != previous_amount {
			self.amount_timestamps[amount as usize] = now;
		}

		self.draw(
			amount,
			rate,
			self.get_eta(now),
			self.get_time(now)
		);

		if amount == 100 {
			println!();
		}
	}

	fn pulse(&mut self, now: u64) -> u64 {
		let difference = now - self.pulse_time;

		if difference >= PULSE_INTERVAL {
			self.pulse_time = now;
			return difference;
		}

		0
	}

	fn get_rate(&mut self, interval: u64) -> u64 {
		self.rate_count += 1;

		if interval > 0 {
			let rate = self.rate_count as f64 / (interval as f64 / 1000.0);

			self.previous_rate = rate as u64;
			self.rate_count = 0;

			return rate.round() as u64;
		}

		self.previous_rate
	}

	fn get_eta(&self, now: u64) -> u64 {
		let amount = 100.0 * self.current as f64 / self.total as f64;

		if amount as u64 == 100 || now == self.initial_time {
			return 0;
		}

		if amount <= 50.0 {
			if now == self.initial_time {
				return 0;
			}

			let rate = self.current as f64 / (now - self.initial_time) as f64;

			if rate == 0.0 {
				return 0;
			}

			return ((self.total - self.current) as f64 / rate) as u64;
		}

		let x = amount * 2.0 - 100.0;

		let x1 = math::min(&[&x, &98.0]) as usize;
		let x2 = x1 + 1;

		let y1 = self.amount_timestamps[x1];
		let y2 = self.amount_timestamps[x2];

		let m = y2 - y1;
		let b = y1 - m * x1 as u64;

		(now as f64 - (m as f64 * x + b as f64)) as u64
	}

	fn get_time(&self, now: u64) -> u64 {
		now - self.initial_time
	}

	fn draw(&self, amount: u64, rate: u64, eta: u64, time: u64) {
		let position = (WIDTH as f64 * amount as f64 / 100.0) as u64;

		print!("\x1B[2K\r[");

		for i in 0..WIDTH {
			let character = match i.cmp(&position) {
				Ordering::Less => FILLED_CHARACTER,
				Ordering::Greater => REMAINING_CHARACTER,
				Ordering::Equal => CURRENT_CHARACTER,
			};

			if amount < 100 {
				print!("\x1B[33m{}\x1B[0m", character);
			} else {
				print!("\x1B[32m{}\x1B[0m", character);
			}
		}

		if amount < 100 {
			print!("] \x1B[33m{amount} %\x1B[0m");
		} else {
			print!("] \x1B[32m{amount} %\x1B[0m");
		}

		for tag in &self.tags {
			match tag {
				Tag::Tps => {
					if amount < 100 && rate > 0 {
						print!(" ({} tps)", fmt::number(rate));
					}
				},

				Tag::Eta => {
					if amount < 100 && eta > 0 {
						print!(" (eta {})", fmt::timespan(eta));
					}
				},

				Tag::Time => {
					if time > 0 {
						print!(" (time {})", fmt::timespan(time));
					}
				},
			}
		}

		print!("\r");

		std::io::stdout().flush().unwrap();
	}
}
