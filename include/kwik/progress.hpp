/*
 * Copyright (c) Kia Shakiba
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

#ifndef _PROGRESS_HPP_
#define _PROGRESS_HPP_

namespace kwik {
	class progress;
};

class kwik::progress {
private:
	const uint64_t total;
	uint64_t current = 0;

	uint64_t rate_time = 0;
	uint64_t rate_count = 0;
	uint64_t previous_rate = 0;

	const uint64_t WIDTH = 70;

	const char FILLED = '=';
	const char CURRENT = '>';
	const char REMAINING = ' ';

public:
	progress(uint64_t);

	void tick(const uint64_t & = 1);
	void set(const uint64_t &);

private:
	uint64_t get_rate();
	void draw(const uint64_t &, const uint64_t &);
};

#endif
