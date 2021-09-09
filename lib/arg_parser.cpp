#include <iostream>
#include <stdexcept>
#include <kwik/arg_parser.hpp>

kwik::arg_parser::arg_parser(int argc, char **argv) {
	this->argc = argc;
	this->argv = argv;
}

void kwik::arg_parser::add(
	std::string short_tag,
	std::string long_tag,
	bool required,
	std::string default_value
) {
	if (this->get_entry(short_tag) != nullptr) this->throw_already_registered(short_tag);
	if (this->get_entry(long_tag) != nullptr) this->throw_already_registered(long_tag);

	this->entries.push_back(new kwik::arg_parser::entry(
		long_tag,
		short_tag,
		required,
		default_value
	));
}

void kwik::arg_parser::parse() {
	for (int i = 1; i < argc; i++) {
		std::string key = argv[i];
		bool is_long = key.find("--") == 0;
		bool is_short = !is_long && key.find("-") == 0;

		if (!is_long && !is_short) this->throw_invalid(key);

		std::string tag = key;
		tag.replace(0, is_long ? 2 : 1, "");

		auto entry = this->get_entry(tag);

		if (entry == nullptr) this->throw_invalid(tag);

		entry->exists = true;
		entry->arg_value = i < argc - 1 && argv[i + 1][0] != '-' ?  argv[++i] : "";
	}

	for (auto entry : this->entries) {
		if (entry->required && !entry->exists) {
			throw std::invalid_argument("Missing required arg <" + entry->long_tag + ">");
		}
	}
}

bool kwik::arg_parser::has(const std::string &tag) const {
	auto entry = this->get_entry(tag);
	if (entry == nullptr) this->throw_not_registered(tag);
	return entry->exists;
}

kwik::arg_parser::entry * kwik::arg_parser::get_entry(const std::string &tag) const {
	for (auto entry : this->entries) {
		if (entry->long_tag == tag || entry->short_tag == tag) {
			return entry;
		}
	}

	return nullptr;
}

void kwik::arg_parser::throw_invalid(const std::string &tag) const {
	throw std::invalid_argument("Invalid arg <" + tag + ">");
}

void kwik::arg_parser::throw_already_registered(const std::string &tag) const {
	throw std::invalid_argument("Arg already registered <" + tag + ">");
}

void kwik::arg_parser::throw_not_registered(const std::string &tag) const {
	throw std::invalid_argument("Arg not registered <" + tag + ">");
}
