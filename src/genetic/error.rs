use thiserror::Error;

#[derive(Debug, Error)]
pub enum GeneticError {
	#[error("could not create valid offspring")]
	MateTimeout,
}
