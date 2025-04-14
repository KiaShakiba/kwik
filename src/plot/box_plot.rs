/*
 * Copyright (c) Kia Shakiba
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

use std::{
	fmt::Display,
	collections::HashMap,
};

use num_traits::AsPrimitive;

use gnuplot::{
	Axes2D,
	AxesCommon,
	AutoOption,
	Major,
	Fix,
	TickOption,
	LabelOption,
	PlotOption,
	LineStyle,
	Color,
	LineWidth,
	DashType,
};

use indexmap::IndexMap;
use statrs::statistics::{Data, Min, Max, Distribution, OrderStatistics};

use crate::plot::{
	Plot,
	Scaler,
	NoScaler,
	SizeScaler,
	TimeScaler,
	auto_option,
};

/// A box plot.
#[derive(Default, Clone)]
pub struct BoxPlot {
	font_type: Option<String>,
	font_size: Option<f64>,

	title: Option<String>,

	x_label: Option<String>,
	y_label: Option<String>,

	y_min: Option<f64>,
	y_max: Option<f64>,

	y_tick: Option<f64>,

	format_y_log: bool,
	format_y_memory: bool,
	format_y_time: bool,

	map: IndexMap<String, Vec<f64>>,

	colors: HashMap<String, String>,
}

struct Stats {
	min: f64,
	max: f64,

	mean: f64,
	median: f64,

	q1: f64,
	q3: f64,
}

impl Plot for BoxPlot {
	fn is_empty(&self) -> bool {
		self.map.is_empty()
	}

	fn set_font_type<T>(&mut self, font_type: T)
	where
		T: Display,
	{
		self.font_type = Some(font_type.to_string());
	}

	fn with_font_type<T>(mut self, font_type: T) -> Self
	where
		T: Display,
	{
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

		let labels = self.map
			.keys()
			.map(|label| label.into())
			.collect::<Vec<String>>();

		let y_scaler = self.y_scaler();

		axes
			.set_x_range(
				AutoOption::Fix(0.0),
				AutoOption::Fix(self.map.len() as f64 + 1.0)
			)
			.set_y_range(
				auto_option(self.y_min, y_scaler.as_ref()),
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
				Some((auto_option(self.y_tick, y_scaler.as_ref()), 0)),
				&[TickOption::Mirror(false), TickOption::Inward(false)],
				&[font]
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

		for (index, label) in labels.iter().enumerate() {
			let x_value = index as f64 + 1.0;
			let stats = self.get_stats(label);

			let color = self.colors
					.get(label)
					.map(|color| color.as_str())
					.unwrap_or("red");

			axes
				.box_and_whisker_set_width(
					[x_value],
					[y_scaler.scale(stats.q1())],
					[y_scaler.scale(stats.min())],
					[y_scaler.scale(stats.max())],
					[y_scaler.scale(stats.q3())],
					[0.25],
					&[
						PlotOption::Color("white"),
						PlotOption::BorderColor(color),
						PlotOption::WhiskerBars(0.5),
						PlotOption::LineWidth(1.25),
					]
				)
				.points(
					[x_value],
					[y_scaler.scale(stats.mean())],
					&[
						PlotOption::Color("blue"),
						PlotOption::PointSymbol('x'),
						PlotOption::PointSize(0.75),
					]
				)
				.points(
					[x_value],
					[y_scaler.scale(stats.median())],
					&[
						PlotOption::Color("blue"),
						PlotOption::PointSymbol('+'),
						PlotOption::PointSize(0.75),
					]
				);
		}
	}
}

impl BoxPlot {
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

	/// Sets the plot's y-tick value.
	pub fn set_y_tick(&mut self, y_tick: impl AsPrimitive<f64>) {
		self.y_tick = Some(y_tick.as_());
	}

	/// Sets the plot's y-tick value.
	pub fn with_y_tick(mut self, y_tick: impl AsPrimitive<f64>) -> Self {
		self.set_y_tick(y_tick);
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

	/// Sets an individual box's color.
	pub fn set_color<T1, T2>(&mut self, label: T1, color: T2)
	where
		T1: Display,
		T2: Display,
	{
		self.colors.insert(label.to_string(), color.to_string());
	}

	/// Sets an individual box's color.
	pub fn with_color<T1, T2>(mut self, label: T1, color: T1) -> Self
	where
		T1: Display,
		T2: Display,
	{
		self.set_color(label, color);
		self
	}

	/// Adds a data point to a box if it exists. Otherwise, creates a new
	/// box with the supplied label.
	pub fn add<T>(&mut self, label: T, value: impl AsPrimitive<f64>)
	where
		T: Display,
	{
		self.map
			.entry(label.to_string())
			.and_modify(|values| values.push(value.as_()))
			.or_insert(vec![value.as_()]);
	}

	fn get_stats(&mut self, label: &str) -> Stats {
		let values = self.map.get_mut(label)
			.expect("Could not get stats");

		Stats::new(values)
	}

	fn max_y_value(&self) -> f64 {
		let mut max = self.y_max;

		for (_, values) in &self.map {
			for y_value in values {
				if max.is_none_or(|value| value < *y_value) {
					max = Some(*y_value);
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

impl Stats {
	fn new(values: &mut Vec<f64>) -> Self {
		let mut data = Data::new(values);

		Stats {
			min: data.min(),
			max: data.max(),

			mean: data.mean().expect("Could not calculate mean of data."),
			median: data.median(),

			q1: data.lower_quartile(),
			q3: data.upper_quartile(),
		}
	}

	fn min(&self) -> f64 { self.min }
	fn max(&self) -> f64 { self.max }

	fn mean(&self) -> f64 { self.mean }
	fn median(&self) -> f64 { self.median }

	fn q1(&self) -> f64 { self.q1 }
	fn q3(&self) -> f64 { self.q3 }
}
