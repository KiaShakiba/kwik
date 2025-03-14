/*
 * Copyright (c) Kia Shakiba
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

use rand::Rng;

/// A gene defines the unit of change in a genetic algorithm. Implement this trait
/// for whichever struct contains the data for an individual member of the genetic
/// system.
///
/// # Examples
/// ```
/// use kwik::genetic::{Gene, Chromosome, Rng};
///
/// #[derive(Clone)]
/// struct MyData {
///     data: u32,
/// }
///
/// impl Gene for MyData {
///     fn mutate(&mut self, rng: &mut impl Rng, _genes: &[Option<Self>]) {
///         self.data = rng.gen_range(0..10);
///     }
/// }
/// ```
pub trait Gene
where
	Self: Clone,
{
	/// Mutates the value of the gene. Ensure the value is mutated only within
	/// the acceptable range of possible values. The current (potentially partially
	/// filled) chromosome is provided.
	fn mutate(&mut self, rng: &mut impl Rng, genes: &[Option<Self>]);
}
