/*
 * Copyright (c) Kia Shakiba
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

mod individual;
mod gene;
mod genes;

use std::marker::PhantomData;
pub use rand::Rng;
use rand::thread_rng;
pub use rand::rngs::ThreadRng;
use crate::utils;
use crate::genetic::individual::{Individual, FITNESS_EPSILON};
pub use crate::genetic::genes::{Genes, Gene};

pub type Fitness = f64;
pub type MutateRng = ThreadRng;

const POPULATION_SIZE: usize = 100;
const CONVERGENCE_SIZE: u32 = 1_000;
const MAX_RUNTIME: u64 = 30_000;
const MUTATION_PROBABILITY: f64 = 0.1;
const ELITE_RATIO: f64 = 0.1;
const MATING_RATIO: f64 = 0.5;

/// Finds the optimal values for a set of inputs using a genetic algorithm.
///
/// # Examples
/// ```
/// struct MyData {
///     data: u32,
/// }
///
/// struct MyConfig {
///     config: Vec<MyData>,
/// }
///
/// let mut initial_genes = MyConfig::new();
///
/// initial_genes.push(MyData { data: 0 });
/// initial_genes.push(MyData { data: 0 });
/// initial_genes.push(MyData { data: 0 });
/// initial_genes.push(MyData { data: 0 });
/// initial_genes.push(MyData { data: 0 });
///
/// let mut genetic = Genetic::<u32, MyData, MyConfig>::new(initial_genes);
/// let optimal_config = genetic.run();
/// ```
pub struct Genetic<G: Gene, GS: Genes<G>>
where
	G: Gene,
	GS: Genes<G>,
{
	population: Vec<Individual<G, GS>>,
	generation_count: usize,

	rng: ThreadRng,

	_gene_marker: PhantomData<G>,
}

impl<G, GS> Genetic<G, GS>
where
	G: Gene,
	GS: Genes<G>,
{
	/// Creates an instance of the genetic runner using the supplied genes as initial values.
	pub fn new(initial_genes: GS) -> Self {
		if !initial_genes.is_valid() {
			panic!("Invalid initial genes.");
		}

		let mut population = Vec::<Individual<G, GS>>::new();

		for _ in 0..POPULATION_SIZE {
			population.push(Individual::new(initial_genes.clone()));
		}

		Genetic {
			population,
			generation_count: 0,

			rng: thread_rng(),

			_gene_marker: PhantomData,
		}
	}

	/// Runs the genetic algorithm until either the most fit individual has a fitness
	/// of 0 or the population has converged and is no longer changing.
	///
	/// A reference to the most fit individual is returned.
	pub fn run(&mut self) -> &GS {
		let start = utils::timestamp();

		let mut convergence_count: u32 = 0;
		let mut last_fitness = self.iterate();

		while
			last_fitness.abs() > FITNESS_EPSILON &&
			convergence_count < CONVERGENCE_SIZE &&
			(utils::timestamp() - start) < MAX_RUNTIME
		{
			let fitness = self.iterate();

			if (fitness - last_fitness).abs() > FITNESS_EPSILON {
				last_fitness = fitness;
				convergence_count = 0;
			} else {
				convergence_count += 1;
			}
		}

		self.population[0].genes()
	}

	/// Performs one iteration of the genetic algorithm, creating a new generation
	/// and overwriting the current population.
	fn iterate(&mut self) -> Fitness {
		self.generation_count += 1;

		let elite_population = (POPULATION_SIZE as f64 * ELITE_RATIO) as usize;
		let mating_population = (POPULATION_SIZE as f64 * MATING_RATIO) as usize;

		let mut new_generation = self.population
			.iter()
			.take(elite_population)
			.cloned()
			.collect::<Vec::<Individual<G, GS>>>();

		for _ in 0..(POPULATION_SIZE - elite_population) {
			let index1: usize = self.rng.gen_range(0..mating_population);
			let mut index2: usize = self.rng.gen_range(0..mating_population);

			while index1 == index2 {
				index2 = self.rng.gen_range(0..mating_population);
			}

			let parent1 = &self.population[index1];
			let parent2 = &self.population[index2];
			let child = parent1.mate(&mut self.rng, parent2);

			new_generation.push(child);
		}

		new_generation.sort();

		self.population = new_generation;
		self.population[0].fitness()
	}
}
