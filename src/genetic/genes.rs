use crate::genetic::Fitness;
use crate::genetic::gene::Gene;

pub trait Genes: Clone {
	fn new() -> Self;

	fn push<T: Gene>(&mut self, _: T);
	fn get<T: Gene>(&self, _: usize) -> &T;

	fn fitness(&self) -> Fitness;
}
