#ifndef _ARG_PARSER_HPP_
#define _ARG_PARSER_HPP_

#include <string>
#include <unordered_map>
#include <set>
#include <utils.hpp>

namespace kwik {
	class arg_parser;
};

class kwik::arg_parser {
private:
	int argc;
	char **argv;

	std::unordered_map<std::string, std::string> args;

	std::unordered_map<std::string, std::string> short_map;
	std::unordered_map<std::string, std::string> long_map;
	std::unordered_map<std::string, bool> required;

public:
	arg_parser(int, char **);

	void add(std::string, std::string, bool = false);
	void parse();

	bool has(const std::string &);

	template <typename T = std::string>
	T get(const std::string &tag) {
		return kwik::utils::cast<T>(this->get_value(tag));
	}

private:
	std::string get_value(const std::string &);
};

#endif
