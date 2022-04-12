/*
 * Copyright (c) Kia Shakiba
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

#include <stdexcept>
#include <kwik/file_writer.hpp>

kwik::file_writer::file_writer(std::string path, std::ios_base::openmode mode) {
	this->file.open(path, mode);

	if (!this->file.is_open()) {
		throw std::invalid_argument("Could not open output file.");
	}
}

kwik::file_writer::~file_writer() {
	this->close();
}

void kwik::file_writer::close() {
	if (this->file.is_open()) {
		this->file.close();
	}
}
