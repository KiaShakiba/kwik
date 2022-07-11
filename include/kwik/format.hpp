/*
 * Copyright (c) Kia Shakiba
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

#ifndef _FORMAT_HPP_
#define _FORMAT_HPP_

#include <string>

namespace kwik {
	namespace format {
		std::string number(const uint64_t &);
		std::string memory(double, const uint8_t & = 0);
		std::string timespan(uint64_t);
	};
};

#endif
