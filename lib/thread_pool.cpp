#include <iostream>
#include <stdexcept>
#include <kwik/thread_pool.hpp>

kwik::thread_pool::thread_pool(size_t size) {
	if (size == 0) {
		throw std::invalid_argument("Thread pool size cannot be 0.");
	}

	for (size_t i = 0; i < size; i++) {
		this->threads.push_back(std::thread(&kwik::thread_pool::worker, this, i));
	}
}

kwik::thread_pool::job_id_t kwik::thread_pool::add(std::function<void()> task) {
	kwik::thread_pool::job *job = new kwik::thread_pool::job(
		this->num_jobs++,
		task
	);

	std::unique_lock<std::mutex> lock (this->pool_lock);
	this->job_queue.push(job);
	this->job_map.emplace(job->id, job);
	lock.unlock();

	this->pool_condition.notify_one();

	return job->id;
}

void kwik::thread_pool::wait() {
	std::unique_lock<std::mutex> lock (this->pool_lock);

	this->wait_condition.wait(lock, [this]() {
		return this->num_running == 0 && this->job_queue.empty();
	});
}

void kwik::thread_pool::wait(kwik::thread_pool::job_id_t job_id) {
	std::unique_lock<std::mutex> lock (this->pool_lock);

	auto got = this->job_map.find(job_id);
	if (got == this->job_map.end()) throw std::invalid_argument("Invalid job id.");

	auto job = got->second;
	if (job->complete) return;

	this->wait_condition.wait(lock, [&job]() {
		return job->complete;
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
	kwik::thread_pool::job *job;

	while (true) {
		{
			std::unique_lock<std::mutex> lock (this->pool_lock);

			this->pool_condition.wait(lock, [this]() {
				return this->stop_work || !this->job_queue.empty();
			});

			if (this->stop_work) return;

			this->num_running++;

			job = this->job_queue.front();
			this->job_queue.pop();
		}

		job->run();

		{
			std::unique_lock<std::mutex> lock (this->pool_lock);
			this->num_running--;
		}

		this->wait_condition.notify_all();
	}
}
