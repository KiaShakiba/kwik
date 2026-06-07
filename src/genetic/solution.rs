/*
 * Copyright (c) Kia Shakiba
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

use std::time::Duration;

use crate::genetic::chromosome::Chromosome;

/// The solution of a genetic run. Holds the chromosome of the fittest
/// individual, the number of generations processed during the run, and the
/// total duration of the run.
pub struct GeneticSolution<C>
where
	C: Chromosome,
{
	chromosomes: Vec<C>,

	generations: u64,
	mutations:   u64,

	runtime: Duration,
}

impl<C> GeneticSolution<C>
where
	C: Chromosome,
{
	pub fn new(chromosomes: Vec<C>, generations: u64, mutations: u64, runtime: Duration) -> Self {
		GeneticSolution {
			chromosomes,

			generations,
			mutations,

			runtime,
		}
	}

	/// Returns a reference to the fittest individual's chromosome.
	#[inline]
	pub fn fittest(&self) -> &C {
		&self.chromosomes[0]
	}

	/// Returns a reference to the individuals' chromosomes, ordered by most to
	/// least fit.
	pub fn chromosomes(&self) -> &[C] {
		&self.chromosomes
	}

	/// Returns the number of generations processed during the run.
	#[inline]
	pub fn generations(&self) -> u64 {
		self.generations
	}

	/// Returns the total number of mutations that occurred during the run.
	#[inline]
	pub fn mutations(&self) -> u64 {
		self.mutations
	}

	/// Returns the total runtime of the run.
	#[inline]
	pub fn runtime(&self) -> Duration {
		self.runtime
	}
}
