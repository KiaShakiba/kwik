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

use crate::plot::{Plot, auto_option};

/// A scatter plot.
#[derive(Default)]
pub struct ScatterPlot {
	title: Option<String>,

	x_label: Option<String>,
	y_label: Option<String>,

	x_min: Option<f64>,
	x_max: Option<f64>,

	y_min: Option<f64>,
	y_max: Option<f64>,

	x_tick: Option<f64>,
	y_tick: Option<f64>,

	points: Vec<(f64, f64)>,
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

	/// Adds a point to the plot at the supplied coordinates.
	pub fn point(&mut self, x_value: f64, y_value: f64) {
		self.points.push((x_value, y_value));
	}
}
