#include <iostream>
#include <kwik.hpp>

void calc() {
	for (int i=0, c=0; i<1000000; i++) c = i * 4;
}

int main(int argc, char **argv) {
	kwik::thread_pool pool;

	for (int i=0; i<1000; i++) {
		pool.add([]() { calc(); });
	}

	pool.wait();
	pool.stop();
	return 0;
}
