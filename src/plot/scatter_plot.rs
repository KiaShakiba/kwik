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
	ColorType,
	DashType,
	BorderLocation2D,
	TickOption,
	LabelOption,
};

use crate::plot::{
	Plot,
	AxisFormat,
	init_scaler,
	auto_option,
	COLORS,
};

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

	x_format: Option<AxisFormat>,
	y_format: Option<AxisFormat>,

	x_log_base: Option<f64>,
	y_log_base: Option<f64>,

	points: Vec<Point>,
}

/// An individual point on a scatter plot.
#[derive(Clone)]
pub struct Point {
	x: f64,
	y: f64,

	symbol: char,
	size: f64,
	color: ColorType,
}

impl Plot for ScatterPlot {
	fn is_empty(&self) -> bool {
		self.points.is_empty()
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

		let x_scaler = init_scaler(self.x_format, self.max_x_value());
		let y_scaler = init_scaler(self.y_format, self.max_y_value());

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
				&[font.clone()],
			)
			.set_y_ticks(
				Some((auto_option(self.y_tick, y_scaler.as_ref()), 0)),
				&[TickOption::Mirror(false), TickOption::Inward(false)],
				&[font.clone()],
			)
			.set_grid_options(false, &[
				PlotOption::Color(ColorType::RGBString("#bbbbbb")),
				PlotOption::LineWidth(2.0),
				PlotOption::LineStyle(DashType::Dot),
			])
			.set_x_grid(true)
			.set_y_grid(true);

		if let Some(title) = &self.title {
			axes.set_title(title, &[font.clone()]);
		}

		if let Some(x_label) = &self.x_label {
			axes.set_x_label(&x_scaler.apply_unit(x_label), &[font.clone()]);
		}

		if let Some(y_label) = &self.y_label {
			axes.set_y_label(&y_scaler.apply_unit(y_label), &[font]);
		}

		if let Some(base) = self.x_log_base {
			axes.set_x_log(Some(base));
		}

		if let Some(base) = self.y_log_base {
			axes.set_y_log(Some(base));
		}

		for point in &self.points {
			axes.points(
				[x_scaler.scale(point.x)],
				[y_scaler.scale(point.y)],
				&[
					PlotOption::PointSymbol(point.symbol),
					PlotOption::PointSize(point.size),
					PlotOption::Color(point.color.to_ref()),
				],
			);
		}
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

	/// Sets the plot's x-format type.
	pub fn set_x_format(&mut self, format_type: AxisFormat) {
		if let AxisFormat::Log(base) = format_type {
			self.x_log_base = Some(base);
			return;
		}

		self.x_format = Some(format_type);
	}

	/// Sets the plot's x-format type.
	pub fn with_x_format(mut self, format_type: AxisFormat) -> Self {
		self.set_x_format(format_type);
		self
	}

	/// Sets the plot's y-format type.
	pub fn set_y_format(&mut self, format_type: AxisFormat) {
		if let AxisFormat::Log(base) = format_type {
			self.y_log_base = Some(base);
			return;
		}

		self.y_format = Some(format_type);
	}

	/// Sets the plot's y-format type.
	pub fn with_y_format(mut self, format_type: AxisFormat) -> Self {
		self.set_y_format(format_type);
		self
	}

	/// Adds a point to the plot.
	pub fn point(&mut self, point: Point) {
		self.points.push(point);
	}

	fn max_x_value(&self) -> f64 {
		let mut max = self.x_max;

		for point in &self.points {
			if max.is_none_or(|value| value < point.x) {
				max = Some(point.x);
			}
		}

		max.unwrap_or(0.0)
	}

	fn max_y_value(&self) -> f64 {
		let mut max = self.y_max;

		for point in &self.points {
			if max.is_none_or(|value| value < point.y) {
				max = Some(point.y);
			}
		}

		max.unwrap_or(0.0)
	}
}

impl Point {
	/// Creates a new point with the supplied x and y values.
	pub fn new(x: impl AsPrimitive<f64>, y: impl AsPrimitive<f64>) -> Self {
		Point {
			x: x.as_(),
			y: y.as_(),

			symbol: 'o',
			size: 1.0,
			color: COLORS[0].into(),
		}
	}

	/// Sets the point's symbol.
	pub fn set_symbol(&mut self, symbol: char) {
		self.symbol = symbol;
	}

	/// Sets the point's symbol.
	pub fn with_symbol(mut self, symbol: char) -> Self {
		self.set_symbol(symbol);
		self
	}

	/// Sets the point's size.
	pub fn set_size(&mut self, size: impl AsPrimitive<f64>) {
		self.size = size.as_();
	}

	/// Sets the point's size.
	pub fn with_size(mut self, size: impl AsPrimitive<f64>) -> Self {
		self.set_size(size);
		self
	}

	/// Sets the point's color.
	pub fn set_color<T>(&mut self, color: T)
	where
		T: Display,
	{
		self.color = color.to_string().into();
	}

	/// Sets the point's color.
	pub fn with_color<T>(mut self, color: T) -> Self
	where
		T: Display,
	{
		self.set_color(color);
		self
	}
}
