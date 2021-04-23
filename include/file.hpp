#ifndef _FILE_HPP_
#define _FILE_HPP_

#include <string>
#include <fstream>
#include <progress.hpp>

namespace kwik {
	class file;
};

class kwik::file {
private:
	std::string path;
	uint64_t num_lines;

	FILE *file_stream;
	kwik::progress *progress;

public:
	file(std::string);
	~file();

	bool read_line(std::string &);

private:
	void count_lines();
};

#endif
