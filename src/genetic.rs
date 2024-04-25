/*
 * Copyright (c) Kia Shakiba
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

mod error;
mod individual;
mod gene;
mod chromosome;
mod solution;

use std::time::{Duration, Instant};

use rand::{
	thread_rng,
	distributions::{Distribution, Uniform},
};

pub use rand::{
	Rng,
	rngs::ThreadRng,
};

use crate::genetic::individual::Individual;

pub use crate::genetic::{
	error::GeneticError,
	chromosome::{Chromosome, Gene},
	solution::GeneticSolution,
};

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
/// use kwik::genetic::{Genetic, Gene, Chromosome, MutateRng, Rng};
///
/// #[derive(Clone, Ord, PartialOrd, PartialEq, Eq)]
/// struct MyData {
///     data: u32,
/// }
///
/// #[derive(Default, Clone, Ord, PartialOrd, PartialEq, Eq)]
/// struct MyConfig {
///     config: Vec<MyData>,
/// }
///
/// let mut initial_chromosome = MyConfig::default();
///
/// initial_chromosome.push(MyData { data: 0 });
/// initial_chromosome.push(MyData { data: 0 });
/// initial_chromosome.push(MyData { data: 0 });
/// initial_chromosome.push(MyData { data: 0 });
/// initial_chromosome.push(MyData { data: 0 });
///
/// let mut genetic = Genetic::<MyConfig>::new(initial_chromosome).unwrap();
/// let result = genetic.run();
///
/// impl Chromosome for MyConfig {
///     type Gene = MyData;
///
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
///         self.data = rng.gen_range(0..50);
///     }
/// }
/// ```
pub struct Genetic<C>
where
	C: Chromosome,
{
	initial_chromosome: C,
	population: Vec<Individual<C>>,

	population_size: usize,
	convergence_limit: u64,
	max_runtime: Duration,
	mutation_probability: f64,
	elite_ratio: f64,
	mating_ratio: f64,

	rng: ThreadRng,
	mating_dist: Uniform<usize>,
}

impl<C> Genetic<C>
where
	C: Chromosome,
{
	/// Creates an instance of the genetic runner using the supplied
	/// chromosome as the initial value.
	pub fn new(initial_chromosome: C) -> Result<Self, GeneticError> {
		if !initial_chromosome.is_valid() {
			return Err(GeneticError::InvalidInitialChromosome);
		}

		let mut population = Vec::<Individual<C>>::new();

		for _ in 0..POPULATION_SIZE {
			population.push(Individual::new(initial_chromosome.clone()));
		}

		let genetic = Genetic {
			initial_chromosome,
			population,

			population_size: POPULATION_SIZE,
			convergence_limit: CONVERGENCE_LIMIT,
			max_runtime: Duration::from_millis(MAX_RUNTIME),
			mutation_probability: MUTATION_PROBABILITY,
			elite_ratio: ELITE_RATIO,
			mating_ratio: MATING_RATIO,

			rng: thread_rng(),
			mating_dist: init_mating_dist(POPULATION_SIZE, MATING_RATIO),
		};

		Ok(genetic)
	}

	/// Sets the population size and fills the population with individuals.
	#[inline]
	pub fn set_population_size(&mut self, population_size: usize) {
		self.population_size = population_size;
		self.population.clear();

		for _ in 0..population_size {
			self.population.push(Individual::new(self.initial_chromosome.clone()));
		}

		self.mating_dist = init_mating_dist(self.population_size, self.mating_ratio);
	}

	/// Sets the population size and fills the population with individuals.
	#[inline]
	#[must_use]
	pub fn with_population_size(mut self, population_size: usize) -> Self {
		self.set_population_size(population_size);
		self
	}

	/// Sets the convergence.
	#[inline]
	pub fn set_convergence_limit(&mut self, convergence_limit: u64) {
		self.convergence_limit = convergence_limit;
	}

	/// Sets the convergence.
	#[inline]
	#[must_use]
	pub fn with_convergence_limit(mut self, convergence_limit: u64) -> Self {
		self.set_convergence_limit(convergence_limit);
		self
	}

	/// Sets the max runtime.
	#[inline]
	pub fn set_max_runtime(&mut self, max_runtime: Duration) {
		self.max_runtime = max_runtime;
	}

	/// Sets the max runtime.
	#[inline]
	#[must_use]
	pub fn with_max_runtime(mut self, max_runtime: Duration) -> Self {
		self.set_max_runtime(max_runtime);
		self
	}

	/// Sets the mutation probability.
	#[inline]
	pub fn set_mutation_probability(&mut self, mutation_probability: f64) {
		self.mutation_probability = mutation_probability;
	}

	/// Sets the mutation probability.
	#[inline]
	#[must_use]
	pub fn with_mutation_probability(mut self, mutation_probability: f64) -> Self {
		self.set_mutation_probability(mutation_probability);
		self
	}

	/// Sets the elite ratio.
	#[inline]
	pub fn set_elite_ratio(&mut self, elite_ratio: f64) {
		self.elite_ratio = elite_ratio;
	}

	/// Sets the elite ratio.
	#[inline]
	#[must_use]
	pub fn with_elite_ratio(mut self, elite_ratio: f64) -> Self {
		self.set_elite_ratio(elite_ratio);
		self
	}

	/// Sets the mating ratio.
	#[inline]
	pub fn set_mating_ratio(&mut self, mating_ratio: f64) {
		self.mating_ratio = mating_ratio;
		self.mating_dist = init_mating_dist(self.population_size, self.mating_ratio);
	}

	/// Sets the mating ratio.
	#[inline]
	#[must_use]
	pub fn with_mating_ratio(mut self, mating_ratio: f64) -> Self {
		self.set_mating_ratio(mating_ratio);
		self
	}

	/// Runs the genetic algorithm until either the most fit individual has a fitness
	/// of 0 or the population has converged and is no longer changing.
	pub fn run(&mut self) -> Result<GeneticSolution<C>, GeneticError> {
		let time = Instant::now();

		self.iterate()?;

		let mut generation_count: u64 = 1;
		let mut convergence_count: u64 = 0;
		let mut last_fittest = self.population[0].clone();

		while
			!last_fittest.is_optimal()
				&& convergence_count < self.convergence_limit
				&& time.elapsed().lt(&self.max_runtime)
		{
			self.iterate()?;

			let fittest = &self.population[0];

			if fittest.eq(&last_fittest) {
				convergence_count += 1;
			} else {
				last_fittest = fittest.clone();
				convergence_count = 0;
			}

			generation_count += 1;
		}

		let solution = GeneticSolution::new(
			self.population[0].chromosome().clone(),
			generation_count,
			time.elapsed(),
		);

		Ok(solution)
	}

	/// Performs one iteration of the genetic algorithm, creating a new generation
	/// and overwriting the current population.
	fn iterate(&mut self) -> Result<(), GeneticError> {
		let elite_population = (self.population_size as f64 * self.elite_ratio) as usize;

		let mut new_generation = self.population
			.iter()
			.take(elite_population)
			.cloned()
			.collect::<Vec::<Individual<C>>>();

		for _ in 0..(self.population_size - elite_population) {
			let (index1, index2) = self.gen_mating_pair();

			let parent1 = &self.population[index1];
			let parent2 = &self.population[index2];

			let child = parent1.mate(
				&mut self.rng,
				parent2,
				self.mutation_probability,
				&self.max_runtime,
			)?;

			new_generation.push(child);
		}

		new_generation.sort();

		self.population = new_generation;

		Ok(())
	}

	fn gen_mating_pair(&mut self) -> (usize, usize) {
		let index1 = self.mating_dist.sample(&mut self.rng);
		let mut index2 = self.mating_dist.sample(&mut self.rng);

		while index1 == index2 {
			index2 = self.mating_dist.sample(&mut self.rng);
		}

		(index1, index2)
	}
}

fn init_mating_dist(
	population_size: usize,
	mating_ratio: f64,
) -> Uniform<usize> {
	let mating_population = (population_size as f64 * mating_ratio) as usize;
	Uniform::from(0..mating_population)
}
