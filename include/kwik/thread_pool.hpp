#ifndef _THREAD_POOL_HPP_
#define _THREAD_POOL_HPP_

#include <thread>
#include <vector>
#include <queue>
#include <unordered_map>
#include <functional>
#include <mutex>
#include <condition_variable>
#include <atomic>

namespace kwik {
	class thread_pool;
};

class kwik::thread_pool {
public:
	typedef uint32_t job_id_t;

private:
	struct job {
		const job_id_t id;
		const std::function<void()> task;
		bool complete = false;

		job(job_id_t id, std::function<void()> task) :
			id(id), task(task) {}

		void run() {
			this->task();
			this->complete = true;
		}
	};

	std::vector<std::thread> threads;

	std::queue<job *> job_queue;
	std::unordered_map<job_id_t, job *> job_map;

	std::mutex pool_lock;
	std::condition_variable pool_condition;
	std::condition_variable wait_condition;

	std::atomic<int> num_running = 0;
	std::atomic<bool> stop_work = false;

	job_id_t num_jobs = 0;

public:
	thread_pool(size_t = std::thread::hardware_concurrency());

	job_id_t add(std::function<void()>);

	void wait();
	void wait(job_id_t);

	void stop();

private:
	void worker(int);
};

#endif
