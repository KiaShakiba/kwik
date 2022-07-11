/*
 * Copyright (c) Kia Shakiba
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

#include <iomanip>
#include <locale>
#include <array>
#include <sstream>
#include <kwik/format.hpp>

std::string kwik::format::number(const uint64_t &value) {
	std::stringstream ss;
	ss.imbue(std::locale(""));
	ss << std::fixed << value;
	return ss.str();
}

std::string kwik::format::memory(double value, const uint8_t &precision) {
	std::array<const char *, 6> names = { "B", "KB", "MB", "GB", "TB", "PB" };
	uint32_t count = 0;

	while ((uint64_t)value / 1024 > 0) {
		value /= 1024;
		count++;
	}

	std::stringstream ss;

	ss
		<< std::fixed
		<< std::setprecision(precision)
		<< value
		<< ' ' << names[count];

	return ss.str();
}

std::string kwik::format::timespan(uint64_t milliseconds) {
	uint64_t days = milliseconds / 1000 / 60 / 60 / 24;
	milliseconds -= days * 1000 * 60 * 60 * 24;

	uint64_t hours = milliseconds / 1000 / 60 / 60;
	milliseconds -= hours * 1000 * 60 * 60;

	uint64_t minutes = milliseconds / 1000 / 60;
	milliseconds -= minutes * 1000 * 60;

	uint64_t seconds = milliseconds / 1000;
	milliseconds -= seconds * 1000;

	bool started = false;
	std::stringstream ss;

	ss << std::setfill('0');

	if (days > 0) {
		ss << days << '.';
		started = true;
	}

	if (started || hours > 0) {
		ss << std::setw(started ? 2 : 0) << hours << ':';
		started = true;
	}

	if (started || minutes > 0) {
		ss << std::setw(started ? 2 : 0) << minutes << ':';
		started = true;
	}

	if (started || seconds > 0) {
		ss << std::setw(started ? 2 : 0) << seconds << '.';
		started = true;
	}

	ss << std::setw(started ? 3 : 0) << milliseconds;

	return ss.str();
}
