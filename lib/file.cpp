#include <iostream>
#include <file.hpp>
#include <format.hpp>

kwik::file::file(std::string path) {
	this->path = path;
	this->count_lines();

	this->file_stream = fopen(path.c_str(), "r");

	if (this->file_stream == NULL) {
		throw std::invalid_argument("Could not open input file.");
	}

	this->progress = new kwik::progress(this->num_lines);
}

kwik::file::~file() {
	if (this->file_stream != NULL) {
		fclose(this->file_stream);
	}

	delete this->progress;
}

bool kwik::file::read_line(std::string &line) {
	char *c_str = NULL;
	size_t length = 0;

	int got = getline(&c_str, &length, this->file_stream);

	if (got == -1) {
		fclose(this->file_stream);
		this->file_stream = NULL;
	} else {
		this->progress->tick();
	}

	line = c_str;

	return got != -1;
}

void kwik::file::count_lines() {
	this->num_lines = 0;

	FILE *file_stream = fopen(this->path.c_str(), "r");

	if (file_stream == NULL) {
		throw std::invalid_argument("Could not open input file.");
	}

	char *line = NULL;
	size_t length = 0;

	while (getline(&line, &length, file_stream) != -1) {
		this->num_lines++;

		if (this->num_lines % 100000 == 0 || this->num_lines == 1) {
			std::cout
				<< "Loading file ("
				<< kwik::format::number(this->num_lines) << ")\r"
				<< std::flush;
		}
	}

	fclose(file_stream);

	std::cout
		<< "Loading file ("
		<< kwik::format::number(this->num_lines) << ')'
		<< std::endl;
}
