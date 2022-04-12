/*
 * Copyright (c) Kia Shakiba
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

#ifndef _BINARY_WRITER_HPP_
#define _BINARY_WRITER_HPP_

#include <kwik/file_writer.hpp>

namespace kwik {
	class binary_writer;
};

class kwik::binary_writer : public kwik::file_writer {
public:
	binary_writer(std::string path) :
		kwik::file_writer(path, std::ofstream::out | std::ofstream::binary) {}

	template <typename T>
	binary_writer & operator<<(T value) {
		this->file.write((const char *) &value, sizeof(T));
		return *this;
	}
};

#endif
