use crate::genetic::Fitness;
pub use crate::genetic::gene::Gene;

/// This defines a set of genes. With this, genes can be added and
/// retrieved. The overall fitness of the genes can also be computed.
///
/// # Examples
/// ```
/// struct MyData {
///     data: u32,
/// }
///
/// struct MyConfig {
///     config: Vec<MyData>,
/// }
///
/// impl Genes<u32, MyData> for MyConfig {
///     fn new(&self) -> Self {
///         MyConfig {
///             config: Vec::new(),
///         }
///     }
///
///     fn is_empty(&self) -> bool {
///         self.config.is_empty()
///
///     fn len(&self) -> usize {
///         self.config.len()
///     }
///
///     fn push(&mut self, data: MyData) -> usize {
///         self.config.push(data);
///     }
///
///     fn get(&self, index: usize) -> &MyData {
///         &self.config[index]
///     }
///
///     fn fitness(&self) -> Fitness {
///         self.config
///             .iter()
///             .map(|item| item.data as Fitness)
///             .sum::<Fitness>()
///     }
/// }
/// ```
pub trait Genes<T, G>
where
	Self: Clone,
	T: Clone,
	G: Gene<T>,
{
	/// Creates a new, empty instance of genes.
	fn new() -> Self;

	/// Returns true if there are no genes.
	fn is_empty(&self) -> bool;

	/// Returns the number of genes.
	fn len(&self) -> usize;

	/// Adds a gene to the genes.
	fn push(&mut self, _: G);

	/// Retrieves a reference to a gene from the genes.
	fn get(&self, _: usize) -> &G;

	/// Computes the overall fitness of the genes. The genetic algorithm
	/// will attempt to get this fitness as close to 0 as possible.
	fn fitness(&self) -> Fitness;
}
