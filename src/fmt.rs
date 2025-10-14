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
pub fn memory(
	value: impl AsPrimitive<u64>,
	precision: Option<usize>,
) -> String {
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
	let mut milliseconds: u64 = value.as_();

	let days = milliseconds / 1000 / 60 / 60 / 24;
	milliseconds -= days * 1000 * 60 * 60 * 24;

	let hours = milliseconds / 1000 / 60 / 60;
	milliseconds -= hours * 1000 * 60 * 60;

	let minutes = milliseconds / 1000 / 60;
	milliseconds -= minutes * 1000 * 60;

	let seconds = milliseconds / 1000;
	milliseconds -= seconds * 1000;

	let mut formatted = String::new();
	let mut started = false;

	if days > 0 {
		formatted.push_str(&format!("{days}."));
		started = true;
	}

	if started || hours > 0 {
		let padding = if started { 2 } else { 0 };
		formatted.push_str(&format!("{hours:0padding$}:"));
		started = true;
	}

	if started || minutes > 0 {
		let padding = if started { 2 } else { 0 };
		formatted.push_str(&format!("{minutes:0padding$}:"));
		started = true;
	}

	if started || seconds > 0 {
		let padding = if started { 2 } else { 0 };
		formatted.push_str(&format!("{seconds:0padding$}."));
		started = true;
	}

	let padding = if started { 3 } else { 0 };
	formatted.push_str(&format!("{milliseconds:0padding$}"));

	formatted
}
