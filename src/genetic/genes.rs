use crate::genetic::Fitness;
pub use crate::genetic::gene::Gene;

pub trait Genes<T, G>
where
	Self: Clone,
	T: Clone,
	G: Gene<T>,
{
	fn new() -> Self;

	fn len(&self) -> usize;
	fn push(&mut self, _: G);
	fn get(&self, _: usize) -> &G;

	fn fitness(&self) -> Fitness;
}
