#include <iomanip>
#include <locale>
#include <kwik/format.hpp>

std::string kwik::format::number(uint64_t const &value) {
	std::stringstream ss;
	ss.imbue(std::locale(""));
	ss << std::fixed << value;
	return ss.str();
}
