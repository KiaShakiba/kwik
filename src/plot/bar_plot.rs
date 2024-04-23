/*
 * Copyright (c) Kia Shakiba
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

use gnuplot::{
	Axes2D,
	AxesCommon,
	TickOption,
	AutoOption,
	Major,
	Fix,
	LabelOption,
	LineStyle,
	Color,
	LineWidth,
	DashType,
	Caption,
};

use crate::{
	math,
	plot::{Plot, auto_option, COLORS},
};

/// A bar plot.
#[derive(Default)]
pub struct BarPlot {
	title: Option<String>,

	x_label: Option<String>,
	y_label: Option<String>,

	y_max: Option<f64>,

	bar_groups: Vec<BarGroup>,

	format_y_log: bool,
	format_y_memory: bool,
}

/// A group of bars on the bar plot.
#[derive(Default)]
pub struct BarGroup {
	label: Option<String>,
	bars: Vec<Bar>,
}

/// An individial bar on the bar plot.
pub struct Bar {
	label: Option<String>,
	value: f64,
}

impl Plot for BarPlot {
	fn is_empty(&self) -> bool {
		self.bar_groups.is_empty()
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
		let labels = self.bar_groups
			.iter()
			.map(|bar_group| bar_group.label.as_deref().unwrap_or("").into())
			.collect::<Vec<String>>();

		let y_tick_format = match self.format_y_memory {
			true => "%.1s %cB",
			false => "%.0f",
		};

		axes
			.set_x_range(
				AutoOption::Fix(0.0),
				AutoOption::Fix(self.bar_groups.len() as f64 + 1.0)
			)
			.set_y_range(
				AutoOption::Fix(0.0),
				auto_option(self.y_max),
			)
			.set_x_ticks_custom(
				labels
					.iter()
					.enumerate()
					.map(|(index, label)| {
						Major(index as f64 + 1.0, Fix(label))
					}),
				&[
					TickOption::Mirror(false),
					TickOption::Inward(false),
				],
				&[
					LabelOption::Rotate(-45.0),
				]
			)
			.set_y_ticks(
				Some((AutoOption::Auto, 0)),
				&[
					TickOption::Mirror(false),
					TickOption::Inward(false),
					TickOption::Format(y_tick_format),
				],
				&[]
			)
			.set_grid_options(false, &[
				Color("#bbbbbb"),
				LineWidth(2.0),
				LineStyle(DashType::Dot),
			])
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

		if self.format_y_log {
			axes.set_y_log(Some(10.0));
		}

		if self.bar_groups.is_empty() {
			return;
		}

		for bar_index in 0..self.bar_groups[0].bars.len() {
			let x_values = self.bar_groups
				.iter()
				.enumerate()
				.map(|(bar_group_index, bar_group)| {
					bar_group.bar_x_value(
						bar_group_index,
						bar_group.bars.len(),
						bar_index,
					)
				});

			let y_values = self.bar_groups
				.iter()
				.map(|bar_group| bar_group.bars[bar_index].value);

			let widths = self.bar_groups
				.iter()
				.map(|bar_group| bar_group.bar_width());

			let mut bar_config = vec![
				Color(COLORS[bar_index % COLORS.len()]),
				LineWidth(1.25),
			];

			if let Some(label) = &self.bar_groups[0].bars[bar_index].label {
				bar_config.push(Caption(label));
			}

			axes.boxes_set_width(
				x_values,
				y_values,
				widths,
				&bar_config,
			);
		}
	}
}

impl BarPlot {
	/// Sets the plot's maximum y-value.
	pub fn set_y_max(&mut self, y_max: f64) {
		self.y_max = Some(y_max);
	}

	/// Sets the plot's maximum y-value.
	pub fn with_y_max(mut self, y_max: f64) -> Self {
		self.y_max = Some(y_max);
		self
	}

	/// Adds a bar group to the plot.
	pub fn add(&mut self, bar_group: BarGroup) {
		self.bar_groups.push(bar_group);
	}
}

impl BarGroup {
	/// Sets the bar group's label.
	pub fn set_label(&mut self, label: &str) {
		self.label = Some(label.into());
	}

	/// Sets the bar group's label.
	pub fn with_label(mut self, label: &str) -> Self {
		self.set_label(label);
		self
	}

	/// Adds a bar into the bar group.
	pub fn push(&mut self, bar: Bar) {
		self.bars.push(bar);
	}

	fn bar_width(&self) -> f64 {
		math::min(&[1.0 / self.bars.len() as f64, 0.15])
	}

	fn bar_x_value(
		&self,
		bar_group_index: usize,
		num_bars: usize,
		bar_index: usize,
	) -> f64 {
		let center = bar_group_index as f64 + 1.0;
		let offset = num_bars as f64 / 2.0 - 0.5;
		let width = self.bar_width();

		center + (bar_index as f64 - offset) * width
	}
}

impl Bar {
	/// Create a new bar with the supplied value.
	pub fn new(value: f64) -> Self {
		Bar {
			label: None,
			value,
		}
	}

	/// Sets the bar's label.
	pub fn set_label(&mut self, label: &str) {
		self.label = Some(label.into());
	}

	/// Sets the bar's label.
	pub fn with_label(mut self, label: &str) -> Self {
		self.set_label(label);
		self
	}
}
