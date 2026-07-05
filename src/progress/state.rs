/*
 * Copyright (c) Kia Shakiba
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

use std::{
	cmp,
	io::{self, Write},
	sync::{
		Arc,
		atomic::{AtomicBool, AtomicU8, AtomicU64, Ordering},
	},
	time::{Duration, Instant},
};

use parking_lot::{Mutex, RwLock};

use crate::{math, progress::ProgressTag};

pub struct ProgressState {
	pub writer: Arc<Mutex<Box<dyn Write + Send>>>,

	pub width: AtomicU64,

	pub filled_character:    AtomicU8,
	pub curr_character:      AtomicU8,
	pub remaining_character: AtomicU8,

	pub total: u64,
	pub curr:  AtomicU64,

	pub stopped: AtomicBool,

	pub tags: RwLock<Vec<ProgressTag>>,
}

pub struct LocalProgressState {
	pub writer: Arc<Mutex<Box<dyn Write + Send>>>,

	pub width: u64,

	pub filled_character:    char,
	pub curr_character:      char,
	pub remaining_character: char,

	pub total: u64,
	pub prev:  u64,
	pub curr:  u64,

	pub stopped: bool,

	pub instants:     [Option<Instant>; 101],
	pub prev_instant: Instant,
	pub curr_instant: Instant,

	pub tags: Box<[ProgressTag]>,
}

impl ProgressState {
	pub fn new(total: u64) -> Self {
		let writer: Box<dyn Write + Send> = Box::new(io::stdout());

		ProgressState {
			writer: Arc::new(Mutex::new(writer)),

			width: AtomicU64::new(70),

			filled_character: AtomicU8::new(b'='),
			curr_character: AtomicU8::new(b'>'),
			remaining_character: AtomicU8::new(b' '),

			total,
			curr: AtomicU64::default(),

			stopped: AtomicBool::default(),

			tags: RwLock::default(),
		}
	}

	#[must_use]
	pub fn is_complete(&self) -> bool {
		self.stopped.load(Ordering::Relaxed) || self.curr.load(Ordering::Relaxed) >= self.total
	}
}

impl LocalProgressState {
	pub fn new(state: &ProgressState) -> Self {
		let now = Instant::now();
		let tags = state.tags.read().iter().copied().collect();

		let mut instants = [None; 101];
		instants[0] = Some(now);

		LocalProgressState {
			width: state.width.load(Ordering::Relaxed),

			filled_character: state.filled_character.load(Ordering::Relaxed) as char,
			curr_character: state.curr_character.load(Ordering::Relaxed) as char,
			remaining_character: state.remaining_character.load(Ordering::Relaxed) as char,

			total: state.total,
			prev: 0,
			curr: state.curr.load(Ordering::Relaxed),

			stopped: state.stopped.load(Ordering::Relaxed),

			instants,
			prev_instant: now,
			curr_instant: now,

			tags,

			writer: state.writer.clone(),
		}
	}

	#[must_use]
	pub fn is_complete(&self) -> bool {
		self.stopped || self.curr >= self.total
	}

	#[must_use]
	pub fn get_curr_progress_amount(&self) -> f64 {
		let curr = cmp::min(self.curr, self.total);
		100.0 * curr as f64 / self.total as f64
	}

	#[must_use]
	pub fn get_prev_progress_amount(&self) -> f64 {
		let prev = cmp::min(self.prev, self.total);
		100.0 * prev as f64 / self.total as f64
	}

	#[must_use]
	pub fn get_tick_duration(&self) -> Duration {
		self.curr_instant
			.duration_since(self.prev_instant)
	}

	#[must_use]
	pub fn get_total_duration(&self) -> Duration {
		self.curr_instant
			.duration_since(self.instants[0].unwrap())
	}

	#[must_use]
	pub fn get_progress_position(&self, amount: u8) -> u64 {
		(self.width as f64 * amount as f64 / 100.0) as u64
	}

	#[must_use]
	pub fn get_rate(&self) -> Option<u64> {
		let tick = self.get_tick_duration();
		if tick.is_zero() {
			return None;
		}

		let sec = tick.as_secs_f64();
		let rate_count = self.curr - self.prev;
		let rate = rate_count as f64 / sec;

		Some(rate.round() as u64)
	}

	#[must_use]
	pub fn get_eta(&self) -> Option<Duration> {
		let curr_amount = self.get_curr_progress_amount();
		let elapsed = self.get_total_duration();

		if curr_amount as u8 == 100 || elapsed.is_zero() {
			return None;
		}

		let x = curr_amount * 2.0 - 100.0;
		let x1 = *math::min(&[x, 98.0]).unwrap() as i64;

		if x1 <= 0 || self.instants[x1 as usize].is_none() {
			let rate = self.curr as f64 / elapsed.as_millis() as f64;

			if rate == 0.0 {
				return None;
			}

			let duration_ms = ((self.total - self.curr) as f64 / rate) as u64;
			let duration = Duration::from_millis(duration_ms);

			return Some(duration);
		}

		let x2 = x1 as usize + 1;

		let y1 = self.instants[x1 as usize].unwrap();
		let y2 = self.instants[x2].unwrap();

		let m = y2 - y1;
		let b = y1 - m * x1 as u32;

		Some(self.curr_instant - (b + Duration::from_millis((m.as_millis() as f64 * x) as u64)))
	}

	pub fn update(&mut self, state: &ProgressState) {
		self.width = state.width.load(Ordering::Relaxed);

		self.filled_character = state.filled_character.load(Ordering::Relaxed) as char;
		self.curr_character = state.curr_character.load(Ordering::Relaxed) as char;
		self.remaining_character = state.remaining_character.load(Ordering::Relaxed) as char;

		self.total = state.total;
		self.prev = self.curr;
		self.curr = state.curr.load(Ordering::Relaxed);

		self.stopped = state.stopped.load(Ordering::Relaxed);

		self.prev_instant = self.curr_instant;
		self.curr_instant = Instant::now();

		let new_tags = state.tags.read();
		if new_tags.iter().ne(&self.tags) {
			self.tags = new_tags.iter().copied().collect();
		}

		let curr_amount = self.get_curr_progress_amount() as u8;
		let prev_amount = self.get_prev_progress_amount() as u8;

		for index in (prev_amount + 1)..=curr_amount {
			self.instants[index as usize] = Some(self.curr_instant);
		}

		if !self.stopped {
			self.stopped = curr_amount == 100;
		}
	}
}
