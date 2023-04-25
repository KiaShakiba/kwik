use std::time::{SystemTime, UNIX_EPOCH};

pub fn timestamp() -> u64 {
	let now = SystemTime::now()
		.duration_since(UNIX_EPOCH)
		.expect("Could not calculate timestamp");

	now.as_secs() * 1000 + now.subsec_nanos() as u64 / 1_000_000
}
