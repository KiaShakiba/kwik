/*
 * Copyright (c) Kia Shakiba
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

use std::cmp::{Ord, Ordering};

use rand::{
	Rng,
	rngs::ThreadRng,
};

use crate::genetic::genes::{Genes, Gene};

#[derive(Clone, PartialEq)]
pub struct Individual<GS>
where
	GS: Genes,
{
	genes: GS,
}

enum MateResult {
	Parent1,
	Parent2,
	Mutation,
}

impl<GS> Individual<GS>
where
	GS: Genes,
{
	pub fn new(genes: GS) -> Self {
		Individual {
			genes,
		}
	}

	pub fn genes(&self) -> &GS {
		&self.genes
	}

	pub fn is_optimal(&self) -> bool {
		self.genes.is_optimal()
	}

	pub fn mate(
		&self,
		rng: &mut ThreadRng,
		partner: &Individual<GS>,
		mutation_probability: f64
	) -> Individual<GS> {
		let mut child_genes = self.genes.base();

		loop {
			for i in 0..self.genes.len() {
				let gene = match get_mate_result(rng, mutation_probability) {
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

		Individual::<GS>::new(child_genes)
	}
}

fn get_mate_result(rng: &mut ThreadRng, mutation_probability: f64) -> MateResult {
	let random: f64 = rng.gen();

	if random < (1.0 - mutation_probability) / 2.0 {
		return MateResult::Parent1;
	}

	if random < 1.0 - mutation_probability {
		return MateResult::Parent2;
	}

	MateResult::Mutation
}

impl<GS> Ord for Individual<GS>
where
	GS: Genes,
{
	fn cmp(&self, other: &Self) -> Ordering {
		self.genes.cmp(&other.genes)
	}
}

impl<GS> PartialOrd for Individual<GS>
where
	GS: Genes,
{
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

impl<GS> Eq for Individual<GS>
where
	GS: Genes,
{}
