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
use rayon::prelude::*;

use rand::{
	thread_rng,
	distributions::{Distribution, Uniform},
};

pub use rand::{
	Rng,
	SeedableRng,
	rngs::SmallRng,
};

use crate::genetic::individual::Individual;

pub use crate::genetic::{
	error::GeneticError,
	chromosome::{Chromosome, Gene},
	solution::GeneticSolution,
};

const POPULATION_SIZE: usize = 100;
const CONVERGENCE_LIMIT: u64 = 1_000;
const MAX_RUNTIME: u64 = 30_000;
const MUTATION_PROBABILITY: f64 = 0.1;
const TOURNAMENT_SIZE: usize = 3;

/// Finds the optimal values for a set of inputs using a genetic algorithm.
///
/// # Examples
/// ```
/// use kwik::genetic::{Genetic, Gene, Chromosome, Rng};
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
///     fn mutate(&mut self, rng: &mut impl Rng) {
///         self.data = rng.gen_range(0..50);
///     }
/// }
/// ```
pub struct Genetic<C>
where
	C: Chromosome + Send + Sync,
{
	initial_chromosome: C,
	population: Vec<Individual<C>>,

	convergence_limit: u64,
	max_runtime: Duration,
	mutation_probability: f64,
	tournament_size: usize,

	mating_dist: Uniform<usize>,
}

impl<C> Genetic<C>
where
	C: Chromosome + Send + Sync,
{
	/// Creates an instance of the genetic runner using the supplied
	/// chromosome as the initial value.
	pub fn new(initial_chromosome: C) -> Result<Self, GeneticError> {
		if initial_chromosome.is_empty() {
			return Err(GeneticError::EmptyInitialChromosome);
		}

		if !initial_chromosome.is_valid() {
			return Err(GeneticError::InvalidInitialChromosome);
		}

		let max_runtime = Duration::from_millis(MAX_RUNTIME);
		let mut population = vec![];

		init_population(
			&mut population,
			POPULATION_SIZE,
			&initial_chromosome,
			&max_runtime,
		)?;

		let mutation_probability = match initial_chromosome.len() {
			0 => MUTATION_PROBABILITY,
			num_genes => 1.0 / num_genes as f64,
		};

		let genetic = Genetic {
			initial_chromosome,
			population,

			convergence_limit: CONVERGENCE_LIMIT,
			max_runtime,
			mutation_probability,
			tournament_size: TOURNAMENT_SIZE,

			mating_dist: init_mating_dist(POPULATION_SIZE),
		};

		Ok(genetic)
	}

	/// Sets the population size and fills the population with individuals.
	///
	/// # Errors
	///
	/// This function returns an error if the population size is zero.
	#[inline]
	pub fn set_population_size(&mut self, population_size: usize) -> Result<(), GeneticError> {
		if population_size == 0 {
			return Err(GeneticError::InvalidPopulationSize);
		}

		init_population(
			&mut self.population,
			population_size,
			&self.initial_chromosome,
			&self.max_runtime,
		)?;

		self.mating_dist = init_mating_dist(population_size);

		Ok(())
	}

	/// Sets the population size and fills the population with individuals.
	///
	/// # Errors
	///
	/// This function returns an error if the population size is zero.
	#[inline]
	pub fn with_population_size(mut self, population_size: usize) -> Result<Self, GeneticError> {
		self.set_population_size(population_size)?;
		Ok(self)
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

	/// Sets the tournament size.
	#[inline]
	pub fn set_tournament_size(&mut self, tournament_size: usize) {
		self.tournament_size = tournament_size;
	}

	/// Sets the tournament size.
	#[inline]
	#[must_use]
	pub fn with_tournament_size(mut self, tournament_size: usize) -> Self {
		self.set_tournament_size(tournament_size);
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
		let population_size = self.population.len();

		let mut new_generation = (0..population_size)
			.into_par_iter()
			.map(|_| {
				let mut rng = SmallRng::from_rng(thread_rng())
					.map_err(|_| GeneticError::Internal)?;

				let (parent1, parent2) = self.gen_mating_pair(&mut rng);

				parent1.mate(
					&mut rng,
					parent2,
					self.mutation_probability,
					&self.max_runtime,
				)
			})
			.collect::<Result<Vec<Individual<C>>, GeneticError>>()?;

		new_generation.sort();

		self.population = new_generation;

		Ok(())
	}

	/// Selects two individuals to mate
	fn gen_mating_pair(&self, rng: &mut impl Rng) -> (&Individual<C>, &Individual<C>) {
		let index1 = self.gen_tournament_parent(rng);
		let mut index2 = self.gen_tournament_parent(rng);

		while index1 == index2 {
			index2 = self.gen_tournament_parent(rng);
		}

		(&self.population[index1], &self.population[index2])
	}

	fn gen_tournament_parent(&self, rng: &mut impl Rng) -> usize {
		self.mating_dist
			.sample_iter(rng)
			.take(self.tournament_size)
			.min()
			.unwrap_or(0)
	}
}

fn init_population<C>(
	population: &mut Vec<Individual<C>>,
	population_size: usize,
	initial_chromosome: &C,
	max_runtime: &Duration,
) -> Result<(), GeneticError>
where
	C: Chromosome + Send + Sync,
{
	population.clear();
	population.push(initial_chromosome.clone().into());

	let mutated_population = (0..(population_size - 1))
		.into_par_iter()
		.map(|_| {
			let chromosome = init_mutated_chromosome(
				initial_chromosome,
				max_runtime,
			)?;

			Ok(chromosome.into())
		})
		.collect::<Result<Vec<Individual<C>>, GeneticError>>()?;

	population.extend(mutated_population);

	Ok(())
}

fn init_mutated_chromosome<C>(
	chromosome: &C,
	max_runtime: &Duration,
) -> Result<C, GeneticError>
where
	C: Chromosome,
{
	let time = Instant::now();

	let mut rng = SmallRng::from_rng(thread_rng())
		.map_err(|_| GeneticError::Internal)?;

	while time.elapsed().lt(max_runtime) {
		let mut mutated = chromosome.base();

		for index in 0..chromosome.len() {
			let mut gene = chromosome.get(index).clone();

			gene.mutate(&mut rng);
			mutated.push(gene);
		}

		if mutated.is_valid() {
			return Ok(mutated);
		}
	}

	Err(GeneticError::InitialPopulationTimeout)
}

fn init_mating_dist(population_size: usize) -> Uniform<usize> {
	Uniform::from(0..population_size)
}

#[cfg(test)]
mod tests {
	use std::cmp::Ordering;

	use crate::genetic::{
		Genetic,
		Gene,
		Chromosome,
		Rng,
	};

	#[derive(Clone)]
	struct TestData {
		data: u32,
	}

	#[derive(Default, Clone)]
	struct TestConfig {
		config: Vec<TestData>,
	}

	impl Chromosome for TestConfig {
		type Gene = TestData;

		fn base(&self) -> Self {
			TestConfig {
				config: Vec::new(),
			}
		}

		fn is_empty(&self) -> bool {
			self.config.is_empty()
		}

		fn len(&self) -> usize {
			self.config.len()
		}

		fn push(&mut self, data: TestData) {
			self.config.push(data);
		}

		fn get(&self, index: usize) -> &TestData {
			&self.config[index]
		}

		fn clear(&mut self) {
			self.config.clear()
		}

		fn is_optimal(&self) -> bool {
			let sum = self.config
				.iter()
				.map(|item| item.data)
				.sum::<u32>();

			sum == 100
		}
	}

	impl TestConfig {
		fn sum(&self) -> u32 {
			self.config
				.iter()
				.map(|item| item.data)
				.sum::<u32>()
		}
	}

	impl Ord for TestConfig {
		fn cmp(&self, other: &Self) -> Ordering {
			let self_diff = (100 - self.sum() as i32).abs();
			let other_diff = (100 - other.sum() as i32).abs();

			self_diff.cmp(&other_diff)
		}
	}

	impl PartialOrd for TestConfig {
		fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
			Some(self.cmp(other))
		}
	}

	impl PartialEq for TestConfig {
		fn eq(&self, other: &Self) -> bool {
			self.sum() == other.sum()
		}
	}

	impl Eq for TestConfig {}

	impl Gene for TestData {
		fn mutate(&mut self, rng: &mut impl Rng) {
			self.data = rng.gen_range(0..50);
		}
	}

	#[test]
	fn it_optimizes() {
		let mut initial_chromosome = TestConfig::default();

		initial_chromosome.push(TestData { data: 0 });
		initial_chromosome.push(TestData { data: 0 });
		initial_chromosome.push(TestData { data: 0 });
		initial_chromosome.push(TestData { data: 0 });
		initial_chromosome.push(TestData { data: 0 });

		let mut genetic = Genetic::<TestConfig>::new(initial_chromosome).unwrap();

		let result = genetic.run().unwrap();

		assert_ne!(result.generations(), 0);
		assert_eq!(result.chromosome().sum(), 100);
	}
}
