use std::fs::File;
use std::io::{BufReader, Read, Error, ErrorKind};
use crate::file_reader::FileReader;

pub struct BinaryReader<T: Chunk> where [u8; T::SIZE]: Sized {
	file: BufReader<File>,
	buf: [u8; T::SIZE],
	count: u64,
}

pub trait Chunk {
	const SIZE: usize;
	fn new(_: &[u8; Self::SIZE]) -> Result<Self, Error> where Self: Sized;
}

impl<'a, T: Chunk> FileReader<'a> for BinaryReader<T> where [u8; T::SIZE]: Sized {
	fn new(path: &'a str) -> Result<Self, Error> {
		let Ok(opened_file) = File::open(path) else {
			return Err(Error::new(
				ErrorKind::NotFound,
				"Could not open binary file."
			));
		};

		let reader = BinaryReader {
			file: BufReader::new(opened_file),
			buf: [0; T::SIZE],
			count: 0,
		};

		Ok(reader)
	}

	fn size(&self) -> u64 {
		let Ok(metadata) = self.file.get_ref().metadata() else {
			panic!("Could not get binary file's size.");
		};

		metadata.len()
	}
}

impl<T: Chunk> BinaryReader<T> where [u8; T::SIZE]: Sized {
	pub fn read_chunk(&mut self) -> Option<T> {
		match self.file.read_exact(&mut self.buf) {
			Ok(_) => {
				self.count += 1;

				let chunk = match T::new(&self.buf) {
					Ok(chunk) => chunk,
					Err(err) => panic!("Parse error in chunk {}: {:?}", self.count, err),
				};

				Some(chunk)
			},

			Err(ref err) if err.kind() ==  ErrorKind::UnexpectedEof => None,

			Err(_) => {
				panic!("An error occurred when reading binary file.");
			},
		}
	}
}
