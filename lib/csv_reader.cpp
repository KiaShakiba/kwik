#include <kwik/csv_reader.hpp>
#include <kwik/utils.hpp>

bool kwik::csv_reader::read_line(std::vector<std::string> &row) {
	std::string line;

	bool got = kwik::file_reader::read_line(line);

	if (!got) {
		return false;
	}

	row = kwik::utils::split(line, ",");

	return true;
}
