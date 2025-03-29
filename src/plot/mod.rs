/*
 * Copyright (c) Kia Shakiba
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

pub mod figure;
pub mod line_plot;
pub mod box_plot;
pub mod scatter_plot;
pub mod bar_plot;

use std::fmt::Display;
use num_traits::AsPrimitive;
use gnuplot::{Axes2D, AutoOption, DashType};
use crate::fmt::MEMORY_UNITS;

const COLORS: &[&str] = &[
	"#c4342b",
	"#0071ad",
	"#71ad00",
	"#554ec9",
	"#f7790d",
	"#e0ca3c",
	"#47a8bd",
];

const DASH_TYPES: &[DashType] = &[
	DashType::Solid,
	DashType::Dash,
	DashType::DotDash,
	DashType::DotDotDash,
	DashType::Dot,
];

const TIME_UNITS: &[&str] = &["ms", "s", "mins", "hrs", "days"];

/// Implementing this trait allows the struct to be added to a
/// plot figure.
pub trait Plot {
	/// Returns `true` if the plot is empty (i.e., has no data).
	fn is_empty(&self) -> bool;

	/// Sets the plot's font type.
	fn set_font_type(&mut self, font_type: &str);

	/// Sets the plot's font type.
	fn with_font_type(self, font_type: &str) -> Self;

	/// Sets the plot's font size.
	fn set_font_size(&mut self, font_size: impl AsPrimitive<f64>);

	/// Sets the plot's font size.
	fn with_font_size(self, font_size: impl AsPrimitive<f64>) -> Self;

	/// Sets the plot's title.
	fn set_title<T>(&mut self, title: T)
	where
		T: Display,
	;

	/// Sets the plot's title.
	fn with_title<T>(self, title: T) -> Self
	where
		T: Display,
	;

	/// Sets the plot's x-axis label.
	fn set_x_label<T>(&mut self, label: T)
	where
		T: Display,
	;

	/// Sets the plot's x-axis label.
	fn with_x_label<T>(self, label: T) -> Self
	where
		T: Display,
	;

	/// Sets the plot's y-axis label.
	fn set_y_label<T>(&mut self, label: T)
	where
		T: Display,
	;

	/// Sets the plot's y-axis label.
	fn with_y_label<T>(self, label: T) -> Self
	where
		T: Display,
	;

	/// Configures the supplied `Gnuplot` `Axes2D` with the
	/// plot's data.
	fn configure(&mut self, axes: &mut Axes2D);
}

trait Scaler {
	fn new(max: impl AsPrimitive<u64>) -> Self
	where
		Self: Sized,
	;

	fn scale(&self, size: f64) -> f64;
	fn apply_unit(&self, label: &str) -> String;
}

struct NoScaler;

struct SizeScaler {
	unit: Option<&'static str>,
	denominator: f64,
}

struct TimeScaler {
	unit: Option<&'static str>,
	denominator: f64,
}

impl Scaler for NoScaler {
	fn new(_: impl AsPrimitive<u64>) -> Self {
		NoScaler
	}

	fn scale(&self, size: f64) -> f64 {
		size
	}

	fn apply_unit(&self, label: &str) -> String {
		label.to_owned()
	}
}

impl Scaler for SizeScaler {
	fn new(max_size: impl AsPrimitive<u64>) -> Self {
		let mut max_size = max_size.as_();
		let mut count: usize = 0;
		let mut denominator: f64 = 1.0;

		while max_size / 1024 > 0 {
			denominator *= 1024.0;
			max_size /= 1024;
			count += 1;
		}

		SizeScaler {
			unit: Some(MEMORY_UNITS[count]),
			denominator,
		}
	}

	fn scale(&self, size: f64) -> f64 {
		size / self.denominator
	}

	fn apply_unit(&self, label: &str) -> String {
		match self.unit {
			Some(unit) => format!("{label} ({unit})"),
			None => label.to_string(),
		}
	}
}

impl Scaler for TimeScaler {
	fn new(max_time: impl AsPrimitive<u64>) -> Self {
		let mut max_time = max_time.as_();
		let mut count: usize = 0;

		let divisors: &[u64] = &[1000, 60, 60, 24];
		let mut denominator: f64 = 1.0;

		for divisor in divisors {
			if max_time / divisor == 0 {
				break;
			}

			denominator *= *divisor as f64;
			max_time /= divisor;
			count += 1;
		}

		TimeScaler {
			unit: Some(TIME_UNITS[count]),
			denominator,
		}
	}

	fn scale(&self, time: f64) -> f64 {
		time / self.denominator
	}

	fn apply_unit(&self, label: &str) -> String {
		match self.unit {
			Some(unit) => format!("{label} ({unit})"),
			None => label.to_string(),
		}
	}
}

fn auto_option(value: Option<f64>, scaler: &dyn Scaler) -> AutoOption<f64> {
	match value {
		Some(value) => AutoOption::Fix(scaler.scale(value)),
		None => AutoOption::Auto,
	}
}

pub use crate::plot::figure::Figure;
