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

use rand::{Rng, seq::SliceRandom};

use crate::genetic::{
	chromosome::{Chromosome, Gene},
	error::GeneticError,
	fitness::Fitness,
	offspring::Offspring,
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
		maybe_max_runtime: Option<&Duration>,
	) -> Result<Offspring<C>, GeneticError> {
		let time = Instant::now();
		let mut mutations = 0u64;

		let mut child_chromosome = self.chromosome.base();
		let mut child_genes = vec![None; self.chromosome.len()];

		loop {
			if let Some(max_runtime) = maybe_max_runtime
				&& time.elapsed().ge(max_runtime)
			{
				return Err(GeneticError::MateTimeout);
			}

			let mut gene_indexes =
				(0..self.chromosome.len()).collect::<Vec<_>>();
			gene_indexes.shuffle(rng);

			for index in gene_indexes {
				let gene = match get_mate_result(rng, mutation_probability) {
					MateResult::Parent1 => self.chromosome.get(index).clone(),
					MateResult::Parent2 => {
						partner.chromosome.get(index).clone()
					},

					MateResult::Mutation => {
						mutations += 1;

						let mut gene = self.chromosome.get(index).clone();

						gene.mutate(rng, &child_genes);
						gene
					},
				};

				child_genes[index] = Some(gene);
			}

			for gene in child_genes.iter_mut() {
				let gene = gene.take().ok_or(GeneticError::Internal)?;

				child_chromosome.push(gene);
			}

			if child_chromosome.len() != self.chromosome.len() {
				return Err(GeneticError::Internal);
			}

			if child_chromosome.is_valid() {
				break;
			}

			child_chromosome.clear();

			child_genes.clear();
			child_genes.resize(self.chromosome.len(), None);
		}

		let offspring = Offspring::new(child_chromosome.into(), mutations);

		Ok(offspring)
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
		matches!(
			self.chromosome.fitness_cmp(other.chromosome()),
			Fitness::Equal
		)
	}
}

impl<C> Eq for Individual<C> where C: Chromosome {}

fn get_mate_result(
	rng: &mut impl Rng,
	mutation_probability: f64,
) -> MateResult {
	let random: f64 = rng.random();

	if random < (1.0 - mutation_probability) / 2.0 {
		return MateResult::Parent1;
	}

	if random < 1.0 - mutation_probability {
		return MateResult::Parent2;
	}

	MateResult::Mutation
}
