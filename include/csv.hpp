#ifndef _CSV_HPP_
#define _CSV_HPP_

#include <string>
#include <vector>
#include <file.hpp>

namespace kwik {
	class csv;
};

class kwik::csv : public kwik::file {
using kwik::file::file;

public:
	bool read_row(std::vector<std::string> &);
};

#endif
