/*
 * Copyright (c) Kia Shakiba
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

use crate::genetic::MutateRng;

/// A gene defines the unit of change in a genetic algorithm. Implement this trait
/// for whichever struct contains the data for an individual member of the genetic
/// system.
///
/// # Examples
/// ```
/// struct MyData {
///     data: u32,
/// }
///
/// impl Gene<u32> for MyData {
///     fn value(&self) -> u32 {
///         self.data
///     }
///
///     fn mutate(&mut self, rng: &mut MutateRng) {
///         self.data = rng.gen_range(0..10);
///     }
/// }
/// ```
pub trait Gene<T>
where
	Self: Clone,
	T: Clone,
{
	/// Returns the value of the gene. This value is what is used to compute
	/// the fitness of an individual.
	fn value(&self) -> T;

	/// Mutates the value of the gene. Ensure the value is mutated only within
	/// the acceptable range of possible values.
	fn mutate(&mut self, _: &mut MutateRng);
}
