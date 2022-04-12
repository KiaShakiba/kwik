/*
 * Copyright (c) Kia Shakiba
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

#ifndef _ARG_PARSER_HPP_
#define _ARG_PARSER_HPP_

#include <string>
#include <sstream>
#include <unordered_map>
#include <set>
#include <kwik/utils.hpp>

namespace kwik {
	class arg_parser;
};

class kwik::arg_parser {
private:
	struct entry {
		std::string long_tag;
		std::string short_tag;
		bool required;
		bool exists;
		std::string arg_value;
		std::string default_value;

		entry(
			std::string long_tag,
			std::string short_tag,
			bool required,
			std::string default_value
		) : long_tag(long_tag), short_tag(short_tag),
			required(required), exists(false), arg_value(""),
			default_value(default_value) {}

		std::string value() {
			if (this->exists) {
				return this->arg_value;
			}

			return this->default_value;
		}
	};

	int argc;
	char **argv;

	std::unordered_map<std::string, std::string> args;
	std::vector<entry *> entries;

public:
	arg_parser(int, char **);
	~arg_parser();

	void parse();

	bool has(const std::string &) const;

	template <typename T = std::string>
	void add(
		std::string short_tag,
		std::string long_tag,
		bool required = false,
		T default_value = ""
	) {
		if (this->get_entry(short_tag) != nullptr) {
			kwik::arg_parser::throw_already_registered(short_tag);
		}

		if (this->get_entry(long_tag) != nullptr) {
			kwik::arg_parser::throw_already_registered(long_tag);
		}

		std::stringstream ss;
		ss << default_value;

		this->entries.push_back(new kwik::arg_parser::entry(
			long_tag,
			short_tag,
			required,
			ss.str()
		));
	}

	template <typename T = std::string>
	T get(const std::string &tag) const {
		auto entry = this->get_entry(tag);
		if (entry == nullptr) this->throw_not_registered(tag);
		return kwik::utils::cast<T>(entry->value());
	}

private:
	entry * get_entry(const std::string &) const;

	static void throw_invalid(const std::string &);
	static void throw_already_registered(const std::string &);
	static void throw_not_registered(const std::string &);
};

#endif
