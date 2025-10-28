/*
 * Copyright (c) Kia Shakiba
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

use std::{fmt::Display, slice};

use gnuplot::{
	AlignType,
	Axes2D,
	AxesCommon,
	BorderLocation2D,
	ColorType,
	Coordinate,
	DashType,
	LabelOption,
	LegendOption,
	PlotOption,
	TickOption,
	XAxis,
	YAxis,
};
use num_traits::AsPrimitive;

use crate::plot::{
	AxisFormat,
	COLORS,
	DEFAULT_FONT_FAMILY,
	DEFAULT_FONT_SIZE,
	LegendPosition,
	Plot,
	auto_option,
	init_scaler,
};

const LINE_STYLES: &[LineStyle] = &[
	LineStyle::Solid,
	LineStyle::Dash,
	LineStyle::DotDash,
	LineStyle::DotDotDash,
	LineStyle::Dot,
];

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

	x_format: Option<AxisFormat>,
	y_format: Option<AxisFormat>,
	y2_format: Option<AxisFormat>,

	x_log_base: Option<f64>,
	y_log_base: Option<f64>,
	y2_log_base: Option<f64>,

	legend_position: Option<LegendPosition>,

	y1_lines: Vec<Line>,
	y2_lines: Vec<Line>,

	vlines: Vec<f64>,
	hlines: Vec<f64>,

	points: Vec<Point>,
}

/// An individual line on a line plot.
#[derive(Clone)]
pub struct Line {
	label: Option<String>,
	width: f64,

	x_values: Vec<f64>,
	y_values: Vec<f64>,

	y2_axis: bool,

	maybe_color: Option<String>,
	maybe_style: Option<LineStyle>,
}

/// The style of a line on a line plot.
#[derive(Clone)]
pub enum LineStyle {
	Solid,
	Dash,
	DotDash,
	DotDotDash,
	Dot,
}

/// An individual point on a line plot.
#[derive(Clone)]
pub struct Point {
	x: f64,
	y: f64,

	symbol: char,
	size: f64,
}

impl Plot for LinePlot {
	fn is_empty(&self) -> bool {
		if self.y1_lines.is_empty() && self.y2_lines.is_empty() {
			// there are no lines in the plot
			return true;
		}

		// there are lines in the plot, though they all may be empty
		let y1_lines_empty = self.y1_lines.iter().all(|line| line.is_empty());
		let y2_lines_empty = self.y2_lines.iter().all(|line| line.is_empty());

		y1_lines_empty && y2_lines_empty
	}

	fn set_font_type<T>(&mut self, font_type: T)
	where
		T: AsRef<str>,
	{
		self.font_type = Some(font_type.as_ref().to_string());
	}

	fn with_font_type<T>(mut self, font_type: T) -> Self
	where
		T: AsRef<str>,
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
			self.font_type
				.as_deref()
				.unwrap_or(DEFAULT_FONT_FAMILY),
			self.font_size.unwrap_or(DEFAULT_FONT_SIZE),
		);

		let x_scaler = init_scaler(self.x_format, self.max_x_value());
		let y_scaler = init_scaler(self.y_format, self.max_y_value());
		let y2_scaler = init_scaler(self.y2_format, self.max_y2_value());

		axes.set_border(
			false,
			&[
				BorderLocation2D::Bottom,
				BorderLocation2D::Left,
			],
			&[],
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
			&[
				TickOption::Mirror(false),
				TickOption::Inward(false),
			],
			slice::from_ref(&font),
		)
		.set_y_ticks(
			Some((auto_option(self.y_tick, y_scaler.as_ref()), 0)),
			&[
				TickOption::Mirror(false),
				TickOption::Inward(false),
			],
			slice::from_ref(&font),
		)
		.set_grid_options(false, &[
			PlotOption::Color(ColorType::RGBString("#bbbbbb")),
			PlotOption::LineWidth(2.0),
			PlotOption::LineStyle(DashType::Dot),
		])
		.set_x_grid(true)
		.set_y_grid(true);

		if let Some(title) = &self.title {
			axes.set_title(title, slice::from_ref(&font));
		}

		if let Some(x_label) = &self.x_label {
			axes.set_x_label(
				&x_scaler.apply_unit(x_label),
				slice::from_ref(&font),
			);
		}

		if let Some(y_label) = &self.y_label {
			axes.set_y_label(
				&y_scaler.apply_unit(y_label),
				slice::from_ref(&font),
			);
		}

		if let Some(base) = self.x_log_base {
			axes.set_x_log(Some(base));
		}

		if let Some(base) = self.y_log_base {
			axes.set_y_log(Some(base));
		}

		if let Some(legend_position) = &self.legend_position {
			let (x, halign) = match legend_position {
				LegendPosition::TopRight | LegendPosition::BottomRight => {
					(Coordinate::Graph(1.0), AlignType::AlignRight)
				},

				LegendPosition::TopLeft | LegendPosition::BottomLeft => {
					(Coordinate::Graph(0.02), AlignType::AlignLeft)
				},
			};

			let (y, valign) = match legend_position {
				LegendPosition::TopRight | LegendPosition::TopLeft => {
					(Coordinate::Graph(1.0), AlignType::AlignTop)
				},

				LegendPosition::BottomRight | LegendPosition::BottomLeft => {
					(Coordinate::Graph(0.0), AlignType::AlignBottom)
				},
			};

			let placement = LegendOption::Placement(halign, valign);
			axes.set_legend(x, y, &[placement], &[]);
		}

		if !self.y2_lines.is_empty() {
			axes.set_y2_range(
				auto_option(self.y2_min, y2_scaler.as_ref()),
				auto_option(self.y2_max, y2_scaler.as_ref()),
			);

			axes.set_y2_ticks(
				Some((auto_option(self.y2_tick, y2_scaler.as_ref()), 0)),
				&[
					TickOption::Mirror(false),
					TickOption::Inward(false),
				],
				slice::from_ref(&font),
			);

			if let Some(y2_label) = &self.y2_label {
				axes.set_y2_label(&y2_scaler.apply_unit(y2_label), &[font]);
			}

			if let Some(base) = self.y2_log_base {
				axes.set_y2_log(Some(base));
			}
		}

		for (index, line) in self.y1_lines.iter().enumerate() {
			let color = line
				.maybe_color
				.as_deref()
				.unwrap_or(COLORS[index % COLORS.len()]);

			let style = line
				.maybe_style
				.as_ref()
				.unwrap_or(&LINE_STYLES[index % LINE_STYLES.len()]);

			let mut line_config: Vec<PlotOption<&str>> = vec![
				PlotOption::LineWidth(line.width),
				PlotOption::Color(color.into()),
				PlotOption::LineStyle(style.into()),
			];

			if let Some(label) = &line.label {
				line_config.push(PlotOption::Caption(label));
			}

			let x_values = line
				.x_values
				.iter()
				.map(|value| x_scaler.scale(*value));

			let y_values = line
				.y_values
				.iter()
				.map(|value| y_scaler.scale(*value));

			axes.lines(x_values, y_values, &line_config);
		}

		for (index, line) in self.y2_lines.iter().enumerate() {
			let global_index = self.y1_lines.len() + index;

			let color = line
				.maybe_color
				.as_deref()
				.unwrap_or(COLORS[global_index % COLORS.len()]);

			let style = line
				.maybe_style
				.as_ref()
				.unwrap_or(&LINE_STYLES[global_index % LINE_STYLES.len()]);

			let mut line_config: Vec<PlotOption<&str>> = vec![
				PlotOption::LineWidth(line.width),
				PlotOption::Color(color.into()),
				PlotOption::LineStyle(style.into()),
				PlotOption::Axes(XAxis::X1, YAxis::Y2),
			];

			if let Some(label) = &line.label {
				line_config.push(PlotOption::Caption(label));
			}

			let x_values = line
				.x_values
				.iter()
				.map(|value| x_scaler.scale(*value));

			let y_values = line
				.y_values
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
				PlotOption::LineWidth(2.0),
				PlotOption::Color(ColorType::RGBString("blue")),
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
				PlotOption::LineWidth(2.0),
				PlotOption::Color(ColorType::RGBString("blue")),
			]);
		}

		for point in &self.points {
			axes.points(
				[x_scaler.scale(point.x)],
				[y_scaler.scale(point.y)],
				&[
					PlotOption::PointSymbol(point.symbol),
					PlotOption::PointSize(point.size),
					PlotOption::Color(ColorType::RGBString("blue")),
				],
			);
		}
	}
}

impl LinePlot {
	/// Sets the plot's y2-axis label.
	pub fn set_y2_label<T>(&mut self, label: T)
	where
		T: AsRef<str>,
	{
		self.y2_label = Some(label.as_ref().to_string());
	}

	/// Sets the plot's y2-axis label.
	pub fn with_y2_label<T>(mut self, label: T) -> Self
	where
		T: AsRef<str>,
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

	/// Sets the plot's y2-format type.
	pub fn set_y2_format(&mut self, format_type: AxisFormat) {
		if let AxisFormat::Log(base) = format_type {
			self.y2_log_base = Some(base);
			return;
		}

		self.y2_format = Some(format_type);
	}

	/// Sets the plot's y2-format type.
	pub fn with_y2_format(mut self, format_type: AxisFormat) -> Self {
		self.set_y2_format(format_type);
		self
	}

	/// Sets the plot's legend position.
	pub fn set_legend_position(&mut self, position: LegendPosition) {
		self.legend_position = Some(position);
	}

	/// Sets the plot's legend position.
	pub fn with_legend_position(mut self, position: LegendPosition) -> Self {
		self.set_legend_position(position);
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

	/// Adds a point to the plot.
	pub fn point(&mut self, point: Point) {
		self.points.push(point);
	}

	fn min_x_value(&self) -> f64 {
		let mut min = self.x_min;

		for line in &self.y1_lines {
			let line_min = line
				.x_values
				.iter()
				.min_by(|a, b| a.total_cmp(b))
				.copied()
				.unwrap_or(0.0);

			if min.is_none() || min.is_some_and(|value| value > line_min) {
				min = Some(line_min);
			}
		}

		for line in &self.y2_lines {
			let line_min = line
				.x_values
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
			let line_max = line
				.x_values
				.iter()
				.max_by(|a, b| a.total_cmp(b))
				.copied()
				.unwrap_or(0.0);

			if max.is_none() || max.is_some_and(|value| value < line_max) {
				max = Some(line_max);
			}
		}

		for line in &self.y2_lines {
			let line_max = line
				.x_values
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
			let line_min = line
				.y_values
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
			let line_max = line
				.y_values
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
			let line_max = line
				.y_values
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
}

impl Line {
	/// Returns `true` if the line is empty.
	pub fn is_empty(&self) -> bool {
		self.x_values.is_empty()
	}

	/// Sets the line's label.
	pub fn set_label<T>(&mut self, label: T)
	where
		T: AsRef<str>,
	{
		self.label = Some(label.as_ref().to_string());
	}

	/// Sets the line's label.
	pub fn with_label<T>(mut self, label: T) -> Self
	where
		T: AsRef<str>,
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

	/// Sets the line's color.
	pub fn set_color<T>(&mut self, color: T)
	where
		T: AsRef<str>,
	{
		self.maybe_color = Some(color.as_ref().to_string());
	}

	/// Sets the line's color.
	pub fn with_color<T>(mut self, color: T) -> Self
	where
		T: AsRef<str>,
	{
		self.set_color(color);
		self
	}

	/// Sets the line's style.
	pub fn set_style(&mut self, style: LineStyle) {
		self.maybe_style = Some(style);
	}

	/// Sets the line's style.
	pub fn with_style(mut self, style: LineStyle) -> Self {
		self.set_style(style);
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

			maybe_color: None,
			maybe_style: None,
		}
	}
}

impl From<LineStyle> for DashType {
	fn from(style: LineStyle) -> Self {
		(&style).into()
	}
}

impl From<&LineStyle> for DashType {
	fn from(style: &LineStyle) -> Self {
		match style {
			LineStyle::Solid => DashType::Solid,
			LineStyle::Dash => DashType::Dash,
			LineStyle::DotDash => DashType::DotDash,
			LineStyle::DotDotDash => DashType::DotDotDash,
			LineStyle::Dot => DashType::Dot,
		}
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
}
