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
	pub fn push(&mut self, instant: Instant, value: impl AsPrimitive<f64>) {
		self.points.insert(instant, value.as_());
	}

	/// Returns the windowed average at the supplied instant based on the
	/// supplied window duration. The window is centered at the insant. If
	/// points in the dataset are within the window range at the supplied
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
}
