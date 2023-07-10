/*
 * Copyright (c) Kia Shakiba
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

use std::marker::PhantomData;
use std::cmp::{Ord, Ordering};
use rand::Rng;
use rand::rngs::ThreadRng;
use crate::genetic::{Fitness, MUTATION_PROBABILITY};
use crate::genetic::genes::{Genes, Gene};

pub const FITNESS_EPSILON: Fitness = 0.000001;

#[derive(Clone)]
pub struct Individual<G, GS>
where
	G: Gene,
	GS: Genes<G>,
{
	genes: GS,

	_gene_marker: PhantomData<G>,
}

enum MateResult {
	Parent1,
	Parent2,
	Mutation,
}

impl<G, GS> Individual<G, GS>
where
	G: Gene,
	GS: Genes<G>,
{
	pub fn new(genes: GS) -> Self {
		Individual {
			genes,

			_gene_marker: PhantomData,
		}
	}

	pub fn genes(&self) -> &GS {
		&self.genes
	}

	pub fn fitness(&self) -> Fitness {
		self.genes.fitness()
	}

	pub fn mate(
		&self,
		rng: &mut ThreadRng,
		partner: &Individual<G, GS>
	) -> Individual<G, GS> {
		let mut child_genes = self.genes.base();

		loop {
			for i in 0..self.genes.len() {
				let gene = match get_mate_result(rng) {
					MateResult::Parent1 => self.genes.get(i).clone(),
					MateResult::Parent2 => partner.genes.get(i).clone(),

					MateResult::Mutation => {
						let mut gene = self.genes.get(i).clone();
						gene.mutate(rng);
						gene
					},
				};

				child_genes.push(gene);
			}

			if !child_genes.is_valid() {
				child_genes.clear();
			} else {
				break;
			}
		}

		Individual::<G, GS>::new(child_genes)
	}
}

fn get_mate_result(rng: &mut ThreadRng) -> MateResult {
	let random: f64 = rng.gen();

	if random < (1.0 - MUTATION_PROBABILITY) / 2.0 {
		return MateResult::Parent1;
	}

	if random < 1.0 - MUTATION_PROBABILITY {
		return MateResult::Parent2;
	}

	MateResult::Mutation
}

impl<G, GS> Ord for Individual<G, GS>
where
	G: Gene,
	GS: Genes<G>,
{
	fn cmp(&self, other: &Self) -> Ordering {
		self.partial_cmp(other).unwrap()
	}
}

impl<G, GS> PartialOrd for Individual<G, GS>
where
	G: Gene,
	GS: Genes<G>,
{
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		self.fitness().abs().partial_cmp(&other.fitness().abs())
	}
}

impl<G, GS> PartialEq for Individual<G, GS>
where
	G: Gene,
	GS: Genes<G>,
{
	fn eq(&self, other: &Self) -> bool {
		(self.fitness() - other.fitness()).abs() < FITNESS_EPSILON
	}
}

impl<G, GS> Eq for Individual<G, GS>
where
	G: Gene,
	GS: Genes<G>,
{}
