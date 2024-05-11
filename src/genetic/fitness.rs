/*
 * Copyright (c) Kia Shakiba
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

/// The relative fitness of two chromosomes.
pub enum Fitness {
	Stronger,
	Weaker,
	Equal,
}

/// This allows for the comparison of two chromosomes.
pub trait FitnessOrd {
	/// Compares the current chromosome with the `other` chromosome.
	/// Return `Fitness::Stronger` if the current chromosome is more
	/// fit than the `other`. Return `Fitness::Weaker` if the current
	/// chromosome is less fit than the `other`. Return `Fitness::Equal`
	/// if the two chromosomes' fitnesses are equal.
	///
	/// # Examples
	/// ```
	/// use kwik::genetic::{Fitness, FitnessOrd};
	///
	/// struct MyConfig {
	///     data: u32,
	/// }
	///
	/// impl FitnessOrd for MyConfig {
	///     fn fitness_cmp(&self, other: &Self) -> Fitness {
	///         if self.data < other.data {
	///             // current is stronger than other
	///             return Fitness::Stronger;
	///         }
	///
	///         if self.data > other.data {
	///             // current is weaker than other
	///             return Fitness::Weaker;
	///         }
	///
	///         // current is equal to other
	///         Fitness::Equal
	///     }
	/// }
	/// ```
	fn fitness_cmp(&self, other: &Self) -> Fitness;
}
