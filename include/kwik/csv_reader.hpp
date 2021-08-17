#ifndef _CSV_HPP_
#define _CSV_HPP_

#include <string>
#include <vector>
#include <kwik/file_reader.hpp>

namespace kwik {
	class csv_reader;
};

class kwik::csv_reader : public kwik::file_reader {
using kwik::file_reader::file_reader;

public:
	bool read_line(std::vector<std::string> &);
};

#endif
