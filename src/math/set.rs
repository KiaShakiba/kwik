/*
 * Copyright (c) Kia Shakiba
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

pub trait Subset {
	/// Returns true if `self` is an improper subset of `other`.
	///
	/// # Examples
	/// ```
	/// use kwik::math::set::Subset;
	///
	/// let a = [1, 2, 3];
	/// let b = [1, 2, 3, 4];
	///
	/// assert!(a.iter().is_subset(b.iter()));
	/// ```
	fn is_subset(&mut self, other: Self) -> bool;
}

pub trait Superset {
	/// Returns true if `self` is an improper superset of `other`.
	///
	/// # Examples
	/// ```
	/// use kwik::math::set::Superset;
	///
	/// let a = [1, 2, 3, 4];
	/// let b = [1, 2, 3];
	///
	/// assert!(a.iter().is_superset(b.iter()));
	/// ```
	fn is_superset(&mut self, other: Self) -> bool;
}

pub trait Multiset {
	/// Returns true if `self` is a multiset (i.e. it contains duplicate
	/// values).
	///
	/// # Examples
	/// ```
	/// use kwik::math::set::Multiset;
	///
	/// let a = [1, 2, 3, 2];
	/// let b = [1, 2, 3, 4];
	///
	/// assert!(a.iter().is_multiset());
	/// assert!(!b.iter().is_multiset());
	/// ```
	fn is_multiset(&mut self) -> bool;
}

impl<I, T> Subset for I
where
	I: Iterator<Item = T> + Clone,
	T: PartialEq,
{
	fn is_subset(&mut self, other: Self) -> bool {
		let other_clone = other.clone();

		!self.any(|value| !other_clone.clone().any(|other| value.eq(&other)))
	}
}

impl<I, T> Superset for I
where
	I: Iterator<Item = T> + Clone,
	T: PartialEq,
{
	fn is_superset(&mut self, mut other: Self) -> bool {
		let self_clone = self.clone();

		!other.any(|value| !self_clone.clone().any(|other| value.eq(&other)))
	}
}

impl<I, T> Multiset for I
where
	I: Iterator<Item = T> + Clone,
	T: PartialEq,
{
	fn is_multiset(&mut self) -> bool {
		let iter_clone = self.clone();

		self.enumerate().any(|(index, value)| {
			iter_clone
				.clone()
				.skip(index + 1)
				.any(|other| value.eq(&other))
		})
	}
}

#[cfg(test)]
mod tests {
	use crate::math::set::{Multiset, Subset, Superset};

	#[test]
	fn it_identifies_subsets() {
		let a = [1, 2, 3];
		let b = [1, 2, 3, 4];
		let c = [1, 2, 3];
		let d = [1, 3, 4, 5];

		assert!(a.iter().is_subset(b.iter()));
		assert!(a.iter().is_subset(c.iter()));
		assert!(!a.iter().is_subset(d.iter()));
	}

	#[test]
	fn it_identifies_supersets() {
		let a = [1, 2, 3, 4];
		let b = [1, 2, 3];
		let c = [1, 2, 3, 4];
		let d = [1, 3, 5];

		assert!(a.iter().is_superset(b.iter()));
		assert!(a.iter().is_superset(c.iter()));
		assert!(!a.iter().is_superset(d.iter()));
	}

	#[test]
	fn it_identifies_multisets() {
		let a = [1, 2, 3, 2];
		let b = [1, 2, 3, 4];

		assert!(a.iter().is_multiset());
		assert!(!b.iter().is_multiset());
	}
}
