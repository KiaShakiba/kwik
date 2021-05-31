#ifndef _THREAD_POOL_HPP_
#define _THREAD_POOL_HPP_

#include <thread>
#include <vector>
#include <queue>
#include <functional>
#include <mutex>
#include <condition_variable>
#include <atomic>

namespace kwik {
	class thread_pool;
};

class kwik::thread_pool {
private:
	std::vector<std::thread> threads;
	std::queue<std::function<void()>> work;

	std::mutex pool_lock;
	std::condition_variable pool_condition;
	std::condition_variable wait_condition;

	std::atomic<int> num_running = 0;
	std::atomic<bool> stop_work = false;

public:
	thread_pool(size_t = std::thread::hardware_concurrency());

	void add(std::function<void()>);
	void wait();
	void stop();

private:
	void worker(int);
};

#endif
