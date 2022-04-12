/*
 * Copyright (c) Kia Shakiba
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

#include <iostream>
#include <stdexcept>
#include <kwik/file_reader.hpp>
#include <kwik/format.hpp>

kwik::file_reader::file_reader(std::string path, std::ios_base::openmode mode, bool show_progress) {
	this->quiet = !show_progress;

	this->file.open(path, mode);

	if (!this->file.is_open()) {
		throw std::invalid_argument("Could not open input file.");
	}

	this->total_size = this->get_total_size();

	if (!this->quiet) {
		this->progress = new kwik::progress(total_size);
	}
}

kwik::file_reader::~file_reader() {
	this->close();

	if (!this->quiet) {
		delete this->progress;
	}
}

bool kwik::file_reader::read_line(std::string &line) {
	std::istream &got = getline(this->file, line);

	if (!got) {
		this->file.close();
	} else if (!this->quiet) {
		this->progress->tick(line.size() + sizeof(char));
	}

	return !!got;
}

uint64_t kwik::file_reader::get_total_size() {
	this->file.seekg(0, this->file.end);
	uint64_t size = this->file.tellg();
	this->file.seekg(0, this->file.beg);

	return size;
}

void kwik::file_reader::close() {
	if (this->file.is_open()) {
		this->file.close();
	}
}
