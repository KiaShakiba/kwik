/*
 * Copyright (c) Kia Shakiba
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

#include <kwik/binary_reader.hpp>

bool kwik::binary_reader::read_chunk(kwik::binary_reader::chunk &chunk) {
	if (this->bytes_read == this->total_size) {
		return false;
	}

	if (this->bytes_read + chunk.size() > this->total_size) {
		throw std::invalid_argument("Chunk size exceeds remaining file size");
	}

	chunk.reset();
	this->file.read(chunk.buffer(), chunk.size());
	this->bytes_read += chunk.size();

	if (this->bytes_read == this->total_size) {
		this->file.close();
	}

	if (!this->quiet) {
		this->progress->tick(chunk.size());
	}

	return true;
}
