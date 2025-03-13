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
	plot::{
		Plot,
		Scaler,
		NoScaler,
		SizeScaler,
		TimeScaler,
		auto_option,
		COLORS,
	},
};

/// A bar plot.
#[derive(Default, Clone)]
pub struct BarPlot {
	font_type: Option<String>,
	font_size: Option<f64>,

	title: Option<String>,

	x_label: Option<String>,
	y_label: Option<String>,

	y_max: Option<f64>,

	format_y_log: bool,
	format_y_memory: bool,
	format_y_time: bool,

	bar_groups: Vec<BarGroup>,
}

/// A group of bars on the bar plot.
#[derive(Default, Clone)]
pub struct BarGroup {
	label: Option<String>,
	bars: Vec<Bar>,
}

/// An individial bar on the bar plot.
#[derive(Clone)]
pub struct Bar {
	label: Option<String>,
	value: f64,
}

impl Plot for BarPlot {
	fn is_empty(&self) -> bool {
		self.bar_groups.is_empty()
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

		let labels = self.bar_groups
			.iter()
			.map(|bar_group| bar_group.label.as_deref().unwrap_or("").into())
			.collect::<Vec<String>>();

		let y_scaler = self.y_scaler();

		axes
			.set_x_range(
				AutoOption::Fix(0.0),
				AutoOption::Fix(self.bar_groups.len() as f64 + 1.0)
			)
			.set_y_range(
				AutoOption::Fix(0.0),
				auto_option(self.y_max, y_scaler.as_ref()),
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
					font,
					LabelOption::Rotate(-45.0),
				]
			)
			.set_y_ticks(
				Some((AutoOption::Auto, 0)),
				&[TickOption::Mirror(false), TickOption::Inward(false)],
				&[font],
			)
			.set_grid_options(false, &[
				Color("#bbbbbb"),
				LineWidth(2.0),
				LineStyle(DashType::Dot),
			])
			.set_y_grid(true);

		if let Some(title) = &self.title {
			axes.set_title(title, &[font]);
		}

		if let Some(x_label) = &self.x_label {
			axes.set_x_label(x_label, &[font]);
		}

		if let Some(y_label) = &self.y_label {
			axes.set_y_label(&y_scaler.apply_unit(y_label), &[font]);
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
				.map(|bar_group| y_scaler.scale(bar_group.bars[bar_index].value));

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
	pub fn set_y_max(&mut self, y_max: impl AsPrimitive<f64>) {
		self.y_max = Some(y_max.as_());
	}

	/// Sets the plot's maximum y-value.
	pub fn with_y_max(mut self, y_max: impl AsPrimitive<f64>) -> Self {
		self.set_y_max(y_max);
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

	/// Enables or disables memory formatting in the y-axis.
	pub fn set_format_y_memory(&mut self, value: bool) {
		self.format_y_memory = value;
	}

	/// Enables or disables memory formatting in the y-axis.
	pub fn with_format_y_memory(mut self, value: bool) -> Self {
		self.set_format_y_memory(value);
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

	/// Adds a bar group to the plot.
	pub fn add(&mut self, bar_group: BarGroup) {
		self.bar_groups.push(bar_group);
	}

	fn max_y_value(&self) -> f64 {
		let mut max = self.y_max;

		for bar_group in &self.bar_groups {
			for bar in &bar_group.bars {
				if max.is_none_or(|value| value < bar.value) {
					max = Some(bar.value);
				}
			}
		}

		max.unwrap_or(0.0)
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
}

impl BarGroup {
	/// Sets the bar group's label.
	pub fn set_label<T>(&mut self, label: T)
	where
		T: Display,
	{
		self.label = Some(label.to_string());
	}

	/// Sets the bar group's label.
	pub fn with_label<T>(mut self, label: T) -> Self
	where
		T: Display,
	{
		self.set_label(label);
		self
	}

	/// Adds a bar into the bar group.
	pub fn push(&mut self, bar: Bar) {
		self.bars.push(bar);
	}

	fn bar_width(&self) -> f64 {
		*math::min(&[1.0 / self.bars.len() as f64, 0.15]).unwrap()
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
	pub fn new(value: impl AsPrimitive<f64>) -> Self {
		Bar {
			label: None,
			value: value.as_(),
		}
	}

	/// Sets the bar's label.
	pub fn set_label<T>(&mut self, label: T)
	where
		T: Display,
	{
		self.label = Some(label.to_string());
	}

	/// Sets the bar's label.
	pub fn with_label<T>(mut self, label: T) -> Self
	where
		T: Display,
	{
		self.set_label(label);
		self
	}
}
