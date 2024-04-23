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

use gnuplot::{Axes2D, AutoOption, DashType};

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

/// Implementing this trait allows the struct to be added to a
/// plot figure.
pub trait Plot {
	/// Checks if the plot is empty (i.e., has no data).
	fn is_empty(&self) -> bool;

	/// Sets the plot's title
	fn set_title(&mut self, title: &str);

	/// Sets the plot's title
	fn with_title(self, title: &str) -> Self;

	/// Sets the plot's x-axis label
	fn set_x_label(&mut self, label: &str);

	/// Sets the plot's x-axis label
	fn with_x_label(self, label: &str) -> Self;

	/// Sets the plot's y-axis label
	fn set_y_label(&mut self, label: &str);

	/// Sets the plot's y-axis label
	fn with_y_label(self, label: &str) -> Self;

	/// Configures the supplied `Gnuplot` `Axes2D` with the
	/// plot's data.
	fn configure(&mut self, axes: &mut Axes2D);
}

fn auto_option(value: Option<f64>) -> AutoOption<f64> {
	match value {
		Some(value) => AutoOption::Fix(value),
		None => AutoOption::Auto,
	}
}

pub use crate::plot::figure::Figure;
