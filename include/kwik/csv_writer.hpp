#ifndef _CSV_WRITER_HPP_
#define _CSV_WRITER_HPP_

#include <sstream>
#include <kwik/file_writer.hpp>

namespace kwik {
	class csv_writer;
};

class kwik::csv_writer : public kwik::file_writer {
using kwik::file_writer::file_writer;

private:
	std::stringstream line;
	struct endl_t {};

public:
	static const endl_t endl;

	template <typename T>
	csv_writer & operator<<(T value) {
		this->line << value << ',';
		return *this;
	}

	csv_writer & operator<<(endl_t);
};

#endif
