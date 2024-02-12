/*
 * Copyright (c) Kia Shakiba
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

mod worker;

use std::sync::{mpsc, Arc, Mutex};
use crate::thread_pool::worker::{Worker, Job};

pub struct ThreadPool {
	workers: Vec<Worker>,
	sender: Option<mpsc::Sender<Job>>,
}

/// A thread pool implementaton that creates a number of threads
/// and executes distributed jobs amongst them.
impl ThreadPool {
	/// Creates a new instance of a thread pool with the specified
	/// number of threads.
	///
	/// # Examples
	/// ```
	/// use kwik::ThreadPool;
	///
	/// // create a thread pool with 4 threads
	/// let pool = ThreadPool::new(4);
	/// ```
	pub fn new(size: usize) -> ThreadPool {
		let mut workers = Vec::<Worker>::new();
		let (sender, receiver) = mpsc::channel();
		let receiver = Arc::new(Mutex::new(receiver));

		for _ in 0..size {
			workers.push(Worker::new(
				Arc::clone(&receiver)
			));
		}

		ThreadPool {
			workers,
			sender: Some(sender),
		}
	}

	/// Executes a job in one of the thread pool's worker threads.
	///
	/// Examples
	/// ```
	/// use kwik::ThreadPool;
	///
	/// let pool = ThreadPool::new(4);
	///
	/// pool.execute(|| {
	///     // do work here
	/// });
	/// ```
	#[inline]
	pub fn execute<F>(&self, f: F)
	where
		F: 'static + FnOnce() + Send,
	{
		let job = Box::new(f);

		self.sender
			.as_ref().unwrap()
			.send(job).unwrap();
	}
}

impl Drop for ThreadPool {
	fn drop(&mut self) {
		drop(self.sender.take());

		for worker in &mut self.workers {
			if let Some(thread) = worker.thread.take() {
				thread.join().unwrap();
			}
		}
	}
}
