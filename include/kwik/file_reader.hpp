/*
 * Copyright (c) Kia Shakiba
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

#ifndef _FILE_READER_HPP_
#define _FILE_READER_HPP_

#include <string>
#include <fstream>
#include <kwik/progress.hpp>

namespace kwik {
	class file_reader;
};

class kwik::file_reader {
protected:
	bool quiet;
	std::ifstream file;
	kwik::progress *progress = nullptr;

	uint64_t total_size;

	file_reader(std::string, std::ios_base::openmode, bool = false);

public:
	file_reader(std::string path, bool show_progress = false) :
		file_reader(path, std::ifstream::in, show_progress) {}

	~file_reader();

	bool read_line(std::string &);

	void close();

private:
	uint64_t get_total_size();
};

#endif
