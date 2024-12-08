use crate::genetic::{
	individual::Individual,
	chromosome::Chromosome,
};

pub struct Offspring<C>
where
	C: Chromosome,
{
	individual: Individual<C>,
	mutations: u64,
}

impl<C> Offspring<C>
where
	C: Chromosome,
{
	pub fn new(individual: Individual<C>, mutations: u64) -> Self {
		Offspring {
			individual,
			mutations,
		}
	}

	pub fn mutations(&self) -> u64 {
		self.mutations
	}

	pub fn into_individual(self) -> Individual<C> {
		self.individual
	}
}
