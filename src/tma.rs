/*
 * Copyright (c) Kia Shakiba
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

use std::{
	time::{Instant, Duration},
	collections::BTreeMap,
	ops::Bound,
};

use num_traits::AsPrimitive;

// A time-based centered moving average.
#[derive(Default)]
pub struct TimeMovingAverage {
	points: BTreeMap<Instant, f64>,
}

pub struct WindowIter<'a> {
	tma: &'a TimeMovingAverage,

	window: Duration,
	current: Option<Instant>,
}

pub struct IntoWindowIter {
	tma: TimeMovingAverage,

	window: Duration,
	current: Option<Instant>,
}

impl TimeMovingAverage {
	/// Returns true if there are no points in the dataset.
	///
	/// # Examples
	/// ```
	/// use kwik::tma::TimeMovingAverage;
	///
	/// let tma = TimeMovingAverage::default();
	/// assert!(tma.is_empty());
	/// ```
	#[inline]
	pub fn is_empty(&self) -> bool {
		self.points.is_empty()
	}

	/// Returns the number of points in the dataset.
	///
	/// # Examples
	/// ```
	/// use kwik::tma::TimeMovingAverage;
	///
	/// let tma = TimeMovingAverage::default();
	/// assert_eq!(tma.len(), 0);
	/// ```
	#[inline]
	pub fn len(&self) -> usize {
		self.points.len()
	}

	/// Adds a point to the dataset.
	///
	/// # Examples
	/// ```
	/// use std::time::Instant;
	/// use kwik::tma::TimeMovingAverage;
	///
	/// let mut tma = TimeMovingAverage::default();
	///
	/// tma.push(Instant::now(), 1.0);
	///
	/// assert!(!tma.is_empty());
	/// assert_eq!(tma.len(), 1);
	/// ```
	#[inline]
	pub fn push(&mut self, instant: Instant, value: impl AsPrimitive<f64>) {
		self.points.insert(instant, value.as_());
	}

	/// Returns the windowed average at the supplied instant based on the
	/// supplied window duration. The window is centered at the insant. If
	/// no points in the dataset are within the window range at the supplied
	/// instant, `None` is returned.
	///
	/// # Examples
	/// ```
	/// use std::time::{Instant, Duration};
	/// use kwik::tma::TimeMovingAverage;
	///
	/// let mut tma = TimeMovingAverage::default();
	///
	/// let now = Instant::now();
	/// let later = now + Duration::from_secs(5);
	///
	/// tma.push(now, 1.0);
	///
	/// let valid_average = tma.get_windowed_average(now, Duration::from_secs(1));
	/// let invalid_average = tma.get_windowed_average(later, Duration::from_secs(1));
	///
	/// assert_eq!(valid_average, Some(1.0));
	/// assert_eq!(invalid_average, None);
	/// ```
	#[inline]
	pub fn get_windowed_average(
		&self,
		instant: Instant,
		window: Duration,
	) -> Option<f64> {
		let shift = window / 2;

		let start = Bound::Included(instant - shift);
		let end = Bound::Included(instant + shift);

		let mut total: f64 = 0.0;
		let mut count: usize = 0;

		for (_, value) in self.points.range((start, end)) {
			total += *value;
			count += 1;
		}

		match count {
			0 => None,
			count => Some(total / count as f64),
		}
	}

	/// Returns an iterator over a windowed average of the points. The iterator
	/// yields averages centered within the windows with half-window overlaps.
	///
	/// # Examples
	/// ```
	/// use std::time::{Instant, Duration};
	/// use kwik::tma::TimeMovingAverage;
	///
	/// let mut tma = TimeMovingAverage::default();
	/// let window = Duration::from_secs(1);
	///
	/// tma.push(Instant::now(), 1.0);
	/// tma.push(Instant::now() + Duration::from_secs(1), 2.0);
	/// tma.push(Instant::now() + Duration::from_secs(2), 3.0);
	///
	/// for (instant, value) in tma.window_iter(window) {
	///     // do something with the instant and value
	/// }
	/// ```
	#[inline]
	pub fn window_iter(&self, window: Duration) -> WindowIter {
		let current = self.points
			.first_key_value()
			.map(|(instant, _)| *instant);

		WindowIter {
			tma: self,

			window,
			current,
		}
	}

	/// Returns an iterator over a windowed average of the points, consuming
	/// the `TimeMovingAverage`. The iterator yields averages centered
	/// within the windows with half-window overlaps.
	///
	/// # Examples
	/// ```
	/// use std::time::{Instant, Duration};
	/// use kwik::tma::TimeMovingAverage;
	///
	/// let mut tma = TimeMovingAverage::default();
	/// let window = Duration::from_secs(1);
	///
	/// tma.push(Instant::now(), 1.0);
	/// tma.push(Instant::now() + Duration::from_secs(1), 2.0);
	/// tma.push(Instant::now() + Duration::from_secs(2), 3.0);
	///
	/// for (instant, value) in tma.into_window_iter(window) {
	///     // do something with the instant and value
	/// }
	/// ```
	#[inline]
	pub fn into_window_iter(self, window: Duration) -> IntoWindowIter {
		let current = self.points
			.first_key_value()
			.map(|(instant, _)| *instant);

		IntoWindowIter {
			tma: self,

			window,
			current,
		}
	}
}

impl<'a> Iterator for WindowIter<'a> {
	type Item = (Instant, f64);

	fn next(&mut self) -> Option<Self::Item> {
		let instant = self.current?;
		let value = self.tma.get_windowed_average(instant, self.window)?;

		self.current = Some(instant + self.window / 2);

		Some((instant, value))
	}
}

impl Iterator for IntoWindowIter {
	type Item = (Instant, f64);

	fn next(&mut self) -> Option<Self::Item> {
		let instant = self.current?;
		let value = self.tma.get_windowed_average(instant, self.window)?;

		self.current = Some(instant + self.window / 2);

		Some((instant, value))
	}
}

#[cfg(test)]
mod tests {
	use std::time::{Instant, Duration};
	use crate::tma::TimeMovingAverage;

	#[test]
	fn it_yields_correct_averages() {
		let mut tma = TimeMovingAverage::default();

		let times = &[0, 1, 2, 3, 4, 5];
		let values = &[1.0, 1.5, 2.0, 3.0, 5.0, 5.5];

		let start = Instant::now();

		for (time, value) in times.iter().zip(values.iter()) {
			tma.push(start + Duration::from_secs(*time), *value);
		}

		let window = Duration::from_secs(2);

		let expected_values = &[
			(1.0 + 1.5) / 2.0,
			(1.0 + 1.5 + 2.0) / 3.0,
			(1.5 + 2.0 + 3.0) / 3.0,
			(2.0 + 3.0 + 5.0) / 3.0,
			(3.0 + 5.0 + 5.5) / 3.0,
			(5.0 + 5.5) / 2.0,
			5.5,
		];

		let mut iter_count = 0;

		for (_, value) in tma.window_iter(window) {
			assert_eq!(value, expected_values[iter_count]);
			iter_count += 1;
		}

		assert_eq!(iter_count, expected_values.len());

		let mut into_iter_count = 0;

		for (_, value) in tma.into_window_iter(window) {
			assert_eq!(value, expected_values[into_iter_count]);
			into_iter_count += 1;
		}

		assert_eq!(into_iter_count, expected_values.len());
	}
}
