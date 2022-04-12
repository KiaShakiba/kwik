/*
 * Copyright (c) Kia Shakiba
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

#include <iostream>
#include <cmath>
#include <kwik/progress.hpp>
#include <kwik/utils.hpp>
#include <kwik/format.hpp>

kwik::progress::progress(uint64_t total) {
	this->total = total;
	this->draw(0, 0);
}

void kwik::progress::tick(uint64_t amount) {
	this->set(this->current + amount);
}

void kwik::progress::set(uint64_t value) {
	uint64_t previous = this->current;
	this->current = value;

	uint64_t progress = (uint64_t)(100 * (double)this->current / this->total);
	uint64_t previous_progress = (uint64_t)(100 * (double)previous / this->total);

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

	std::cout << "\33[2K\r[";

	for (int i = 0; i<kwik::progress::WIDTH; i++) {
		std::cout << (
			i < position ? kwik::progress::FILLED :
			i == position ? kwik::progress::CURRENT :
			kwik::progress::REMAINING
		);
	}

	std::cout << "] " << progress << " %";

	// if the current progress is 100, do not show a rate
	if (progress < 100) {
		std::cout << " (" + kwik::format::number(rate) + " rps)";
	}

	std::cout << '\r' << std::flush;
}
