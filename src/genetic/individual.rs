use std::cmp::{Ord, Ordering};
use rand::{thread_rng, Rng};
use crate::genetic::Fitness;
use crate::genetic::genes::Genes;

#[derive(Clone)]
pub struct Individual<T: Genes> {
	genes: Box<T>,
}

impl<T: Genes> Individual<T> {
	pub fn new(genes: Box<T>) -> Self {
		Individual {
			genes,
		}
	}

	pub fn fitness(&self) -> Fitness {
		self.genes.fitness()
	}

	pub fn mate(&self, partner: &Individual<T>) -> Individual<T> {
		let mut rng = thread_rng();

		todo!();
	}
}

impl<T: Genes> Ord for Individual<T> {
	fn cmp(&self, other: &Self) -> Ordering {
		self.partial_cmp(other).unwrap()
	}
}

impl<T: Genes> PartialOrd for Individual<T> {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		other.fitness().partial_cmp(&self.fitness())
	}
}

impl<T: Genes> PartialEq for Individual<T> {
	fn eq(&self, other: &Self) -> bool {
		self.fitness() == other.fitness()
	}
}

impl<T: Genes> Eq for Individual<T> {}
