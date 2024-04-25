/*
 * Copyright (c) Kia Shakiba
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

use thiserror::Error;

#[derive(Debug, Error)]
pub enum GeneticError {
	#[error("invalid initial chromosome")]
	InvalidInitialChromosome,

	#[error("could not create valid offspring")]
	MateTimeout,
}
