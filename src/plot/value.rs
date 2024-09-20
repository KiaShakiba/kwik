/*
 * Copyright (c) Kia Shakiba
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

/// A value displayable on a plot.
pub type PlotValue = f64;

/// Implementing this trait allows for the use of the implemented type as a value on a plot.
pub trait ToPlotValue {
	/// Returns the `PlotValue` of the type.
	///
	/// # Examples
	/// ```
	/// struct MyStruct {
	///     data: u64,
	/// }
	///
	/// impl ToPlotValue for MyStruct {
	///     fn to_plot_value(&self) -> PlotValue {
	///         self.data as PlotValue
	///     }
	/// }
	/// ```
	fn to_plot_value(&self) -> PlotValue;
}

impl ToPlotValue for u8 {
	fn to_plot_value(&self) -> PlotValue {
		*self as PlotValue
	}
}

impl ToPlotValue for i8 {
	fn to_plot_value(&self) -> PlotValue {
		*self as PlotValue
	}
}

impl ToPlotValue for u16 {
	fn to_plot_value(&self) -> PlotValue {
		*self as PlotValue
	}
}

impl ToPlotValue for i16 {
	fn to_plot_value(&self) -> PlotValue {
		*self as PlotValue
	}
}

impl ToPlotValue for u32 {
	fn to_plot_value(&self) -> PlotValue {
		*self as PlotValue
	}
}

impl ToPlotValue for i32 {
	fn to_plot_value(&self) -> PlotValue {
		*self as PlotValue
	}
}

impl ToPlotValue for u64 {
	fn to_plot_value(&self) -> PlotValue {
		*self as PlotValue
	}
}

impl ToPlotValue for i64 {
	fn to_plot_value(&self) -> PlotValue {
		*self as PlotValue
	}
}

impl ToPlotValue for u128 {
	fn to_plot_value(&self) -> PlotValue {
		*self as PlotValue
	}
}

impl ToPlotValue for i128 {
	fn to_plot_value(&self) -> PlotValue {
		*self as PlotValue
	}
}

impl ToPlotValue for usize {
	fn to_plot_value(&self) -> PlotValue {
		*self as PlotValue
	}
}

impl ToPlotValue for isize {
	fn to_plot_value(&self) -> PlotValue {
		*self as PlotValue
	}
}

impl ToPlotValue for f32 {
	fn to_plot_value(&self) -> PlotValue {
		*self as PlotValue
	}
}

impl ToPlotValue for f64 {
	fn to_plot_value(&self) -> PlotValue {
		*self
	}
}
