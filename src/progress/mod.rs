/*
 * Copyright (c) Kia Shakiba
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

mod state;
mod worker;

use std::{
	fmt::Debug,
	sync::{Arc, atomic::Ordering},
};

use num_traits::AsPrimitive;

use crate::progress::{state::ProgressState, worker::ProgressWorker};

/// Displays a progress bar in terminal
pub struct Progress {
	state:  Arc<ProgressState>,
	worker: ProgressWorker,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ProgressTag {
	/// Ticks per second
	Tps,

	/// Data per second
	Dps,

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
	pub fn new(total: impl AsPrimitive<u64>) -> Self {
		let total = total.as_();
		assert_ne!(total, 0, "Total cannot be zero.");

		let state = Arc::new(ProgressState::new(total));
		let worker = ProgressWorker::new(state.clone());

		Progress {
			state,
			worker,
		}
	}

	/// Sets the progress bar's width. The default is 70.
	///
	/// # Panics
	///
	/// Panics if the width is zero.
	#[inline]
	pub fn set_width(&self, width: impl AsPrimitive<u64>) {
		let width = width.as_();

		assert_ne!(width, 0, "Width cannot be zero.");
		self.state.width.store(width, Ordering::Relaxed);
	}

	/// Sets the progress bar's width. The default is 70.
	///
	/// # Panics
	///
	/// Panics if the width is zero.
	#[inline]
	#[must_use]
	pub fn with_width(self, width: impl AsPrimitive<u64>) -> Self {
		self.set_width(width);
		self
	}

	/// Sets the progress bar's filled character. The default is '='.
	#[inline]
	pub fn set_filled_character(&self, filled_character: char) {
		self.state
			.filled_character
			.store(filled_character as u8, Ordering::Relaxed);
	}

	/// Sets the progress bar's filled character. The default is '='.
	#[inline]
	#[must_use]
	pub fn with_filled_character(self, filled_character: char) -> Self {
		self.set_filled_character(filled_character);
		self
	}

	/// Sets the progress bar's current character. The default is '>'.
	#[inline]
	pub fn set_current_character(&self, curr_character: char) {
		self.state
			.curr_character
			.store(curr_character as u8, Ordering::Relaxed);
	}

	/// Sets the progress bar's current character. The default is '>'.
	#[inline]
	#[must_use]
	pub fn with_current_character(self, current_character: char) -> Self {
		self.set_current_character(current_character);
		self
	}

	/// Sets the progress bar's remaining character. The default is ' '.
	#[inline]
	pub fn set_remaining_character(&self, remaining_character: char) {
		self.state
			.remaining_character
			.store(remaining_character as u8, Ordering::Relaxed);
	}

	/// Sets the progress bar's remaining character. The default is ' '.
	#[inline]
	#[must_use]
	pub fn with_remaining_character(self, remaining_character: char) -> Self {
		self.set_remaining_character(remaining_character);
		self
	}

	/// Adds the supplied tag to the enabled tags.
	///
	/// # Examples
	/// ```
	/// use kwik::progress::{Progress, ProgressTag};
	///
	/// let mut progress = Progress::new(100);
	///
	/// progress.set_tag(ProgressTag::Tps);
	/// progress.set_tag(ProgressTag::Dps);
	/// progress.set_tag(ProgressTag::Eta);
	/// progress.set_tag(ProgressTag::Time);
	/// ```
	///
	/// # Panics
	///
	/// Panics if the tag is already enabled.
	#[inline]
	pub fn set_tag(&mut self, tag: ProgressTag) {
		assert!(
			!self.state.tags.read().contains(&tag),
			"Progress tag {tag:?} is already enabled.",
		);

		self.state.tags.write().push(tag);
	}

	/// Adds the supplied tag to the enabled tags.
	///
	/// # Examples
	/// ```
	/// use kwik::progress::{Progress, ProgressTag};
	///
	/// let progress = Progress::new(100)
	///     .with_tag(ProgressTag::Tps)
	///     .with_tag(ProgressTag::Dps)
	///     .with_tag(ProgressTag::Eta)
	///     .with_tag(ProgressTag::Time);
	/// ```
	///
	/// # Panics
	///
	/// Panics if the tag is already enabled.
	#[inline]
	#[must_use]
	pub fn with_tag(mut self, tag: ProgressTag) -> Self {
		self.set_tag(tag);
		self
	}

	/// Checks if the progress is complete.
	#[inline]
	#[must_use]
	pub fn is_complete(&self) -> bool {
		self.state.is_complete()
	}

	/// Ticks the progress bar by the supplied amount.
	///
	/// # Panics
	///
	/// Panics if the tick amount is greater than the total.
	#[inline]
	pub fn tick(&mut self, value: impl AsPrimitive<u64>) {
		self.state
			.curr
			.fetch_add(value.as_(), Ordering::Relaxed);
	}

	/// Stops the progress bar and moves the cursor to a new line.
	///
	/// # Examples
	/// ```
	/// use kwik::progress::{Progress, ProgressTag};
	///
	/// let mut progress = Progress::new(100);
	///
	/// progress.tick(50);
	/// progress.stop(); // the progress bar will stop at 50%
	/// ```
	#[inline]
	pub fn stop(&mut self) {
		self.state.stopped.store(true, Ordering::Relaxed);
	}
}

impl Drop for Progress {
	fn drop(&mut self) {
		if let Some(thread) = self.worker.thread.take() {
			let _ = thread.join();
		}
	}
}
