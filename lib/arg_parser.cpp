#include <iostream>
#include <stdexcept>
#include <kwik/arg_parser.hpp>

kwik::arg_parser::arg_parser(int argc, char **argv) {
	this->argc = argc;
	this->argv = argv;
}

void kwik::arg_parser::add(std::string short_tag, std::string long_tag, bool required) {
	this->long_map[long_tag] = short_tag;
	this->short_map[short_tag] = long_tag;
	this->required[long_tag] = required;
}

void kwik::arg_parser::parse() {
	for (int i = 1; i < argc; i++) {
		std::string key = argv[i];
		bool is_long = key.find("--") == 0;
		bool is_short = !is_long && key.find("-") == 0;

		if (!is_long && !is_short) {
			throw std::invalid_argument("Invalid arg <" + key + ">");
		}

		std::string tag = key;
		tag.replace(0, is_long ? 2 : 1, "");

		if (is_long && this->long_map.find(tag) == this->long_map.end() ||
			is_short && this->short_map.find(tag) == this->short_map.end()) {

			throw std::invalid_argument("Invalid arg <" + tag + ">");
		}

		this->args[key] = i < argc - 1 && argv[i + 1][0] != '-' ?
			argv[++i] : "";
	}

	for (auto &it : this->required) {
		if (it.second && this->args.find("--" + it.first) == this->args.end() &&
			this->args.find("-" + this->long_map[it.first]) == this->args.end()) {

			throw std::invalid_argument("Missing required arg <" + it.first + ">");
		}
	}
}

bool kwik::arg_parser::has(const std::string &tag) {
	return this->args.find("--" + tag) != this->args.end() ||
		this->args.find("-" + this->long_map[tag]) != this->args.end() ||
		this->args.find("-" + tag) != this->args.end() ||
		this->args.find("--" + this->short_map[tag]) != this->args.end();
}

std::string kwik::arg_parser::get_value(const std::string &tag) {
	auto got_long = this->long_map.find(tag);
	auto got_short = this->short_map.find(tag);

	bool has_long = got_long != this->long_map.end();
	bool has_short = got_short != this->short_map.end();

	if (!has_long && !has_short) {
		throw std::invalid_argument("Invalid arg <" + tag + ">");
	}

	std::unordered_map<std::string, std::string>::const_iterator got;

	if (has_long) {
		got = this->args.find("--" + tag);

		if (got != this->args.end()) {
			return got->second;
		}

		got = this->args.find("-" + got_long->second);

		if (got != this->args.end()) {
			return got->second;
		}
	}

	got = this->args.find("-" + tag);

	if (got != this->args.end()) {
		return got->second;
	}

	got = this->args.find("--" + got_short->second);

	if (got == this->args.end()) {
		throw std::invalid_argument("Arg not found <" + tag + ">");
	}

	return got->second;
}
