/*
 * Copyright (c) Kia Shakiba
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

use std::{
	cmp::Ordering,
	time::{Duration, Instant},
};

use rand::Rng;

use crate::genetic::{
	error::GeneticError,
	chromosome::{Chromosome, Gene},
	fitness::Fitness,
};

#[derive(Clone)]
pub struct Individual<C>
where
	C: Chromosome,
{
	chromosome: C,
}

enum MateResult {
	Parent1,
	Parent2,
	Mutation,
}

impl<C> Individual<C>
where
	C: Chromosome,
{
	#[inline]
	pub fn chromosome(&self) -> &C {
		&self.chromosome
	}

	#[inline]
	pub fn is_optimal(&self) -> bool {
		self.chromosome.is_optimal()
	}

	pub fn mate(
		&self,
		rng: &mut impl Rng,
		partner: &Individual<C>,
		mutation_probability: f64,
		max_runtime: &Duration,
	) -> Result<Individual<C>, GeneticError> {
		let time = Instant::now();
		let mut child_chromosome = self.chromosome.base();

		loop {
			if time.elapsed().ge(max_runtime) {
				return Err(GeneticError::MateTimeout);
			}

			for i in 0..self.chromosome.len() {
				let gene = match get_mate_result(rng, mutation_probability) {
					MateResult::Parent1 => self.chromosome.get(i).clone(),
					MateResult::Parent2 => partner.chromosome.get(i).clone(),

					MateResult::Mutation => {
						let mut gene = self.chromosome.get(i).clone();
						gene.mutate(rng);
						gene
					},
				};

				child_chromosome.push(gene);
			}

			if child_chromosome.is_valid() {
				break;
			}

			child_chromosome.clear();
		}

		Ok(child_chromosome.into())
	}
}

impl<C> From<C> for Individual<C>
where
	C: Chromosome,
{
	fn from(chromosome: C) -> Self {
		Individual {
			chromosome,
		}
	}
}

impl<C> Ord for Individual<C>
where
	C: Chromosome,
{
	fn cmp(&self, other: &Self) -> Ordering {
		match self.chromosome.fitness_cmp(other.chromosome()) {
			Fitness::Stronger => Ordering::Less,
			Fitness::Weaker => Ordering::Greater,
			Fitness::Equal => Ordering::Equal,
		}
	}
}

impl<C> PartialOrd for Individual<C>
where
	C: Chromosome,
{
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

impl<C> PartialEq for Individual<C>
where
	C: Chromosome,
{
	fn eq(&self, other: &Self) -> bool {
		matches!(self.chromosome.fitness_cmp(other.chromosome()), Fitness::Equal)
	}
}

impl<C> Eq for Individual<C>
where
	C: Chromosome,
{}

fn get_mate_result(rng: &mut impl Rng, mutation_probability: f64) -> MateResult {
	let random: f64 = rng.gen();

	if random < (1.0 - mutation_probability) / 2.0 {
		return MateResult::Parent1;
	}

	if random < 1.0 - mutation_probability {
		return MateResult::Parent2;
	}

	MateResult::Mutation
}
