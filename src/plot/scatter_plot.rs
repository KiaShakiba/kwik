/*
 * Copyright (c) Kia Shakiba
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

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
};

use crate::plot::{
	Plot,
	auto_option,
	value::{PlotValue, ToPlotValue},
};

/// A scatter plot.
#[derive(Default, Clone)]
pub struct ScatterPlot {
	title: Option<String>,

	x_label: Option<String>,
	y_label: Option<String>,

	x_min: Option<PlotValue>,
	x_max: Option<PlotValue>,

	y_min: Option<PlotValue>,
	y_max: Option<PlotValue>,

	x_tick: Option<PlotValue>,
	y_tick: Option<PlotValue>,

	format_x_log: bool,
	format_y_log: bool,

	points: Vec<(PlotValue, PlotValue)>,
}

impl Plot for ScatterPlot {
	fn is_empty(&self) -> bool {
		self.points.is_empty()
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
				&[
					TickOption::Mirror(true),
					TickOption::Inward(true),
				],
				&[]
			)
			.set_y_ticks(
				Some((auto_option(self.y_tick), 0)),
				&[
					TickOption::Mirror(true),
					TickOption::Inward(true),
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

		if let Some(x_label) = &self.x_label {
			axes.set_x_label(x_label, &[]);
		}

		if let Some(y_label) = &self.y_label {
			axes.set_y_label(y_label, &[]);
		}

		if self.format_x_log {
			axes.set_x_log(Some(10.0));
		}

		if self.format_y_log {
			axes.set_y_log(Some(10.0));
		}

		let mut x_values = Vec::<PlotValue>::new();
		let mut y_values = Vec::<PlotValue>::new();

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
	pub fn set_x_min(&mut self, x_min: impl ToPlotValue) {
		self.x_min = Some(x_min.to_plot_value());
	}

	/// Sets the plot's minimum x-value.
	pub fn with_x_min(mut self, x_min: impl ToPlotValue) -> Self {
		self.x_min = Some(x_min.to_plot_value());
		self
	}

	/// Sets the plot's maximum x-value.
	pub fn set_x_max(&mut self, x_max: impl ToPlotValue) {
		self.x_max = Some(x_max.to_plot_value());
	}

	/// Sets the plot's maximum x-value.
	pub fn with_x_max(mut self, x_max: impl ToPlotValue) -> Self {
		self.x_max = Some(x_max.to_plot_value());
		self
	}

	/// Sets the plot's minimum y-value.
	pub fn set_y_min(&mut self, y_min: impl ToPlotValue) {
		self.y_min = Some(y_min.to_plot_value());
	}

	/// Sets the plot's minimum y-value.
	pub fn with_y_min(mut self, y_min: impl ToPlotValue) -> Self {
		self.y_min = Some(y_min.to_plot_value());
		self
	}

	/// Sets the plot's maximum y-value.
	pub fn set_y_max(&mut self, y_max: impl ToPlotValue) {
		self.y_max = Some(y_max.to_plot_value());
	}

	/// Sets the plot's maximum y-value.
	pub fn with_y_max(mut self, y_max: impl ToPlotValue) -> Self {
		self.y_max = Some(y_max.to_plot_value());
		self
	}

	/// Sets the plot's x-tick value.
	pub fn set_x_tick(&mut self, x_tick: impl ToPlotValue) {
		self.x_tick = Some(x_tick.to_plot_value());
	}

	/// Sets the plot's x-tick value.
	pub fn with_x_tick(mut self, x_tick: impl ToPlotValue) -> Self {
		self.x_tick = Some(x_tick.to_plot_value());
		self
	}

	/// Sets the plot's y-tick value.
	pub fn set_y_tick(&mut self, y_tick: impl ToPlotValue) {
		self.y_tick = Some(y_tick.to_plot_value());
	}

	/// Sets the plot's y-tick value.
	pub fn with_y_tick(mut self, y_tick: impl ToPlotValue) -> Self {
		self.y_tick = Some(y_tick.to_plot_value());
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

	/// Adds a point to the plot at the supplied coordinates.
	pub fn point(&mut self, x_value: impl ToPlotValue, y_value: impl ToPlotValue) {
		self.points.push((x_value.to_plot_value(), y_value.to_plot_value()));
	}
}
