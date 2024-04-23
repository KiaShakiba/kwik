/*
 * Copyright (c) Kia Shakiba
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

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
};

use crate::plot::{Plot, auto_option, COLORS, DASH_TYPES};

/// A line plot.
#[derive(Default)]
pub struct LinePlot {
	title: Option<String>,

	x_label: Option<String>,
	y_label: Option<String>,

	x_min: Option<f64>,
	x_max: Option<f64>,

	y_min: Option<f64>,
	y_max: Option<f64>,

	x_tick: Option<f64>,
	y_tick: Option<f64>,

	lines: Vec<Line>,

	hlines: Vec<f64>,
	points: Vec<(f64, f64)>,
}

/// An individual line on a line plot.
#[derive(Default)]
pub struct Line {
	label: Option<String>,

	x_values: Vec<f64>,
	y_values: Vec<f64>,
}

impl Plot for LinePlot {
	fn is_empty(&self) -> bool {
		if self.lines.is_empty() {
			// there are no lines in the plot
			return true;
		}

		// there are lines in the plot, though they all may be empty
		self.lines
			.iter()
			.all(|line| line.is_empty())
	}

	fn set_title(&mut self, title: &str) {
		self.title = Some(title.into());
	}

	fn with_title(mut self, title: &str) -> Self {
		self.set_title(title);
		self
	}

	fn set_x_label(&mut self, label: &str) {
		self.x_label = Some(label.into());
	}

	fn with_x_label(mut self, label: &str) -> Self {
		self.set_x_label(label);
		self
	}

	fn set_y_label(&mut self, label: &str) {
		self.y_label = Some(label.into());
	}

	fn with_y_label(mut self, label: &str) -> Self {
		self.set_y_label(label);
		self
	}

	fn configure(&mut self, axes: &mut Axes2D) {
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
				auto_option(self.x_min),
				auto_option(self.x_max),
			)
			.set_y_range(
				auto_option(self.y_min),
				auto_option(self.y_max),
			)
			.set_x_ticks(
				Some((auto_option(self.x_tick), 0)),
				&[
					TickOption::Mirror(false),
					TickOption::Inward(false),
				],
				&[]
			)
			.set_y_ticks(
				Some((auto_option(self.y_tick), 0)),
				&[
					TickOption::Mirror(false),
					TickOption::Inward(false),
				],
				&[]
			)
			.set_grid_options(false, &[
				Color("#bbbbbb"),
				LineWidth(2.0),
				LineStyle(DashType::Dot),
			])
			.set_x_grid(true)
			.set_y_grid(true);

		if let Some(title) = &self.title {
			axes.set_title(title, &[]);
		}

		if let Some(label) = &self.x_label {
			axes.set_x_label(label, &[]);
		}

		if let Some(label) = &self.y_label {
			axes.set_y_label(label, &[]);
		}

		for (index, line) in self.lines.iter().enumerate() {
			let mut line_config = vec![
				LineWidth(4.0),
				Color(COLORS[index % COLORS.len()]),
				LineStyle(DASH_TYPES[index & COLORS.len()]),
			];

			if let Some(label) = &line.label {
				line_config.push(Caption(label));
			}

			axes.lines(&line.x_values, &line.y_values, &line_config);
		}

		for hline_x in &self.hlines {
			let x = vec![hline_x, hline_x];
			let y = vec![self.min_y(), self.max_y()];

			axes.lines(x, y, &[
				LineWidth(2.0),
				Color("blue"),
			]);
		}

		for (x_value, y_value) in &self.points {
			let x = vec![x_value];
			let y = vec![y_value];

			axes.points(x, y, &[
				PointSymbol('o'),
				PointSize(1.0),
				Color("#009600"),
			]);
		}
	}
}

impl LinePlot {
	fn min_y(&self) -> f64 {
		let mut min: Option<f64> = None;

		for line in &self.lines {
			let line_min = line.y_values
				.iter()
				.min_by(|a, b| a.total_cmp(b))
				.copied()
				.unwrap_or(0.0);

			if min.is_none() || min.is_some_and(|value| value > line_min) {
				min = Some(line_min);
			}
		}

		min.unwrap_or(0.0)
	}

	fn max_y(&self) -> f64 {
		let mut max: Option<f64> = None;

		for line in &self.lines {
			let line_max = line.y_values
				.iter()
				.max_by(|a, b| a.total_cmp(b))
				.copied()
				.unwrap_or(0.0);

			if max.is_none() || max.is_some_and(|value| value < line_max) {
				max = Some(line_max);
			}
		}

		max.unwrap_or(0.0)
	}

	/// Sets the plot's minimum x-value.
	pub fn set_x_min(&mut self, x_min: f64) {
		self.x_min = Some(x_min);
	}

	/// Sets the plot's minimum x-value.
	pub fn with_x_min(mut self, x_min: f64) -> Self {
		self.x_min = Some(x_min);
		self
	}

	/// Sets the plot's maximum x-value.
	pub fn set_x_max(&mut self, x_max: f64) {
		self.x_max = Some(x_max);
	}

	/// Sets the plot's maximum x-value.
	pub fn with_x_max(mut self, x_max: f64) -> Self {
		self.x_max = Some(x_max);
		self
	}

	/// Sets the plot's minimum y-value.
	pub fn set_y_min(&mut self, y_min: f64) {
		self.y_min = Some(y_min);
	}

	/// Sets the plot's minimum y-value.
	pub fn with_y_min(mut self, y_min: f64) -> Self {
		self.y_min = Some(y_min);
		self
	}

	/// Sets the plot's maximum y-value.
	pub fn set_y_max(&mut self, y_max: f64) {
		self.y_max = Some(y_max);
	}

	/// Sets the plot's maximum y-value.
	pub fn with_y_max(mut self, y_max: f64) -> Self {
		self.y_max = Some(y_max);
		self
	}

	/// Sets the plot's x-tick value.
	pub fn set_x_tick(&mut self, x_tick: f64) {
		self.x_tick = Some(x_tick);
	}

	/// Sets the plot's x-tick value.
	pub fn with_x_tick(mut self, x_tick: f64) -> Self {
		self.x_tick = Some(x_tick);
		self
	}

	/// Sets the plot's y-tick value.
	pub fn set_y_tick(&mut self, y_tick: f64) {
		self.y_tick = Some(y_tick);
	}

	/// Sets the plot's y-tick value.
	pub fn with_y_tick(mut self, y_tick: f64) -> Self {
		self.y_tick = Some(y_tick);
		self
	}

	/// Adds a line to the plot.
	pub fn line(&mut self, line: Line) {
		self.lines.push(line);
	}

	/// Adds a horizontal line to the plot at the supplied x-value.
	pub fn hline(&mut self, x_value: f64) {
		self.hlines.push(x_value);
	}

	/// Adds a point to the plot at the supplied coordinates.
	pub fn point(&mut self, x_value: f64, y_value: f64) {
		self.points.push((x_value, y_value));
	}
}

impl Line {
	/// Checks if the line is empty.
	pub fn is_empty(&self) -> bool {
		self.x_values.is_empty()
	}

	/// Set the line's label.
	pub fn set_label(&mut self, label: &str) {
		self.label = Some(label.into());
	}

	/// Set the line's label.
	pub fn with_label(mut self, label: &str) -> Self {
		self.set_label(label);
		self
	}

	/// Adds a data point to the line.
	pub fn push(&mut self, x: f64, y: f64) {
		self.x_values.push(x);
		self.y_values.push(y);
	}
}
