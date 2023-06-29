pub trait Gene: Clone {
	fn mutate(&mut self);
}
