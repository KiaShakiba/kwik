#ifndef _FILE_HPP_
#define _FILE_HPP_

#include <string>
#include <fstream>
#include <kwik/progress.hpp>

namespace kwik {
	class file_reader;
};

class kwik::file_reader {
private:
	std::string path;
	bool quiet;

	uint64_t num_lines;

	FILE *file_stream;
	kwik::progress *progress = nullptr;

public:
	file_reader(std::string, bool = false);
	~file_reader();

	bool read_line(std::string &);

private:
	void count_lines();
};

#endif
