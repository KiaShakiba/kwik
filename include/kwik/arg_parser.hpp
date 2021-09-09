#ifndef _ARG_PARSER_HPP_
#define _ARG_PARSER_HPP_

#include <string>
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

	void add(std::string, std::string, bool = false, std::string = "");
	void parse();

	bool has(const std::string &) const;

	template <typename T>
	void add(
		std::string short_tag,
		std::string long_tag,
		bool required,
		T default_value
	) {
		this->add(short_tag, long_tag, required, std::to_string(default_value));
	}

	template <typename T = std::string>
	T get(const std::string &tag) const {
		auto entry = this->get_entry(tag);
		if (entry == nullptr) this->throw_not_registered(tag);
		return kwik::utils::cast<T>(entry->value());
	}

private:
	entry * get_entry(const std::string &) const;

	void throw_invalid(const std::string &) const;
	void throw_already_registered(const std::string &) const;
	void throw_not_registered(const std::string &) const;
};

#endif
