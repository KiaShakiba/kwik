/*
 * Copyright (c) Kia Shakiba
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

use crate::genetic::FitnessOrd;
pub use crate::genetic::gene::Gene;

/// This defines a chromosome (i.e., a set of genes). With this,
/// genes can be added and retrieved. The overall fitness of the
/// chromosome can also be computed.
///
/// # Examples
/// ```
/// use kwik::genetic::{
///     Gene,
///     Chromosome,
///     Fitness,
///     FitnessOrd,
///     Rng,
/// };
///
/// #[derive(Clone)]
/// struct MyData {
///     data: u32,
/// }
///
/// #[derive(Clone)]
/// struct MyConfig {
///     config: Vec<MyData>,
/// }
///
/// impl Chromosome for MyConfig {
///     type Gene = MyData;
///
///     fn base(&self) -> Self {
///         MyConfig {
///             config: Vec::new(),
///         }
///     }
///
///     fn is_empty(&self) -> bool {
///         self.config.is_empty()
///     }
///
///     fn len(&self) -> usize {
///         self.config.len()
///     }
///
///     fn insert(&mut self, _index: usize, data: MyData) {
///         self.config.push(data);
///     }
///
///     fn get(&self, index: usize) -> &MyData {
///         &self.config[index]
///     }
///
///     fn clear(&mut self) {
///         self.config.clear();
///     }
///
///     fn is_valid(&self) -> bool {
///         true
///     }
///
///     fn is_optimal(&self) -> bool {
///         self.sum() == 100
///     }
/// }
///
/// impl MyConfig {
///     fn sum(&self) -> u32 {
///         self.config
///             .iter()
///             .map(|item| item.data)
///             .sum::<u32>()
///     }
/// }
///
/// impl FitnessOrd for MyConfig {
///     fn fitness_cmp(&self, other: &Self) -> Fitness {
///         let self_diff = (100 - self.sum() as i32).abs();
///         let other_diff = (100 - other.sum() as i32).abs();
///
///         if self_diff < other_diff {
///             return Fitness::Stronger;
///         }
///
///         if self_diff > other_diff {
///             return Fitness::Weaker;
///         }
///
///         Fitness::Equal
///     }
/// }
///
/// impl Gene for MyData {
///     fn mutate(&mut self, rng: &mut impl Rng, _chromosome: &impl Chromosome) {
///         self.data = rng.gen_range(0..10);
///     }
/// }
/// ```
pub trait Chromosome
where
	Self: Clone + FitnessOrd,
{
	type Gene: Gene;

	/// Creates a new, empty instance of the base chromosome.
	#[must_use]
	fn base(&self) -> Self;

	/// Returns true if there are no genes.
	#[must_use]
	fn is_empty(&self) -> bool;

	/// Returns the number of genes.
	#[must_use]
	fn len(&self) -> usize;

	/// Inserts a gene into the chromosome at the specified index. If the index
	/// does not matter, it can be ignored and the gene can be inserted at any
	/// index.
	fn insert(&mut self, index: usize, gene: Self::Gene);

	/// Clears the chromosome.
	fn clear(&mut self);

	/// Retrieves a reference to a gene from the chromosome.
	#[must_use]
	fn get(&self, index: usize) -> &Self::Gene;

	/// Returns true if the chromosome is valid.
	#[must_use]
	fn is_valid(&self) -> bool {
		true
	}

	/// Returns true if the chromosome produces an optimal result.
	/// This will stop the genetic algorithm.
	#[must_use]
	fn is_optimal(&self) -> bool;

	/// Inserts a gene at the end of the chromosome.
	fn push(&mut self, gene: Self::Gene) {
		self.insert(self.len(), gene);
	}

	/// Returns the value of the partially filled chromosome to be used
	/// while mutating genes, if one exists.
	#[must_use]
	fn partial_value<T>(&self) -> Option<T> {
		None
	}
}
