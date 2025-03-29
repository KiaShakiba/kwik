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
	Caption,
	Color,
	LineWidth,
	LineStyle,
	DashType,
	BorderLocation2D,
	TickOption,
	PointSymbol,
	PointSize,
	LabelOption,
	PlotOption,
	XAxis,
	YAxis,
};

use crate::plot::{
	Plot,
	Scaler,
	NoScaler,
	SizeScaler,
	TimeScaler,
	auto_option,
	COLORS,
	DASH_TYPES,
};

/// A line plot.
#[derive(Default, Clone)]
pub struct LinePlot {
	font_type: Option<String>,
	font_size: Option<f64>,

	title: Option<String>,

	x_label: Option<String>,
	y_label: Option<String>,
	y2_label: Option<String>,

	x_min: Option<f64>,
	x_max: Option<f64>,

	y_min: Option<f64>,
	y_max: Option<f64>,
	y2_min: Option<f64>,
	y2_max: Option<f64>,

	x_tick: Option<f64>,
	y_tick: Option<f64>,
	y2_tick: Option<f64>,

	format_x_log: bool,
	format_y_log: bool,
	format_y2_log: bool,

	format_x_memory: bool,
	format_y_memory: bool,
	format_y2_memory: bool,

	format_x_time: bool,
	format_y_time: bool,
	format_y2_time: bool,

	y1_lines: Vec<Line>,
	y2_lines: Vec<Line>,

	vlines: Vec<f64>,
	hlines: Vec<f64>,

	points: Vec<(f64, f64)>,
}

/// An individual line on a line plot.
#[derive(Clone)]
pub struct Line {
	label: Option<String>,
	width: f64,

	x_values: Vec<f64>,
	y_values: Vec<f64>,

	y2_axis: bool,
}

impl Plot for LinePlot {
	fn is_empty(&self) -> bool {
		if self.y1_lines.is_empty() && self.y2_lines.is_empty() {
			// there are no lines in the plot
			return true;
		}

		// there are lines in the plot, though they all may be empty
		let y1_lines_empty = self.y1_lines
			.iter()
			.all(|line| line.is_empty());

		let y2_lines_empty = self.y2_lines
			.iter()
			.all(|line| line.is_empty());

		y1_lines_empty && y2_lines_empty
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

		let x_scaler = self.x_scaler();
		let y_scaler = self.y_scaler();
		let y2_scaler = self.y2_scaler();

		axes
			.set_border(
				false,
				&[
					BorderLocation2D::Bottom,
					BorderLocation2D::Left,
				],
				&[]
			)
			.set_x_range(
				auto_option(self.x_min, x_scaler.as_ref()),
				auto_option(self.x_max, x_scaler.as_ref()),
			)
			.set_y_range(
				auto_option(self.y_min, y_scaler.as_ref()),
				auto_option(self.y_max, y_scaler.as_ref()),
			)
			.set_x_ticks(
				Some((auto_option(self.x_tick, x_scaler.as_ref()), 0)),
				&[TickOption::Mirror(false), TickOption::Inward(false)],
				&[font],
			)
			.set_y_ticks(
				Some((auto_option(self.y_tick, y_scaler.as_ref()), 0)),
				&[TickOption::Mirror(false), TickOption::Inward(false)],
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
			axes.set_x_label(&x_scaler.apply_unit(x_label), &[font]);
		}

		if let Some(y_label) = &self.y_label {
			axes.set_y_label(&y_scaler.apply_unit(y_label), &[font]);
		}

		if self.format_x_log {
			axes.set_x_log(Some(10.0));
		}

		if self.format_y_log {
			axes.set_y_log(Some(10.0));
		}

		if !self.y2_lines.is_empty() {
			axes.set_y2_range(
				auto_option(self.y2_min, y2_scaler.as_ref()),
				auto_option(self.y2_max, y2_scaler.as_ref()),
			);

			axes.set_y2_ticks(
				Some((auto_option(self.y2_tick, y2_scaler.as_ref()), 0)),
				&[TickOption::Mirror(false), TickOption::Inward(false)],
				&[font],
			);

			if let Some(y2_label) = &self.y2_label {
				axes.set_y2_label(&y2_scaler.apply_unit(y2_label), &[font]);
			}

			if self.format_y2_log {
				axes.set_y2_log(Some(10.0));
			}
		}

		for (index, line) in self.y1_lines.iter().enumerate() {
			let mut line_config = vec![
				LineWidth(line.width),
				Color(COLORS[index % COLORS.len()]),
				LineStyle(DASH_TYPES[index % DASH_TYPES.len()]),
			];

			if let Some(label) = &line.label {
				line_config.push(Caption(label));
			}

			let x_values = line.x_values
				.iter()
				.map(|value| x_scaler.scale(*value));

			let y_values = line.y_values
				.iter()
				.map(|value| y_scaler.scale(*value));

			axes.lines(x_values, y_values, &line_config);
		}

		for (index, line) in self.y2_lines.iter().enumerate() {
			let global_index = self.y1_lines.len() + index;

			let mut line_config = vec![
				LineWidth(line.width),
				Color(COLORS[global_index % COLORS.len()]),
				LineStyle(DASH_TYPES[global_index % DASH_TYPES.len()]),
				PlotOption::Axes(XAxis::X1, YAxis::Y2),
			];

			if let Some(label) = &line.label {
				line_config.push(Caption(label));
			}

			let x_values = line.x_values
				.iter()
				.map(|value| x_scaler.scale(*value));

			let y_values = line.y_values
				.iter()
				.map(|value| y2_scaler.scale(*value));

			axes.lines(x_values, y_values, &line_config);
		}

		for vline_x in &self.vlines {
			let x = vec![
				x_scaler.scale(*vline_x),
				x_scaler.scale(*vline_x),
			];

			let y = vec![
				y_scaler.scale(self.min_y_value()),
				y_scaler.scale(self.max_y_value()),
			];

			axes.lines(x, y, &[
				LineWidth(2.0),
				Color("blue"),
			]);
		}

		for hline_y in &self.hlines {
			let x = vec![
				x_scaler.scale(self.min_x_value()),
				x_scaler.scale(self.max_x_value()),
			];

			let y = vec![
				y_scaler.scale(*hline_y),
				y_scaler.scale(*hline_y),
			];

			axes.lines(x, y, &[
				LineWidth(2.0),
				Color("blue"),
			]);
		}

		for (x_value, y_value) in &self.points {
			let x = vec![x_scaler.scale(*x_value)];
			let y = vec![y_scaler.scale(*y_value)];

			axes.points(x, y, &[
				PointSymbol('o'),
				PointSize(1.0),
				Color("#009600"),
			]);
		}
	}
}

impl LinePlot {
	/// Sets the plot's y2-axis label.
	pub fn set_y2_label<T>(&mut self, label: T)
	where
		T: Display,
	{
		self.y2_label = Some(label.to_string());
	}

	/// Sets the plot's y2-axis label.
	pub fn with_y2_label<T>(mut self, label: T) -> Self
	where
		T: Display,
	{
		self.set_y2_label(label);
		self
	}

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

	/// Sets the plot's minimum y2-value.
	pub fn set_y2_min(&mut self, y2_min: impl AsPrimitive<f64>) {
		self.y2_min = Some(y2_min.as_());
	}

	/// Sets the plot's minimum y2-value.
	pub fn with_y2_min(mut self, y2_min: impl AsPrimitive<f64>) -> Self {
		self.set_y2_min(y2_min);
		self
	}

	/// Sets the plot's maximum y2-value.
	pub fn set_y2_max(&mut self, y2_max: impl AsPrimitive<f64>) {
		self.y_max = Some(y2_max.as_());
	}

	/// Sets the plot's maximum y2-value.
	pub fn with_y2_max(mut self, y2_max: impl AsPrimitive<f64>) -> Self {
		self.set_y2_max(y2_max);
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

	/// Sets the plot's y2-tick value.
	pub fn set_y2_tick(&mut self, y2_tick: impl AsPrimitive<f64>) {
		self.y2_tick = Some(y2_tick.as_());
	}

	/// Sets the plot's y2-tick value.
	pub fn with_y2_tick(mut self, y2_tick: impl AsPrimitive<f64>) -> Self {
		self.set_y2_tick(y2_tick);
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

	/// Enables or disables logarithmic formatting in the y2-axis.
	pub fn set_format_y2_log(&mut self, value: bool) {
		self.format_y2_log = value;
	}

	/// Enables or disables logarithmic formatting in the y2-axis.
	pub fn with_format_y2_log(mut self, value: bool) -> Self {
		self.set_format_y2_log(value);
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

	/// Enables or disables memory formatting in the y2-axis.
	pub fn set_format_y2_memory(&mut self, value: bool) {
		self.format_y2_memory = value;
	}

	/// Enables or disables memory formatting in the y2-axis.
	pub fn with_format_y2_memory(mut self, value: bool) -> Self {
		self.set_format_y2_memory(value);
		self
	}

	/// Enables or disables time formatting in the x-axis.
	pub fn set_format_x_time(&mut self, value: bool) {
		self.format_x_time = value;
	}

	/// Enables or disables time formatting in the x-axis.
	pub fn with_format_x_time(mut self, value: bool) -> Self {
		self.set_format_x_time(value);
		self
	}

	/// Enables or disables time formatting in the y-axis.
	pub fn set_format_y_time(&mut self, value: bool) {
		self.format_y_time = value;
	}

	/// Enables or disables time formatting in the y-axis.
	pub fn with_format_y_time(mut self, value: bool) -> Self {
		self.set_format_y_time(value);
		self
	}

	/// Enables or disables time formatting in the y2-axis.
	pub fn set_format_y2_time(&mut self, value: bool) {
		self.format_y2_time = value;
	}

	/// Enables or disables time formatting in the y2-axis.
	pub fn with_format_y2_time(mut self, value: bool) -> Self {
		self.set_format_y2_time(value);
		self
	}

	/// Adds a line to the plot.
	pub fn line(&mut self, line: Line) {
		if !line.y2_axis {
			self.y1_lines.push(line);
		} else {
			self.y2_lines.push(line);
		}
	}

	/// Adds a vertical line to the plot at the supplied x-value.
	pub fn vline(&mut self, x_value: impl AsPrimitive<f64>) {
		self.vlines.push(x_value.as_());
	}

	/// Adds a horizontal line to the plot at the supplied y-value.
	pub fn hline(&mut self, y_value: impl AsPrimitive<f64>) {
		self.hlines.push(y_value.as_());
	}

	/// Adds a point to the plot at the supplied coordinates.
	pub fn point(&mut self, x_value: impl AsPrimitive<f64>, y_value: impl AsPrimitive<f64>) {
		self.points.push((x_value.as_(), y_value.as_()));
	}

	fn min_x_value(&self) -> f64 {
		let mut min = self.x_min;

		for line in &self.y1_lines {
			let line_min = line.x_values
				.iter()
				.min_by(|a, b| a.total_cmp(b))
				.copied()
				.unwrap_or(0.0);

			if min.is_none() || min.is_some_and(|value| value > line_min) {
				min = Some(line_min);
			}
		}

		for line in &self.y2_lines {
			let line_min = line.x_values
				.iter()
				.min_by(|a, b| a.total_cmp(b))
				.copied()
				.unwrap_or(0.0);

			if min.is_none() || min.is_some_and(|value| value > line_min) {
				min = Some(line_min);
			}
		}

		for vline_x in &self.vlines {
			if min.is_none() || min.is_some_and(|value| value > *vline_x) {
				min = Some(*vline_x);
			}
		}

		min.unwrap_or(0.0)
	}

	fn max_x_value(&self) -> f64 {
		let mut max = self.x_max;

		for line in &self.y1_lines {
			let line_max = line.x_values
				.iter()
				.max_by(|a, b| a.total_cmp(b))
				.copied()
				.unwrap_or(0.0);

			if max.is_none() || max.is_some_and(|value| value < line_max) {
				max = Some(line_max);
			}
		}

		for line in &self.y2_lines {
			let line_max = line.x_values
				.iter()
				.max_by(|a, b| a.total_cmp(b))
				.copied()
				.unwrap_or(0.0);

			if max.is_none() || max.is_some_and(|value| value < line_max) {
				max = Some(line_max);
			}
		}

		for vline_x in &self.vlines {
			if max.is_none() || max.is_some_and(|value| value < *vline_x) {
				max = Some(*vline_x);
			}
		}

		max.unwrap_or(0.0)
	}

	fn min_y_value(&self) -> f64 {
		let mut min = self.y_min;

		for line in &self.y1_lines {
			let line_min = line.y_values
				.iter()
				.min_by(|a, b| a.total_cmp(b))
				.copied()
				.unwrap_or(0.0);

			if min.is_none() || min.is_some_and(|value| value > line_min) {
				min = Some(line_min);
			}
		}

		for hline_y in &self.hlines {
			if min.is_none() || min.is_some_and(|value| value > *hline_y) {
				min = Some(*hline_y);
			}
		}

		min.unwrap_or(0.0)
	}

	fn max_y_value(&self) -> f64 {
		let mut max = self.y_max;

		for line in &self.y1_lines {
			let line_max = line.y_values
				.iter()
				.max_by(|a, b| a.total_cmp(b))
				.copied()
				.unwrap_or(0.0);

			if max.is_none_or(|value| value < line_max) {
				max = Some(line_max);
			}
		}

		for hline_y in &self.hlines {
			if max.is_none_or(|value| value < *hline_y) {
				max = Some(*hline_y);
			}
		}

		max.unwrap_or(0.0)
	}

	fn max_y2_value(&self) -> f64 {
		let mut max = self.y2_max;

		for line in &self.y2_lines {
			let line_max = line.y_values
				.iter()
				.max_by(|a, b| a.total_cmp(b))
				.copied()
				.unwrap_or(0.0);

			if max.is_none() || max.is_some_and(|value| value < line_max) {
				max = Some(line_max);
			}
		}

		for hline_y in &self.hlines {
			if max.is_none() || max.is_some_and(|value| value < *hline_y) {
				max = Some(*hline_y);
			}
		}

		max.unwrap_or(0.0)
	}

	fn x_scaler(&self) -> Box<dyn Scaler> {
		let max_x_value = self.max_x_value();

		if self.format_x_memory {
			return Box::new(SizeScaler::new(max_x_value));
		}

		if self.format_x_time {
			return Box::new(TimeScaler::new(max_x_value));
		}

		Box::new(NoScaler::new(max_x_value))

	}

	fn y_scaler(&self) -> Box<dyn Scaler> {
		let max_y_value = self.max_y_value();

		if self.format_y_memory {
			return Box::new(SizeScaler::new(max_y_value));
		}

		if self.format_y_time {
			return Box::new(TimeScaler::new(max_y_value));
		}

		Box::new(NoScaler::new(max_y_value))
	}

	fn y2_scaler(&self) -> Box<dyn Scaler> {
		let max_y2_value = self.max_y2_value();

		if self.format_y2_memory {
			return Box::new(SizeScaler::new(max_y2_value));
		}

		if self.format_y2_time {
			return Box::new(TimeScaler::new(max_y2_value));
		}

		Box::new(NoScaler::new(max_y2_value))
	}
}

impl Line {
	/// Returns `true` if the line is empty.
	pub fn is_empty(&self) -> bool {
		self.x_values.is_empty()
	}

	/// Sets the line's label.
	pub fn set_label<T>(&mut self, label: T)
	where
		T: Display,
	{
		self.label = Some(label.to_string());
	}

	/// Sets the line's label.
	pub fn with_label<T>(mut self, label: T) -> Self
	where
		T: Display,
	{
		self.set_label(label);
		self
	}

	/// Sets the line's width.
	pub fn set_width(&mut self, width: impl AsPrimitive<f64>) {
		self.width = width.as_();
	}

	/// Sets the line's width.
	pub fn with_width(mut self, width: impl AsPrimitive<f64>) -> Self {
		self.set_width(width);
		self
	}

	/// Assigns the line to the y2-axis.
	pub fn set_y2_axis(&mut self) {
		self.y2_axis = true;
	}

	/// Assigns the line to the y2-axis.
	pub fn with_y2_axis(mut self) -> Self {
		self.set_y2_axis();
		self
	}

	/// Adds a data point to the line.
	pub fn push(&mut self, x: impl AsPrimitive<f64>, y: impl AsPrimitive<f64>) {
		self.x_values.push(x.as_());
		self.y_values.push(y.as_());
	}
}

impl Default for Line {
	fn default() -> Self {
		Line {
			label: None,
			width: 2.0,

			x_values: Vec::new(),
			y_values: Vec::new(),

			y2_axis: false,
		}
	}
}
