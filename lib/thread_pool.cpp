#include <thread_pool.hpp>

#include <iostream>

kwik::thread_pool::thread_pool(size_t size) {
	if (size == 0) {
		throw std::invalid_argument("Thread pool size cannot be 0.");
	}

	for (size_t i = 0; i < size; i++) {
		this->threads.push_back(std::thread(&kwik::thread_pool::worker, this, i));
	}
}

void kwik::thread_pool::add(std::function<void()> job) {
	std::unique_lock<std::mutex> lock (this->pool_lock);
	this->work.push(job);
	lock.unlock();

	this->pool_condition.notify_one();
}

void kwik::thread_pool::wait() {
	std::unique_lock<std::mutex> lock (this->pool_lock);

	this->wait_condition.wait(lock, [this]() {
		return this->num_running == 0 && this->work.empty();
	});
}

void kwik::thread_pool::stop() {
	this->stop_work = true;
	this->pool_condition.notify_all();

	for (size_t i = 0; i < this->threads.size(); i++) {
		this->threads[i].join();
	}
}

void kwik::thread_pool::worker(int id) {
	std::function<void()> job;

	while (true) {
		{
			std::unique_lock<std::mutex> lock (this->pool_lock);

			this->pool_condition.wait(lock, [this]() {
				return this->stop_work || !this->work.empty();
			});

			if (this->stop_work) return;

			this->num_running++;

			job = this->work.front();
			this->work.pop();
		}

		job();

		this->num_running--;
		this->wait_condition.notify_all();
	}
}
