use thiserror::Error;

#[derive(Debug, Error)]
pub enum GeneticError {
	#[error("invalid initial genes")]
	InvalidInitialGenes,

	#[error("could not create valid offspring")]
	MateTimeout,
}
