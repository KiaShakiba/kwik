use std::{
	cmp,
	path::Path,
	io::{Error, ErrorKind},
};

use gnuplot::Figure as GnuplotFigure;
use crate::plot::Plot;

/// A figure which may hold one or more plots.
pub struct Figure {
	figure: GnuplotFigure,

	columns: usize,
	count: usize,

	plot_width_px: f32,
	plot_height_px: f32,
}

pub const DPI: f32 = 72.0;
pub const DEFAULT_WIDTH_PX: f32 = 323.0;
pub const DEFAULT_HEIGHT_PX: f32 = 190.0;

impl Figure {
	/// Constructs a new figure with the supplied maximum number
	/// of columns. The number of rows can grow, though the number
	/// of columns will be limited to the supplied amount.
	///
	/// # Examples
	/// ```
	/// use kwik::plot::Figure;
	///
	/// let figure = Figure::new(4); // creates a figure with a maximum of 4 columns
	/// ```
	pub fn new(columns: usize) -> Self {
		Figure {
			figure: GnuplotFigure::new(),

			columns,
			count: 0,

			plot_width_px: DEFAULT_WIDTH_PX,
			plot_height_px: DEFAULT_HEIGHT_PX,
		}
	}

	/// Sets the width (in pixels) of an individual plot in the figure.
	/// By default, this value is initially set the `DEFAULT_WIDTH_PX`.
	///
	/// # Examples
	/// ```
	/// use kwik::plot::Figure;
	///
	/// let mut figure = Figure::new(1);
	/// figure.set_plot_width(200.0);
	/// ```
	pub fn set_plot_width(&mut self, plot_width_px: f32) {
		self.plot_width_px = plot_width_px;
	}

	/// Sets the width (in pixels) of an individual plot in the figure.
	/// By default, this value is initially set the `DEFAULT_WIDTH_PX`.
	///
	/// # Examples
	/// ```
	/// use kwik::plot::Figure;
	///
	/// let figure = Figure::new(1)
	///     .with_plot_width(200.0);
	/// ```
	pub fn with_plot_width(mut self, plot_width_px: f32) -> Self {
		self.set_plot_width(plot_width_px);
		self
	}

	/// Sets the height (in pixels) of an individual plot in the figure.
	/// By default, this value is initially set the `DEFAULT_HEIGHT_PX`.
	///
	/// # Examples
	/// ```
	/// use kwik::plot::Figure;
	///
	/// let mut figure = Figure::new(1);
	/// figure.set_plot_height(200.0);
	/// ```
	pub fn set_plot_height(&mut self, plot_height_px: f32) {
		self.plot_height_px = plot_height_px;
	}

	/// Sets the height (in pixels) of an individual plot in the figure.
	/// By default, this value is initially set the `DEFAULT_HEIGHT_PX`.
	///
	/// # Examples
	/// ```
	/// use kwik::plot::Figure;
	///
	/// let figure = Figure::new(1)
	///     .with_plot_height(200.0);
	/// ```
	pub fn with_plot_height(mut self, plot_height_px: f32) -> Self {
		self.set_plot_height(plot_height_px);
		self
	}

	/// Checks if the figure is empty (i.e., contains no plots).
	///
	/// # Examples
	/// ```
	/// use kwik::plot::Figure;
	///
	/// let figure = Figure::new(1);
	/// assert!(figure.is_empty());
	/// ```
	pub fn is_empty(&self) -> bool {
		self.count == 0
	}

	/// Adds a plot to the figure, appending it to the end of the current plots.
	pub fn add(&mut self, mut plot: impl Plot) {
		if plot.is_empty() {
			return;
		}

		self.count += 1;

		self.figure.set_multiplot_layout(
			(self.count as f32 / self.columns as f32).ceil() as usize,
			*cmp::min(&self.count, &self.columns)
		);

		plot.configure(self.figure.axes2d());
	}

	/// Saves the figure to a file at the supplied path.
	///
	/// # Errors
	///
	/// This function will return an error if the figure could not be
	/// saved to the file at the supplied path.
	pub fn save<P>(&mut self, path: P) -> Result<(), Error>
	where
		P: AsRef<Path>,
	{
		if self.is_empty() {
			return Err(Error::new(
				ErrorKind::InvalidData,
				"Could not save figure with no plots"
			));
		}

		let columns = cmp::min(&self.count, &self.columns);
		let rows = (self.count as f32 / self.columns as f32).ceil() as u32;

		let plot_width_in = self.plot_width_px / DPI;
		let plot_height_in = self.plot_height_px / DPI;

		let width = *columns as f32 * plot_width_in;
		let height = rows as f32 * plot_height_in;

		match self.figure.save_to_pdf(path, width, height) {
			Ok(_) => Ok(()),

			Err(_) => Err(Error::new(
				ErrorKind::PermissionDenied,
				"Could not save figure"
			))
		}
	}
}
