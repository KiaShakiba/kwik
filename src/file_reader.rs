use std::io::Error;

pub trait FileReader<'a> {
	fn new(_: &'a str) -> Result<Self, Error> where Self: Sized;
	fn size(&self) -> u64;
}
