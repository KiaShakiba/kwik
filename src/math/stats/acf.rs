/*
 * Copyright (c) Kia Shakiba
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

use num_traits::AsPrimitive;
use thiserror::Error;

/// Calculates the streaming autocorrelation coefficient.
#[derive(Clone, Default)]
pub struct Acf {
	values: Vec<f64>,

	cached_mean: Option<f64>,
	cached_variance: Option<f64>,
}

#[derive(Debug, Error)]
pub enum AcfError {
	#[error("empty values")]
	EmptyValues,

	#[error("lag smaller than number of values")]
	InvalidLag,
}

impl Acf {
	/// Returns `true` if there are no observations.
	///
	/// # Examples
	/// ```
	/// use kwik::math::stats::Acf;
	///
	/// let acf = Acf::default();
	/// assert!(acf.is_empty());
	/// ```
	pub fn is_empty(&self) -> bool {
		self.values.is_empty()
	}

	/// Returns the number of observations.
	///
	/// # Examples
	/// ```
	/// use kwik::math::stats::Acf;
	///
	/// let mut acf = Acf::default();
	///
	/// acf.insert(1);
	/// acf.insert(2);
	/// acf.insert(3);
	///
	/// assert_eq!(acf.len(), 3);
	/// ```
	pub fn len(&self) -> usize {
		self.values.len()
	}

	/// Appends a value.
	///
	/// # Examples
	/// ```
	/// use kwik::math::stats::Acf;
	///
	/// let mut acf = Acf::default();
	///
	/// acf.insert(1);
	/// acf.insert(2);
	/// acf.insert(3);
	/// ```
	pub fn insert(&mut self, value: impl AsPrimitive<f64>) {
		self.values.push(value.as_());

		self.cached_mean = None;
		self.cached_variance = None;
	}

	/// Calculates the estimated autocorrelation coefficient.
	///
	/// # Examples
	/// ```
	/// use kwik::math::stats::Acf;
	///
	/// let mut acf = Acf::default();
	///
	/// acf.insert(1);
	/// acf.insert(2);
	/// acf.insert(3);
	/// acf.insert(1);
	/// acf.insert(2);
	/// acf.insert(3);
	///
	/// let coefficient = acf.coefficient(4).unwrap();
	/// assert_eq!(coefficient, 0.0);
	/// ```
	pub fn coefficient(
		&mut self,
		lag: impl AsPrimitive<usize>,
	) -> Result<f64, AcfError> {
		if self.values.is_empty() {
			return Err(AcfError::EmptyValues);
		}

		let lag = lag.as_();

		if lag >= self.values.len() {
			return Err(AcfError::InvalidLag);
		}

		let mean = self.mean()?;

		let sum = (0..(self.values.len() - lag))
			.map(|i| (self.values[i] - mean) * (self.values[i + lag] - mean))
			.sum::<f64>();

		Ok(sum / ((self.values.len() - lag) as f64 * self.variance(mean)?))
	}

	fn variance(
		&mut self,
		mean: impl AsPrimitive<f64>,
	) -> Result<f64, AcfError> {
		if let Some(variance) = self.cached_variance {
			return Ok(variance);
		};

		if self.values.is_empty() {
			return Err(AcfError::EmptyValues);
		}

		let mean = mean.as_();

		let sum = self
			.values
			.iter()
			.map(|value| (*value - mean).powf(2.0))
			.sum::<f64>();

		let variance = sum / self.values.len() as f64;
		self.cached_variance = Some(variance);

		Ok(variance)
	}

	fn mean(&mut self) -> Result<f64, AcfError> {
		if let Some(mean) = self.cached_mean {
			return Ok(mean);
		};

		if self.values.is_empty() {
			return Err(AcfError::EmptyValues);
		}

		let sum = self.values.iter().sum::<f64>();

		let mean = sum / self.values.len() as f64;
		self.cached_mean = Some(mean);

		Ok(mean)
	}
}

#[cfg(test)]
mod tests {
	use approx::assert_relative_eq;

	use crate::math::stats::acf::{Acf, AcfError};

	#[test]
	fn it_calculates_mean_correctly() {
		let mut acf = Acf::default();

		acf.insert(1);
		acf.insert(2);
		acf.insert(3);

		assert!(matches!(acf.mean(), Ok(2.0)));
	}

	#[test]
	fn it_returns_mean_error_for_empty_values() {
		let mut acf = Acf::default();
		assert!(matches!(acf.mean(), Err(AcfError::EmptyValues)));
	}

	#[test]
	fn it_calculates_variance_correctly() {
		let mut acf = Acf::default();

		acf.insert(1);
		acf.insert(2);
		acf.insert(3);
		acf.insert(4);

		assert!(matches!(acf.variance(2.5), Ok(1.25)));
	}

	#[test]
	fn it_returns_variance_error_for_empty_values() {
		let mut acf = Acf::default();
		assert!(matches!(acf.variance(0), Err(AcfError::EmptyValues)));
	}

	#[test]
	fn it_calculates_coefficient_correctly() {
		let mut acf = Acf::default();

		acf.insert(1);
		acf.insert(2);
		acf.insert(3);
		acf.insert(1);
		acf.insert(2);
		acf.insert(3);

		for (lag, expected) in [
			1.0, -0.3, -0.75, 1.0, 0.0, -1.5,
		]
		.into_iter()
		.enumerate()
		{
			let coefficient = acf.coefficient(lag).unwrap();
			assert_relative_eq!(coefficient, expected);
		}
	}

	#[test]
	fn it_returns_coefficient_error_for_empty_values() {
		let mut acf = Acf::default();
		assert!(matches!(acf.coefficient(0), Err(AcfError::EmptyValues)));
	}

	#[test]
	fn it_returns_coefficient_error_for_invalid_lag() {
		let mut acf = Acf::default();

		acf.insert(1);
		acf.insert(2);
		acf.insert(3);

		assert!(matches!(acf.coefficient(3), Err(AcfError::InvalidLag)));
	}
}
