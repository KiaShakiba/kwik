/*
 * Copyright (c) Kia Shakiba
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

use num_format::{Locale, ToFormattedString};

pub const MEMORY_UNITS: &[&str] = &["B", "KiB", "MiB", "GiB", "TiB", "PiB", "EiB"];

pub fn number(value: &u64) -> String {
	value.to_formatted_string(&Locale::en)
}

pub fn memory(value: &u64, precision: Option<usize>) -> String {
	let mut copy = *value as f64;
	let decimals = precision.unwrap_or(0);
	let mut count: usize = 0;

	while (copy / 1024.0) as u64 > 0 {
		copy /= 1024.0;
		count += 1;
	}

	let unit = MEMORY_UNITS[count];

	format!("{:.1$} {unit}", copy, decimals)
}

pub fn timespan(value: &u64) -> String {
	let mut milliseconds: u64 = *value;

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
		formatted.push_str(&format!("{:0width$}:", hours, width = padding));
		started = true;
	}

	if started || minutes > 0 {
		let padding = if started { 2 } else { 0 };
		formatted.push_str(&format!("{:0width$}:", minutes, width = padding));
		started = true;
	}

	if started || seconds > 0 {
		let padding = if started { 2 } else { 0 };
		formatted.push_str(&format!("{:0width$}.", seconds, width = padding));
		started = true;
	}

	let padding = if started { 3 } else { 0 };
	formatted.push_str(&format!("{:0width$}", milliseconds, width = padding));

	formatted
}
