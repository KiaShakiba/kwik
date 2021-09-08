#include <cstring>
#include <chrono>
#include <stdexcept>
#include <kwik/utils.hpp>

std::vector<std::string> kwik::utils::split(std::string const &input, const char *delimiter) {
	std::vector<std::string> values;

	char *save_ptr;
	char *token;

	for (token = strtok_r((char *)input.c_str(), delimiter, &save_ptr);
		token != NULL;
		token = strtok_r(NULL, delimiter, &save_ptr)) {

		values.push_back(std::string(token));
	}

	return values;
}

uint64_t kwik::utils::timestamp() {
	return std::chrono::duration_cast<std::chrono::milliseconds>(
		std::chrono::system_clock::now().time_since_epoch()
	).count();
}

template <typename T>
T kwik::utils::cast(std::string value) {
	throw std::invalid_argument("Type not supported.");
}

template <>
int kwik::utils::cast<int>(std::string value) {
	return std::stoi(value);
}

template <>
float kwik::utils::cast<float>(std::string value) {
	return std::stof(value);
}

template <>
double kwik::utils::cast<double>(std::string value) {
	return std::stod(value);
}

template <>
uint32_t kwik::utils::cast<uint32_t>(std::string value) {
	return std::stoul(value);
}

template <>
uint64_t kwik::utils::cast<uint64_t>(std::string value) {
	return std::stoull(value);
}

template <>
std::string kwik::utils::cast<std::string>(std::string value) {
	return value;
}
