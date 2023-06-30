use rand::rngs::ThreadRng;

pub trait Gene<T>
where
	Self: Clone,
	T: Clone,
{
	fn value(&self) -> T;
	fn mutate(&mut self, _: &mut ThreadRng);
}
