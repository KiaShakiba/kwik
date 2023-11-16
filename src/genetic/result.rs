/*
 * Copyright (c) Kia Shakiba
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

pub use crate::genetic::genes::Genes;

/// The result of a genetic run. Holds the genes of the fittest individual,
/// the number of generations processed during the run, and the total runtime
/// of the run.
pub struct GeneticResult<GS: Genes> {
	genes: GS,

	generations: u64,
	runtime: u64,
}

impl<GS: Genes> GeneticResult<GS> {
	pub fn new(genes: GS, generations: u64, runtime: u64) -> Self {
		GeneticResult {
			genes,

			generations,
			runtime,
		}
	}

	/// Returns a reference to the fittest individual's genes.
	pub fn genes(&self) -> &GS {
		&self.genes
	}

	/// Returns the number of generations processed during the run.
	pub fn generations(&self) -> u64 {
		self.generations
	}

	/// Returns the total runtime of the run.
	pub fn runtime(&self) -> u64 {
		self.runtime
	}
}
