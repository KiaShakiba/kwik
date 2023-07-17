use std::sync::{mpsc, Arc, Mutex};
use std::thread;

pub struct Worker {
	pub thread: Option<thread::JoinHandle<()>>,
}

pub type Job = Box<dyn 'static + FnOnce() + Send>;

impl Worker {
	pub fn new(receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
		let thread = thread::spawn(move || loop {
			let job = receiver
				.lock().unwrap()
				.recv();

			match job {
				Ok(job) => job(),
				Err(_) => break,
			}
		});

		Worker {
			thread: Some(thread),
		}
	}
}
