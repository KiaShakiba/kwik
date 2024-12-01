/*
 * Copyright (c) Kia Shakiba
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

pub mod set;

use std::{
	hash::Hash,
	collections::HashMap,
};

use nohash_hasher::{BuildNoHashHasher, IsEnabled};
use linreg::linear_regression;

/// Returns a clone of the minimum value in the supplied splice.
///
/// # Examples
/// ```
/// use kwik::math;
///
/// let value = *math::min::<u64>(&[3, 4, 1, 2]).unwrap();
/// assert_eq!(value, 1);
/// ```
#[inline]
pub fn min<T>(values: &[T]) -> Option<&T>
where
	T: PartialOrd,
{
	if values.is_empty() {
		return None;
	}

	let mut min_value = &values[0];

	for value in values {
		if value < min_value {
			min_value = value;
		}
	}

	Some(min_value)
}

/// Returns a clone of the maximum value in the supplied splice.
///
/// # Examples
/// ```
/// use kwik::math;
///
/// let value = *math::max::<u64>(&[3, 4, 1, 2]).unwrap();
/// assert_eq!(value, 4);
/// ```
#[inline]
pub fn max<T>(values: &[T]) -> Option<&T>
where
	T: PartialOrd,
{
	if values.is_empty() {
		return None;
	}

	let mut max_value = &values[0];

	for value in values {
		if value > max_value {
			max_value = value;
		}
	}

	Some(max_value)
}

/// Returns the zipfian alpha of the provided values.
///
/// # Examples
/// ```
/// use kwik::math;
///
/// let alpha = math::zipf_alpha::<u64>(&[1, 1, 1, 1, 2, 2, 3]).unwrap();
/// assert!(alpha >= 1.0);
/// ```
#[inline]
pub fn zipf_alpha<T>(values: &[T]) -> Option<f64>
where
	T: Copy + Eq + Hash + IsEnabled,
{
	if values.is_empty() {
		return None;
	}

	let mut frequencies = HashMap::<T, u64, BuildNoHashHasher<T>>::with_hasher(
		BuildNoHashHasher::default(),
	);

	for value in values {
		frequencies
			.entry(*value)
			.and_modify(|frequency| *frequency += 1)
			.or_insert(1);
	}

	let mut log_x = Vec::<f64>::new();
	let mut log_y = Vec::<f64>::new();

	for (index, (_, frequency)) in frequencies.iter().enumerate() {
		log_x.push((index as f64 + 1.0).log10());
		log_y.push((*frequency as f64).log10());
	}

	linear_regression::<f64, f64, f64>(&log_x, &log_y)
		.map(|(m, _)| -m)
		.ok()
}
