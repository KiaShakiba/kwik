#ifndef _UTILS_HPP_
#define _UTILS_HPP_

#include <string>
#include <vector>

namespace kwik {
	namespace utils {
		std::vector<std::string> split(std::string const &, char);
		uint64_t timestamp();

		template <typename T>
		T cast(std::string);
	}
};

#endif
