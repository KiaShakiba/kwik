/*
 * Copyright (c) Kia Shakiba
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

#ifndef _BINARY_READER_HPP_
#define _BINARY_READER_HPP_

#include <stdexcept>
#include <kwik/file_reader.hpp>

#include <iostream>

namespace kwik {
	class binary_reader;
};

class kwik::binary_reader : public kwik::file_reader {
private:
	using kwik::file_reader::read_line;

	uint64_t bytes_read = 0;

public:
	struct chunk {
	private:
		typedef char byte_t;

		size_t total_size;
		size_t current = 0;
		byte_t *buf = nullptr;

	public:
		chunk(size_t size) : total_size(size) {
			if (size == 0) throw std::invalid_argument("Invalid chunk size");
			this->buf = new byte_t[size];
		}

		~chunk() {
			delete [] this->buf;
		}

		size_t size() const {
			return this->total_size;
		}

		byte_t * buffer() const {
			return this->buf;
		}

		void reset() {
			this->current = 0;
		}

		template <typename T>
		T get() {
			size_t type_size = sizeof(T);

			if (type_size + this->current > this->total_size) {
				throw std::invalid_argument("Type size exceeds remaining chunk size");
			}

			T value = *(T *)(this->buf + this->current);
			this->current += type_size;

			return value;
		}
	};

	binary_reader(std::string path, bool show_progress = false) :
		kwik::file_reader(path, std::ifstream::in | std::ifstream::binary, show_progress) {}

	bool read_chunk(kwik::binary_reader::chunk &);
};

#endif
