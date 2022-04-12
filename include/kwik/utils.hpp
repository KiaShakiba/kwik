/*
 * Copyright (c) Kia Shakiba
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

#ifndef _UTILS_HPP_
#define _UTILS_HPP_

#include <string>
#include <vector>
#include <random>
#include <stdexcept>

namespace kwik {
	namespace utils {
		std::vector<std::string> split(std::string const &, const char *);
		uint64_t timestamp();

		template <typename T>
		T cast(std::string);

		template <typename T = double>
		T random(T min = 0.0, T max = 1.0) {
			if (min > max) {
				throw std::invalid_argument("Min must be less than max");
			}

			std::random_device rd;
			std::mt19937 gen(rd());
			std::uniform_real_distribution<double> dis (0.0, 1.0);

			return dis(gen) * (max - min) + min;
		}
	};
};

#endif
