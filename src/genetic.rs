mod individual;
mod gene;
mod genes;

use rand::{thread_rng, Rng};
use crate::genetic::individual::Individual;
use crate::genetic::genes::Genes;

const POPULATION_SIZE: usize = 100;
const CONVERGENCE_SIZE: u32 = 1_000;
const MUTATION_PROBABILITY: f64 = 0.1;
const ELITE_RATIO: f64 = 0.1;
const MATING_RATIO: f64 = 0.5;

pub type Fitness = u64;

/// Finds the optimal values for a set of inputs using a genetic algorithm.
pub struct Genetic<T: Genes> {
	population: Vec<Individual<T>>,

	target_fitness: Fitness,
	generation_count: usize,
}

impl<T: Genes> Genetic<T> {
	/// Creates an instance of the genetic runner using the supplied genes as initial values.
	///
	/// The runner will attempt to find the genes that will give the closest value to the supplied
	/// target fitness.
	pub fn new(
		initial_genes: Box<T>,
		target_fitness: Fitness
	) -> Self {
		let mut population = Vec::<Individual<T>>::new();

		for _ in 0..POPULATION_SIZE {
			population.push(Individual::new(initial_genes.clone()));
		}

		Genetic {
			population,

			target_fitness,
			generation_count: 0,
		}
	}

	pub fn run(&mut self) -> &Individual<T> {
		let mut last_fitness = self.iterate();
		let mut convergence_count: u32 = 0;

		while last_fitness != 0 && convergence_count < CONVERGENCE_SIZE {
			let fitness = self.iterate();

			if fitness != last_fitness {
				last_fitness = fitness;
				convergence_count = 0;
			} else {
				convergence_count += 1;
			}
		}

		&self.population[0]
	}

	fn iterate(&mut self) -> Fitness {
		self.generation_count += 1;

		let elite_population = (POPULATION_SIZE as f64 * ELITE_RATIO) as usize;
		let mating_population = (POPULATION_SIZE as f64 * MATING_RATIO) as usize;

		let mut new_generation = self.population
			.iter()
			.take(elite_population)
			.cloned()
			.collect::<Vec::<Individual<T>>>();

		let mut rng = thread_rng();

		for _ in 0..(POPULATION_SIZE - elite_population) {
			let index1: usize = rng.gen_range(0..mating_population);
			let mut index2: usize = rng.gen_range(0..mating_population);

			while index1 == index2 {
				index2 = rng.gen_range(0..mating_population);
			}

			let parent1 = &self.population[index1];
			let parent2 = &self.population[index2];
			let child = parent1.mate(&parent2);

			new_generation.push(child);
		}

		new_generation.sort();

		self.population = new_generation;
		self.population[0].fitness()
	}
}
