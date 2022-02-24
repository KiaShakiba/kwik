#ifndef _CSV_READER_HPP_
#define _CSV_READER_HPP_

#include <string>
#include <vector>
#include <kwik/file_reader.hpp>

namespace kwik {
	class csv_reader;
};

class kwik::csv_reader : public kwik::file_reader {
using kwik::file_reader::file_reader;

private:
	using kwik::file_reader::read_line;

public:
	bool read_row(std::vector<std::string> &);
};

#endif
