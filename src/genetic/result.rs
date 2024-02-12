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
pub struct GeneticResult<GS>
where
	GS: Genes,
{
	genes: GS,

	generations: u64,
	runtime: u64,
}

impl<GS> GeneticResult<GS>
where
	GS: Genes,
{
	pub fn new(genes: GS, generations: u64, runtime: u64) -> Self {
		GeneticResult {
			genes,

			generations,
			runtime,
		}
	}

	/// Returns a reference to the fittest individual's genes.
	#[inline]
	pub fn genes(&self) -> &GS {
		&self.genes
	}

	/// Returns the number of generations processed during the run.
	#[inline]
	pub fn generations(&self) -> u64 {
		self.generations
	}

	/// Returns the total runtime of the run.
	#[inline]
	pub fn runtime(&self) -> u64 {
		self.runtime
	}
}
