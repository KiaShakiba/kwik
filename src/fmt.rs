use num_format::{Locale, ToFormattedString};

pub const MEMORY_UNITS: &'static [&'static str] = &["B", "KB", "MB", "GB", "TB", "PB", "EB"];

pub fn number(value: &u64) -> String {
	value.to_formatted_string(&Locale::en)
}

pub fn memory(value: &u64, precision: Option<usize>) -> String {
	let mut copy = *value as f64;

	let decimals = match precision {
		Some(precision) => precision,
		None => 0,
	};

	let mut count: usize = 0;

	while (copy / 1024.0) as u64 > 0 {
		copy /= 1024.0;
		count += 1;
	}

	let unit = MEMORY_UNITS[count];

	format!("{:.1$} {unit}", copy, decimals)
}
