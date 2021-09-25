#ifndef _FILE_READER_HPP_
#define _FILE_READER_HPP_

#include <string>
#include <fstream>
#include <kwik/progress.hpp>

namespace kwik {
	class file_reader;
};

class kwik::file_reader {
private:
	bool quiet;
	std::ifstream file;
	kwik::progress *progress = nullptr;

protected:
	file_reader(std::string, std::ios_base::openmode, bool = false);

public:
	file_reader(std::string path, bool show_progress = false) :
		file_reader(path, std::ifstream::in, show_progress) {}

	~file_reader();

	bool read_line(std::string &);

private:
	uint64_t get_total_size();
};

#endif
