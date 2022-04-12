/*
 * Copyright (c) Kia Shakiba
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

#include <kwik/csv_reader.hpp>
#include <kwik/utils.hpp>

bool kwik::csv_reader::read_row(std::vector<std::string> &row) {
	std::string line;

	bool got = kwik::file_reader::read_line(line);

	if (!got) {
		return false;
	}

	row = kwik::utils::split(line, ",");

	return true;
}
