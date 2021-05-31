#include <kwik/csv.hpp>
#include <kwik/utils.hpp>

bool kwik::csv::read_row(std::vector<std::string> &row) {
	std::string line;

	bool got = this->read_line(line);

	if (!got) {
		return false;
	}

	row = kwik::utils::split(line, ',');

	return true;
}
