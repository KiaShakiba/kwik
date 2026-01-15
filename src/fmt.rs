/*
 * Copyright (c) Kia Shakiba
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

use num_format::{Locale, ToFormattedString};
use num_traits::AsPrimitive;

pub const MEMORY_UNITS: &[&str] = &[
	"B", "KiB", "MiB", "GiB", "TiB", "PiB", "EiB",
];

/// Formats a number with commas.
///
/// # Examples
/// ```
/// use kwik::fmt;
///
/// assert_eq!(fmt::number(1234567), "1,234,567");
/// ```
#[inline]
#[must_use]
pub fn number(value: impl AsPrimitive<u64>) -> String {
	value.as_().to_formatted_string(&Locale::en)
}

/// Formats a number of bytes with memory units, rounded
/// to the supplied number of decimal places.
///
/// # Examples
/// ```
/// use kwik::fmt;
///
/// assert_eq!(fmt::memory(1234567, Some(2)), "1.18 MiB");
/// ```
#[inline]
#[must_use]
pub fn memory(value: impl AsPrimitive<u64>, precision: Option<usize>) -> String {
	let value = value.as_();
	let mut copy: f64 = value.as_();

	let decimals = precision.unwrap_or(0);
	let mut count: usize = 0;

	while (copy / 1024.0) as u64 > 0 {
		copy /= 1024.0;
		count += 1;
	}

	let unit = MEMORY_UNITS[count];

	format!("{copy:.decimals$} {unit}")
}

/// Formats a timespan in milliseconds to D.hh:mm:ss.ms.
///
/// # Examples
/// ```
/// use kwik::fmt;
///
/// assert_eq!(fmt::timespan(1234567), "20:34.567");
/// ```
#[must_use]
pub fn timespan(value: impl AsPrimitive<u64>) -> String {
	let mut ms: u64 = value.as_();

	let days = ms / 1000 / 60 / 60 / 24;
	ms -= days * 1000 * 60 * 60 * 24;

	let hrs = ms / 1000 / 60 / 60;
	ms -= hrs * 1000 * 60 * 60;

	let mins = ms / 1000 / 60;
	ms -= mins * 1000 * 60;

	let secs = ms / 1000;
	ms -= secs * 1000;

	match (days, hrs, mins, secs, ms) {
		(0, 0, 0, 0, ms) => format!("{ms}"),
		(0, 0, 0, secs, ms) => format!("{secs}.{ms:03}"),
		(0, 0, mins, secs, ms) => format!("{mins}:{secs:02}.{ms:03}"),
		(0, hrs, mins, secs, ms) => format!("{hrs}:{mins:02}:{secs:02}.{ms:03}"),
		(days, hrs, mins, secs, ms) => format!("{days}.{hrs:02}:{mins:02}:{secs:02}.{ms:03}"),
	}
}

#[cfg(test)]
mod tests {
	use crate::fmt;

	#[test]
	fn it_fmts_timespan_ms() {
		assert_eq!(fmt::timespan(0), "0");
		assert_eq!(fmt::timespan(1), "1");
		assert_eq!(fmt::timespan(10), "10");
		assert_eq!(fmt::timespan(100), "100");
	}

	#[test]
	fn it_fmts_timespan_secs() {
		assert_eq!(fmt::timespan(1_000), "1.000");
		assert_eq!(fmt::timespan(1_001), "1.001");
		assert_eq!(fmt::timespan(10_000), "10.000");
		assert_eq!(fmt::timespan(31_234), "31.234");
		assert_eq!(fmt::timespan(59_999), "59.999");
	}

	#[test]
	fn it_fmts_timespan_mins() {
		assert_eq!(fmt::timespan(60_000), "1:00.000");
		assert_eq!(fmt::timespan(60_001), "1:00.001");
		assert_eq!(fmt::timespan(600_000), "10:00.000");
		assert_eq!(fmt::timespan(1_234_567), "20:34.567");
		assert_eq!(fmt::timespan(3_599_999), "59:59.999");
	}

	#[test]
	fn it_fmts_timespan_hrs() {
		assert_eq!(fmt::timespan(3_600_000), "1:00:00.000");
		assert_eq!(fmt::timespan(3_600_001), "1:00:00.001");
		assert_eq!(fmt::timespan(36_000_000), "10:00:00.000");
		assert_eq!(fmt::timespan(12_345_678), "3:25:45.678");
		assert_eq!(fmt::timespan(86_399_999), "23:59:59.999");
	}

	#[test]
	fn it_fmts_timespan_days() {
		assert_eq!(fmt::timespan(86_400_000), "1.00:00:00.000");
		assert_eq!(fmt::timespan(86_400_001), "1.00:00:00.001");
		assert_eq!(fmt::timespan(864_000_000), "10.00:00:00.000");
		assert_eq!(fmt::timespan(123_456_789), "1.10:17:36.789");
		assert_eq!(fmt::timespan(950_399_999), "10.23:59:59.999");
	}
}
