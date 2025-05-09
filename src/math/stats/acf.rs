/*
 * Copyright (c) Kia Shakiba
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

use thiserror::Error;
use num_traits::AsPrimitive;

/// Calculates the streaming autocorrelation coefficient.
#[derive(Clone, Default)]
pub struct Acf {
	values: Vec<f64>,
}

#[derive(Debug, Error)]
pub enum AcfError {
	#[error("lag smaller than number of values")]
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
	pub fn coefficient(&self, lag: impl AsPrimitive<usize>) -> Result<f64, AcfError> {
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

		Ok(sum / (self.values.len() as f64 * self.variance(mean)?))
	}

	fn variance(&self, mean: impl AsPrimitive<f64>) -> Result<f64, AcfError> {
		if self.values.is_empty() {
			return Err(AcfError::EmptyValues);
		}

		let mean = mean.as_();

		let sum = self.values
			.iter()
			.map(|value| (*value - mean).powf(2.0))
			.sum::<f64>();

		Ok(sum / self.values.len() as f64)
	}

	fn mean(&self) -> Result<f64, AcfError> {
		if self.values.is_empty() {
			return Err(AcfError::EmptyValues);
		}

		let sum = self.values
			.iter()
			.sum::<f64>();

		Ok(sum / self.values.len() as f64)
	}
}

#[cfg(test)]
mod tests {
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
		let acf = Acf::default();
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
		let acf = Acf::default();
		assert!(matches!(acf.variance(0), Err(AcfError::EmptyValues)));
	}

	#[test]
	fn it_calculates_coefficient_correcntly() {
		let mut acf = Acf::default();

		acf.insert(1);
		acf.insert(2);
		acf.insert(3);
		acf.insert(1);
		acf.insert(2);
		acf.insert(3);

		for (lag, expected) in [1.0, -0.25, -0.5, 0.5, 0.0, -0.25].into_iter().enumerate() {
			let coefficient = acf.coefficient(lag).unwrap();
			assert_eq!(coefficient, expected);
		}
	}

	#[test]
	fn it_returns_coefficient_error_for_empty_values() {
		let acf = Acf::default();
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
