#include <cstring>
#include <chrono>
#include <utils.hpp>

std::vector<std::string> kwik::utils::split(std::string const &input, char delimiter) {
	std::vector<std::string> values;

	char *save_ptr;
	char *token;

	for (token = strtok_r((char *)input.c_str(), &delimiter, &save_ptr);
		token != NULL;
		token = strtok_r(NULL, &delimiter, &save_ptr)) {

		values.push_back(std::string(token));
	}

	return values;
}

uint64_t kwik::utils::timestamp() {
	return std::chrono::duration_cast<std::chrono::milliseconds>(
		std::chrono::system_clock::now().time_since_epoch()
	).count();
}
