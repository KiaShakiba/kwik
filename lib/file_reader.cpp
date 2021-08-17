#include <iostream>
#include <stdexcept>
#include <kwik/file_reader.hpp>
#include <kwik/format.hpp>

kwik::file_reader::file_reader(std::string path, bool show_progress) {
	this->path = path;
	this->quiet = !show_progress;

	this->count_lines();

	this->file_stream = fopen(path.c_str(), "r");

	if (this->file_stream == NULL) {
		throw std::invalid_argument("Could not open input file.");
	}

	if (!this->quiet) {
		this->progress = new kwik::progress(this->num_lines);
	}
}

kwik::file_reader::~file_reader() {
	if (this->file_stream != NULL) {
		fclose(this->file_stream);
	}

	if (!this->quiet) {
		delete this->progress;
	}
}

bool kwik::file_reader::read_line(std::string &line) {
	char *c_str = NULL;
	size_t length = 0;

	int got = getline(&c_str, &length, this->file_stream);

	if (got == -1) {
		fclose(this->file_stream);
		this->file_stream = NULL;
	} else if (!this->quiet) {
		this->progress->tick();
	}

	line = c_str;

	return got != -1;
}

void kwik::file_reader::count_lines() {
	this->num_lines = 0;

	FILE *file_stream = fopen(this->path.c_str(), "r");

	if (file_stream == NULL) {
		throw std::invalid_argument("Could not open input file.");
	}

	char *line = NULL;
	size_t length = 0;

	while (getline(&line, &length, file_stream) != -1) {
		this->num_lines++;

		if (!this->quiet && (this->num_lines % 100000 == 0 || this->num_lines == 1)) {
			std::cout
				<< "Loading file ("
				<< kwik::format::number(this->num_lines) << ")\r"
				<< std::flush;
		}
	}

	fclose(file_stream);

	if (!this->quiet) {
		std::cout
			<< "Loading file ("
			<< kwik::format::number(this->num_lines) << ')'
			<< std::endl;
	}
}
