/*
 * Copyright (c) Kia Shakiba
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

#include <kwik/csv_writer.hpp>

kwik::csv_writer & kwik::csv_writer::operator<<(endl_t) {
	std::string line_value = this->line.str();

	if (line_value.length()) {
		line_value = line_value.substr(0, line_value.length() - 1);
		this->line.str("");
	}

	(kwik::file_writer &) (*this) << line_value << kwik::file_writer::endl;

	return *this;
}
