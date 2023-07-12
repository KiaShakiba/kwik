/*
 * Copyright (c) Kia Shakiba
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

mod individual;
mod gene;
mod genes;
mod result;

pub use rand::Rng;
use rand::thread_rng;
pub use rand::rngs::ThreadRng;
use crate::utils;
use crate::genetic::individual::Individual;
pub use crate::genetic::genes::{Genes, Gene};
pub use crate::genetic::result::GeneticResult;

pub type MutateRng = ThreadRng;

const POPULATION_SIZE: usize = 100;
const CONVERGENCE_LIMIT: u64 = 1_000;
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
	initial_genes: GS,
	population: Vec<Individual<G, GS>>,

	population_size: usize,
	convergence_limit: u64,
	max_runtime: u64,
	mutation_probability: f64,
	elite_ratio: f64,
	mating_ratio: f64,

	rng: ThreadRng,
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
			initial_genes,
			population,

			population_size: POPULATION_SIZE,
			convergence_limit: CONVERGENCE_LIMIT,
			max_runtime: MAX_RUNTIME,
			mutation_probability: MUTATION_PROBABILITY,
			elite_ratio: ELITE_RATIO,
			mating_ratio: MATING_RATIO,

			rng: thread_rng(),
		}
	}

	/// Sets the population size using the builder pattern and fills the
	/// population with individuals.
	pub fn set_population_size(mut self, population_size: usize) -> Self {
		self.population_size = population_size;
		self.population.clear();

		for _ in 0..population_size {
			self.population.push(Individual::new(self.initial_genes.clone()));
		}

		self
	}

	/// Sets the convergence using the builder pattern.
	pub fn set_convergence_limit(mut self, convergence_limit: u64) -> Self {
		self.convergence_limit = convergence_limit;
		self
	}

	/// Sets the max runtime using the builder pattern.
	pub fn set_max_runtime(mut self, max_runtime: u64) -> Self {
		self.max_runtime = max_runtime;
		self
	}

	/// Sets the mutation probability using the builder pattern.
	pub fn set_mutation_probability(mut self, mutation_probability: f64) -> Self {
		self.mutation_probability = mutation_probability;
		self
	}

	/// Sets the elite ratio using the builder pattern.
	pub fn set_elite_ratio(mut self, elite_ratio: f64) -> Self {
		self.elite_ratio = elite_ratio;
		self
	}

	/// Sets the mating ratio using the builder pattern.
	pub fn set_mating_ratio(mut self, mating_ratio: f64) -> Self {
		self.mating_ratio = mating_ratio;
		self
	}

	/// Runs the genetic algorithm until either the most fit individual has a fitness
	/// of 0 or the population has converged and is no longer changing.
	///
	/// A reference to the most fit individual is returned.
	pub fn run(&mut self) -> GeneticResult<G, GS> {
		let start = utils::timestamp();

		self.iterate();

		let mut generation_count: u64 = 1;
		let mut convergence_count: u64 = 0;
		let mut last_fittest = self.population[0].clone();

		while
			!last_fittest.is_optimal() &&
			convergence_count < self.convergence_limit &&
			(utils::timestamp() - start) < self.max_runtime
		{
			self.iterate();

			let fittest = &self.population[0];

			if !fittest.eq(&last_fittest) {
				last_fittest = fittest.clone();
				convergence_count = 0;
			} else {
				convergence_count += 1;
			}

			generation_count += 1;
		}

		GeneticResult::new(
			self.population[0].genes().clone(),
			generation_count,
			utils::timestamp() - start
		)
	}

	/// Performs one iteration of the genetic algorithm, creating a new generation
	/// and overwriting the current population.
	fn iterate(&mut self) {
		let elite_population = (self.population_size as f64 * self.elite_ratio) as usize;
		let mating_population = (self.population_size as f64 * self.mating_ratio) as usize;

		let mut new_generation = self.population
			.iter()
			.take(elite_population)
			.cloned()
			.collect::<Vec::<Individual<G, GS>>>();

		for _ in 0..(self.population_size - elite_population) {
			let index1: usize = self.rng.gen_range(0..mating_population);
			let mut index2: usize = self.rng.gen_range(0..mating_population);

			while index1 == index2 {
				index2 = self.rng.gen_range(0..mating_population);
			}

			let parent1 = &self.population[index1];
			let parent2 = &self.population[index2];
			let child = parent1.mate(&mut self.rng, parent2, self.mutation_probability);

			new_generation.push(child);
		}

		new_generation.sort();

		self.population = new_generation;
	}
}
