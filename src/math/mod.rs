/*
 * Copyright (c) Kia Shakiba
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

pub mod set;
pub mod stats;

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
