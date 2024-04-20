/*
 * Copyright (c) Kia Shakiba
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

use std::time::{SystemTime, UNIX_EPOCH};

/// Returns the current system timestamp in milliseconds.
///
/// # Examples
/// ```
/// use kwik::utils;
///
/// assert!(utils::timestamp() > 0);
/// ```
///
/// # Panics
///
/// Panics if the current timestamp could not be calculated.
#[inline]
#[must_use]
pub fn timestamp() -> u64 {
	let now = SystemTime::now()
		.duration_since(UNIX_EPOCH)
		.expect("Could not calculate timestamp");

	now.as_secs() * 1000 + u64::from(now.subsec_nanos()) / 1_000_000
}
