#include <chrono>
#include <utils.hpp>

std::vector<std::string> kwik::utils::split(std::string input, char delimiter) {
	std::vector<std::string> values;

	std::string value;

	// loop through every character in the input
	for (int i = 0; i < input.length(); i++) {
		// if the character is not the delimiter or a new line,
		// add it to the current value
		// otherwise, if there is a current value, add it to the values
		if (input[i] != delimiter && input[i] != '\n') {
			value += input[i];
		} else if (!value.empty()) {
			values.push_back(value);
			value = "";
		}
	}

	// handle the last value that is not followed by a delimiter
	if (!value.empty()) {
		values.push_back(value);
	}

	return values;
}

uint64_t kwik::utils::timestamp() {
	return std::chrono::duration_cast<std::chrono::milliseconds>(
		std::chrono::system_clock::now().time_since_epoch()
	).count();
}
