/*
 * Copyright (c) Kia Shakiba
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

use crate::genetic::{chromosome::Chromosome, individual::Individual};

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
