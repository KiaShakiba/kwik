/*
 * Copyright (c) Kia Shakiba
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

use std::{
	hash::Hash,
	collections::HashMap,
};

use nohash_hasher::{BuildNoHashHasher, IsEnabled};
use linreg::linear_regression;

/// Calculates streaming Zipf distribution statistics.
pub struct Zipf<T> {
	frequencies: HashMap<T, u64, BuildNoHashHasher<T>>,
}

impl<T> Zipf<T> {
	/// Inserts a value into the distribution.
	///
	/// # Examples
	/// ```
	/// use kwik::math::zipf::Zipf;
	///
	/// let mut zipf = Zipf::<u64>::default();
	///
	/// zipf.insert(1);
	/// zipf.insert(2);
	/// zipf.insert(3);
	/// ```
	pub fn insert(&mut self, value: T)
	where
		T: Copy + Eq + Hash + IsEnabled,
	{
		self.frequencies
			.entry(value)
			.and_modify(|frequency| *frequency += 1)
			.or_insert(1);
	}

	/// Calculates the Zipf alpha parameter of the distribution.
	///
	/// # Examples
	/// ```
	/// use kwik::math::zipf::Zipf;
	///
	/// let mut zipf = Zipf::<u64>::default();
	///
	/// zipf.insert(1);
	/// zipf.insert(1);
	/// zipf.insert(1);
	/// zipf.insert(1);
	/// zipf.insert(2);
	/// zipf.insert(2);
	/// zipf.insert(3);
	///
	/// let alpha = zipf.alpha().unwrap();
	/// assert!(alpha >= 1.0);
	/// ```
	pub fn alpha(&self) -> Option<f64> {
		if self.frequencies.is_empty() {
			return None;
		}

		let mut log_x = Vec::<f64>::new();
		let mut log_y = Vec::<f64>::new();

		for (index, (_, frequency)) in self.frequencies.iter().enumerate() {
			log_x.push((index as f64 + 1.0).log10());
			log_y.push((*frequency as f64).log10());
		}

		linear_regression::<f64, f64, f64>(&log_x, &log_y)
			.map(|(m, _)| -m)
			.ok()
	}
}

impl<T> Default for Zipf<T> {
	fn default() -> Self {
		let hasher = BuildNoHashHasher::default();

		Zipf {
			frequencies: HashMap::with_hasher(hasher),
		}
	}
}
