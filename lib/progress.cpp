#include <iostream>
#include <cmath>
#include <progress.hpp>
#include <utils.hpp>
#include <format.hpp>

kwik::progress::progress(uint64_t total) {
	this->total = total;
	this->draw(0, 0);
}

void kwik::progress::tick() {
	this->current++;

	uint64_t progress = (uint64_t)(100 * (double)this->current / this->total);
	uint64_t previous_progress = (uint64_t)(100 * (double)(this->current - 1) / this->total);

	uint64_t rate = this->get_rate();

	// if the progress and rate have not changed since the last
	// update and the progress is not complete, do nothing
	if (progress == previous_progress &&
		rate == previous_rate &&
		progress != 100) {

		return;
	}

	this->draw(progress, rate);

	this->previous_rate = rate;

	// if the bar is complete, go to a new line
	if (progress == 100) {
		std::cout << std::endl;
	}
}

uint64_t kwik::progress::get_rate() {
	this->rate_count++;

	uint64_t now = kwik::utils::timestamp();

	// if more than 1 second has passed since the last update,
	// return the number of updates in the last second
	if (now - this->rate_time >= 1000) {
		double rate = this->rate_count / ((now - this->rate_time) / (double)1000);

		this->rate_time = now;
		this->rate_count = 0;

		return round(rate);
	}

	// if 1 second has not yet passed since the last update,
	// return the previous rate
	return this->previous_rate;
}

void kwik::progress::draw(uint64_t progress, uint64_t rate) {
	int position = kwik::progress::WIDTH * ((double)progress / 100);

	std::cout << '[';

	for (int i = 0; i<kwik::progress::WIDTH; i++) {
		std::cout << (
			i < position ? kwik::progress::FILLED :
			i == position ? kwik::progress::CURRENT :
			kwik::progress::REMAINING
		);
	}

	std::cout << "] " << progress << " %";

	// if the current progress is 100, do not show a rate
	std::string rate_string = progress < 100 ?
		" (" + kwik::format::number(rate) + " rps)" : "";

	std::cout << rate_string;

	std::string previous_rate_string = " (" + kwik::format::number(this->previous_rate) + " rps)";

	// if the current rate is less than the previous rate,
	// cover the old characters with spaces to hide them
	if (previous_rate_string.size() > rate_string.size()) {
		for (int i=0; i<previous_rate_string.size() - rate_string.size(); i++) {
			std::cout << ' ';
		}
	}

	std::cout << '\r' << std::flush;
}
