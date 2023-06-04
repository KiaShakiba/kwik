/*
 * Copyright (c) Kia Shakiba
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

/// Returns the minimum value in the supplied splice.
///
/// # Examples
/// ```
/// let value = min::<u64>(&[&3, &4, &1, &2]);
/// assert_eq!(value, 1);
/// ```
pub fn min<T: PartialOrd + Copy>(values: &[&T]) -> T {
	if values.is_empty() {
		panic!("Cannot find min value");
	}

	let mut min_value: T = *values[0];

	for value in values {
		if **value < min_value {
			min_value = **value;
		}
	}

	min_value
}

/// Returns the maximum value in the supplied splice.
///
/// # Examples
/// ```
/// let value = max::<u64>(&[&3, &4, &1, &2]);
/// assert_eq!(value, 4);
/// ```
pub fn max<T: PartialOrd + Copy>(values: &[&T]) -> T {
	if values.is_empty() {
		panic!("Cannot find max value");
	}

	let mut max_value: T = *values[0];

	for value in values {
		if **value > max_value {
			max_value = **value;
		}
	}

	max_value
}
