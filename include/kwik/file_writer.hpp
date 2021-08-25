#ifndef _FILE_WRITER_HPP_
#define _FILE_WRITER_HPP_

#include <string>
#include <fstream>

namespace kwik {
	class file_writer;
};

class kwik::file_writer {
protected:
	std::ofstream file;

public:
	static const char endl = '\n';

	file_writer(std::string);
	~file_writer();

	template <typename T>
	file_writer & operator<<(T value) {
		this->file << value;
		return *this;
	}
};

#endif
