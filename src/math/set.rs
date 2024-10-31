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
	/// let a = &[1, 2, 3];
	/// let b = &[1, 2, 3, 4];
	///
	/// assert!(a.is_subset(b));
	/// ```
	fn is_subset(&self, other: &Self) -> bool;
}

pub trait Superset {
	/// Returns true if `self` is an improper superset of `other`.
	///
	/// # Examples
	/// ```
	/// use kwik::math::set::Superset;
	///
	/// let a = &[1, 2, 3, 4];
	/// let b = &[1, 2, 3];
	///
	/// assert!(a.is_superset(b));
	/// ```
	fn is_superset(&self, other: &Self) -> bool;
}

impl<T> Subset for &[T]
where
	T: PartialEq,
{
	fn is_subset(&self, other: &Self) -> bool {
		!self.iter().any(|e| !other.contains(e))
	}
}

impl<T> Subset for [T]
where
	T: PartialEq,
{
	fn is_subset(&self, other: &Self) -> bool {
		(&self).is_subset(&other)
	}
}

impl<T> Subset for Vec<T>
where
	T: PartialEq,
{
	fn is_subset(&self, other: &Self) -> bool {
		let self_slice: &[T] = self;
		let other_slice: &[T] = other;

		self_slice.is_subset(other_slice)
	}
}

impl<T> Superset for &[T]
where
	T: PartialEq,
{
	fn is_superset(&self, other: &Self) -> bool {
		other.is_subset(self)
	}
}

impl<T> Superset for [T]
where
	T: PartialEq,
{
	fn is_superset(&self, other: &Self) -> bool {
		other.is_subset(self)
	}
}

impl<T> Superset for Vec<T>
where
	T: PartialEq,
{
	fn is_superset(&self, other: &Self) -> bool {
		other.is_subset(self)
	}
}

#[cfg(test)]
mod tests {
	use crate::math::set::{Subset, Superset};

	#[test]
	fn it_identifies_slice_subsets() {
		let a = &[1, 2, 3];
		let b = &[1, 2, 3, 4];
		let c = &[1, 2, 3];
		let d = &[1, 3, 4, 5];

		assert!(a.is_subset(b));
		assert!(a.is_subset(c));
		assert!(!a.is_subset(d));
	}

	#[test]
	fn it_identifies_array_subsets() {
		let a = [1, 2, 3];
		let b = [1, 2, 3, 4];
		let c = [1, 2, 3];
		let d = [1, 3, 4, 5];

		assert!(a.is_subset(&b));
		assert!(a.is_subset(&c));
		assert!(!a.is_subset(&d));
	}

	#[test]
	fn it_identifies_vec_subsets() {
		let a = vec![1, 2, 3];
		let b = vec![1, 2, 3, 4];
		let c = vec![1, 2, 3];
		let d = vec![1, 3, 4, 5];

		assert!(a.is_subset(&b));
		assert!(a.is_subset(&c));
		assert!(!a.is_subset(&d));
	}

	#[test]
	fn it_identifies_slice_supersets() {
		let a = &[1, 2, 3, 4];
		let b = &[1, 2, 3];
		let c = &[1, 2, 3, 4];
		let d = &[1, 3, 5];

		assert!(a.is_superset(b));
		assert!(a.is_superset(c));
		assert!(!a.is_superset(d));
	}

	#[test]
	fn it_identifies_array_supersets() {
		let a = [1, 2, 3, 4];
		let b = [1, 2, 3];
		let c = [1, 2, 3, 4];
		let d = [1, 3, 5];

		assert!(a.is_superset(&b));
		assert!(a.is_superset(&c));
		assert!(!a.is_superset(&d));
	}

	#[test]
	fn it_identifies_vec_supersets() {
		let a = vec![1, 2, 3, 4];
		let b = vec![1, 2, 3];
		let c = vec![1, 2, 3, 4];
		let d = vec![1, 3, 5];

		assert!(a.is_superset(&b));
		assert!(a.is_superset(&c));
		assert!(!a.is_superset(&d));
	}
}
