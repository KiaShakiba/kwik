/*
 * Copyright (c) Kia Shakiba
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

pub use crate::genetic::gene::Gene;

/// This defines a set of genes. With this, genes can be added and
/// retrieved. The overall fitness of the genes can also be computed.
///
/// # Examples
/// ```
/// use kwik::genetic::{Gene, Genes, MutateRng, Rng};
///
/// #[derive(Clone, Ord, PartialOrd, PartialEq, Eq)]
/// struct MyData {
///     data: u32,
/// }
///
/// #[derive(Clone, Ord, PartialOrd, PartialEq, Eq)]
/// struct MyConfig {
///     config: Vec<MyData>,
/// }
///
/// impl Genes<MyData> for MyConfig {
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
///     fn push(&mut self, data: MyData) {
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
///         let sum = self.config
///             .iter()
///             .map(|item| item.data)
///             .sum::<u32>();
///
///         sum == 100
///     }
/// }
///
/// impl Gene for MyData {
///     fn mutate(&mut self, rng: &mut MutateRng) {
///         self.data = rng.gen_range(0..10);
///     }
/// }
/// ```
pub trait Genes<G>
where
	Self: Clone + Ord,
	G: Gene,
{
	/// Creates a new, empty instance of the base genes.
	fn base(&self) -> Self;

	/// Returns true if there are no genes.
	fn is_empty(&self) -> bool;

	/// Returns the number of genes.
	fn len(&self) -> usize;

	/// Adds a gene to the genes.
	fn push(&mut self, _: G);

	/// Clears the genes.
	fn clear(&mut self);

	/// Retrieves a reference to a gene from the genes.
	fn get(&self, _: usize) -> &G;

	/// Returns true if the genes are valid.
	fn is_valid(&self) -> bool {
		true
	}

	/// Returns true if the genes produce an optimal result.
	/// This will stop the genetic algorithm.
	fn is_optimal(&self) -> bool;
}
