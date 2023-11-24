/*
 * Copyright (c) Kia Shakiba
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

/// Returns a clone of the minimum value in the supplied splice.
///
/// # Panics
/// Panics if the supplied slice is empty.
///
/// # Examples
/// ```
/// use kwik::math;
///
/// let value = math::min::<u64>(&[3, 4, 1, 2]);
/// assert_eq!(value, 1);
/// ```
pub fn min<T: Clone + PartialOrd>(values: &[T]) -> T {
	if values.is_empty() {
		panic!("Cannot find min value.");
	}

	let mut min_value = &values[0];

	for value in values {
		if value < min_value {
			min_value = value;
		}
	}

	min_value.clone()
}

/// Returns a clone of the maximum value in the supplied splice.
///
/// # Panics
/// Panics if the supplied slice is empty.
///
/// # Examples
/// ```
/// use kwik::math;
///
/// let value = math::max::<u64>(&[3, 4, 1, 2]);
/// assert_eq!(value, 4);
/// ```
pub fn max<T: Clone + PartialOrd>(values: &[T]) -> T {
	if values.is_empty() {
		panic!("Cannot find max value.");
	}

	let mut max_value = &values[0];

	for value in values {
		if value > max_value {
			max_value = value;
		}
	}

	max_value.clone()
}
