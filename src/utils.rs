/*
 * Copyright (c) Kia Shakiba
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

use std::time::{SystemTime, UNIX_EPOCH};

/// Returns the current system timestamp in milliseconds.
pub fn timestamp() -> u64 {
	let now = SystemTime::now()
		.duration_since(UNIX_EPOCH)
		.expect("Could not calculate timestamp");

	now.as_secs() * 1000 + now.subsec_nanos() as u64 / 1_000_000
}
