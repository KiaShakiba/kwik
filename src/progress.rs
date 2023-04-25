use std::convert::TryInto;
use indicatif::{ProgressBar, ProgressStyle, ProgressDrawTarget};

pub struct Progress {
	bar: ProgressBar,
}

impl Progress {
	pub fn new(max: u64) -> Self {
		let bar = ProgressBar::new(max);

		bar.set_style(ProgressStyle::with_template(
			"{wide_bar} {percent} % (tps {per_sec}) (eta {eta}) (time {elapsed})"
		).unwrap());

		bar.set_draw_target(ProgressDrawTarget::stdout());

		Progress {
			bar
		}
	}

	pub fn tick<T>(&self, value: T)
	where
		T: TryInto<u64>,
		<T as TryInto<u64>>::Error: std::fmt::Debug,
	{
		self.bar.inc(value.try_into().unwrap());
	}
}
