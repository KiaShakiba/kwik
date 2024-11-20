/*
 * Copyright (c) Kia Shakiba
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

use std::fmt::Display;
use num_traits::AsPrimitive;

use gnuplot::{
	Axes2D,
	AxesCommon,
	PlotOption,
	Color,
	LineWidth,
	LineStyle,
	DashType,
	BorderLocation2D,
	TickOption,
	LabelOption,
};

use crate::plot::{Plot, auto_option};

/// A scatter plot.
#[derive(Default, Clone)]
pub struct ScatterPlot {
	font_type: Option<String>,
	font_size: Option<f64>,

	title: Option<String>,

	x_label: Option<String>,
	y_label: Option<String>,

	x_min: Option<f64>,
	x_max: Option<f64>,

	y_min: Option<f64>,
	y_max: Option<f64>,

	x_tick: Option<f64>,
	y_tick: Option<f64>,

	format_x_log: bool,
	format_y_log: bool,

	format_x_memory: bool,
	format_y_memory: bool,

	points: Vec<(f64, f64)>,
}

impl Plot for ScatterPlot {
	fn is_empty(&self) -> bool {
		self.points.is_empty()
	}

	fn set_font_type(&mut self, font_type: &str) {
		self.font_type = Some(font_type.into());
	}

	fn with_font_type(mut self, font_type: &str) -> Self {
		self.set_font_type(font_type);
		self
	}

	fn set_font_size(&mut self, font_size: impl AsPrimitive<f64>) {
		self.font_size = Some(font_size.as_());
	}

	fn with_font_size(mut self, font_size: impl AsPrimitive<f64>) -> Self {
		self.set_font_size(font_size);
		self
	}

	fn set_title<T>(&mut self, title: T)
	where
		T: Display,
	{
		self.title = Some(title.to_string());
	}

	fn with_title<T>(mut self, title: T) -> Self
	where
		T: Display,
	{
		self.set_title(title);
		self
	}

	fn set_x_label<T>(&mut self, label: T)
	where
		T: Display,
	{
		self.x_label = Some(label.to_string());
	}

	fn with_x_label<T>(mut self, label: T) -> Self
	where
		T: Display,
	{
		self.set_x_label(label);
		self
	}

	fn set_y_label<T>(&mut self, label: T)
	where
		T: Display,
	{
		self.y_label = Some(label.to_string());
	}

	fn with_y_label<T>(mut self, label: T) -> Self
	where
		T: Display,
	{
		self.set_y_label(label);
		self
	}

	fn configure(&mut self, axes: &mut Axes2D) {
		let font = LabelOption::Font(
			self.font_type.as_deref().unwrap_or("Arial"),
			self.font_size.unwrap_or(16.0),
		);

		let mut x_tick_options = vec![
			TickOption::Mirror(false),
			TickOption::Inward(false),
		];

		let mut y_tick_options = vec![
			TickOption::Mirror(false),
			TickOption::Inward(false),
		];

		if self.format_x_memory {
			x_tick_options.push(TickOption::Format("%.1s %cB"));
		}

		if self.format_y_memory {
			y_tick_options.push(TickOption::Format("%.1s %cB"));
		}

		axes
			.set_border(
				false,
				&[
					BorderLocation2D::Top,
					BorderLocation2D::Right,
					BorderLocation2D::Bottom,
					BorderLocation2D::Left,
				],
				&[]
			)
			.set_x_range(
				auto_option(self.x_min),
				auto_option(self.x_max),
			)
			.set_y_range(
				auto_option(self.y_min),
				auto_option(self.y_max),
			)
			.set_x_ticks(
				Some((auto_option(self.x_tick), 0)),
				&x_tick_options,
				&[font],
			)
			.set_y_ticks(
				Some((auto_option(self.y_tick), 0)),
				&y_tick_options,
				&[font],
			)
			.set_grid_options(false, &[
				Color("#bbbbbb"),
				LineWidth(2.0),
				LineStyle(DashType::Dot),
			])
			.set_x_grid(true)
			.set_y_grid(true);

		if let Some(title) = &self.title {
			axes.set_title(title, &[font]);
		}

		if let Some(x_label) = &self.x_label {
			axes.set_x_label(x_label, &[font]);
		}

		if let Some(y_label) = &self.y_label {
			axes.set_y_label(y_label, &[font]);
		}

		if self.format_x_log {
			axes.set_x_log(Some(10.0));
		}

		if self.format_y_log {
			axes.set_y_log(Some(10.0));
		}

		let mut x_values = Vec::<f64>::new();
		let mut y_values = Vec::<f64>::new();

		for (x_value, y_value) in &self.points {
			x_values.push(*x_value);
			y_values.push(*y_value);
		}

		axes.points(
			x_values,
			y_values,
			&[
				PlotOption::Color("red"),
				PlotOption::PointSymbol('o'),
				PlotOption::PointSize(1.0),
			]
		);
	}
}

impl ScatterPlot {
	/// Sets the plot's minimum x-value.
	pub fn set_x_min(&mut self, x_min: impl AsPrimitive<f64>) {
		self.x_min = Some(x_min.as_());
	}

	/// Sets the plot's minimum x-value.
	pub fn with_x_min(mut self, x_min: impl AsPrimitive<f64>) -> Self {
		self.set_x_min(x_min);
		self
	}

	/// Sets the plot's maximum x-value.
	pub fn set_x_max(&mut self, x_max: impl AsPrimitive<f64>) {
		self.x_max = Some(x_max.as_());
	}

	/// Sets the plot's maximum x-value.
	pub fn with_x_max(mut self, x_max: impl AsPrimitive<f64>) -> Self {
		self.set_x_max(x_max);
		self
	}

	/// Sets the plot's minimum y-value.
	pub fn set_y_min(&mut self, y_min: impl AsPrimitive<f64>) {
		self.y_min = Some(y_min.as_());
	}

	/// Sets the plot's minimum y-value.
	pub fn with_y_min(mut self, y_min: impl AsPrimitive<f64>) -> Self {
		self.set_y_min(y_min);
		self
	}

	/// Sets the plot's maximum y-value.
	pub fn set_y_max(&mut self, y_max: impl AsPrimitive<f64>) {
		self.y_max = Some(y_max.as_());
	}

	/// Sets the plot's maximum y-value.
	pub fn with_y_max(mut self, y_max: impl AsPrimitive<f64>) -> Self {
		self.set_y_max(y_max);
		self
	}

	/// Sets the plot's x-tick value.
	pub fn set_x_tick(&mut self, x_tick: impl AsPrimitive<f64>) {
		self.x_tick = Some(x_tick.as_());
	}

	/// Sets the plot's x-tick value.
	pub fn with_x_tick(mut self, x_tick: impl AsPrimitive<f64>) -> Self {
		self.set_x_tick(x_tick);
		self
	}

	/// Sets the plot's y-tick value.
	pub fn set_y_tick(&mut self, y_tick: impl AsPrimitive<f64>) {
		self.y_tick = Some(y_tick.as_());
	}

	/// Sets the plot's y-tick value.
	pub fn with_y_tick(mut self, y_tick: impl AsPrimitive<f64>) -> Self {
		self.set_y_tick(y_tick);
		self
	}

	/// Enables or disables logarithmic formatting in the x-axis.
	pub fn set_format_x_log(&mut self, value: bool) {
		self.format_x_log = value;
	}

	/// Enables or disables logarithmic formatting in the x-axis.
	pub fn with_format_x_log(mut self, value: bool) -> Self {
		self.set_format_x_log(value);
		self
	}

	/// Enables or disables logarithmic formatting in the y-axis.
	pub fn set_format_y_log(&mut self, value: bool) {
		self.format_y_log = value;
	}

	/// Enables or disables logarithmic formatting in the y-axis.
	pub fn with_format_y_log(mut self, value: bool) -> Self {
		self.set_format_y_log(value);
		self
	}

	/// Enables or disables memory formatting in the x-axis.
	pub fn set_format_x_memory(&mut self, value: bool) {
		self.format_x_memory = value;
	}

	/// Enables or disables memory formatting in the x-axis.
	pub fn with_format_x_memory(mut self, value: bool) -> Self {
		self.set_format_x_memory(value);
		self
	}

	/// Enables or disables memory formatting in the y-axis.
	pub fn set_format_y_memory(&mut self, value: bool) {
		self.format_y_memory = value;
	}

	/// Enables or disables memory formatting in the y-axis.
	pub fn with_format_y_memory(mut self, value: bool) -> Self {
		self.set_format_y_memory(value);
		self
	}

	/// Adds a point to the plot at the supplied coordinates.
	pub fn point(&mut self, x_value: impl AsPrimitive<f64>, y_value: impl AsPrimitive<f64>) {
		self.points.push((x_value.as_(), y_value.as_()));
	}
}
