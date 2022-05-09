/*
 * Copyright (c) Kia Shakiba
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

#ifndef _HASH_LIST_HPP_
#define _HASH_LIST_HPP_

#include <unordered_map>
#include <kwik/list.hpp>

namespace kwik {
	template <typename K, typename V>
	class hash_list;
};

template <typename K, typename V>
class kwik::hash_list : public kwik::list<V> {
using kwik::list<V>::list;

public:
	using node = typename kwik::list<V>::node;

private:
	std::unordered_map<K, kwik::hash_list<K, V>::node *> map;

public:
	kwik::hash_list<K, V>::node * get(K key) {
		auto got = this->map.find(key);
		if (got == this->map.end()) return nullptr;
		return got->second;
	}

	void push_front(K key, V data) {
		hash_list<K, V>::node *node = hash_list<K, V>::new_node(data);
		this->emplace_node(key, node);
		kwik::list<V>::push_front(node);
	}

	void push_front(K key, kwik::hash_list<K, V>::node *node) {
		this->emplace_node(key, node);
		kwik::list<V>::push_front(node);
	}

	void push_back(K key, V data) {
		hash_list<K, V>::node *node = hash_list<K, V>::new_node(data);
		this->emplace_node(key, node);
		kwik::list<V>::push_back(node);
	}

	void push_back(K key, kwik::hash_list<K, V>::node *node) {
		this->emplace_node(key, node);
		kwik::list<V>::push_back(node);
	}

	void place_before(kwik::hash_list<K, V>::node *node, K new_key, kwik::hash_list<K, V>::node *new_node) {
		kwik::hash_list<K, V>::node *got_node = this->get(new_key);

		if (got_node != nullptr && got_node != new_node) throw std::invalid_argument("Invalid <key, node> pair");
		if (got_node == nullptr) this->emplace_node(new_key, new_node);

		kwik::list<V>::place_before(node, new_node);
	}

	void place_after(kwik::hash_list<K, V>::node *node, K new_key, kwik::hash_list<K, V>::node *new_node) {
		kwik::hash_list<K, V>::node *got_node = this->get(new_key);

		if (got_node != nullptr && got_node != new_node) throw std::invalid_argument("Invalid <key, node> pair");
		if (got_node == nullptr) this->emplace_node(new_key, new_node);

		kwik::list<V>::place_after(node, new_node);
	}

	void move_before(K key, K new_key) {
		kwik::hash_list<K, V>::node *node = this->get(key);
		if (node == nullptr) throw std::invalid_argument("Invalid key");

		kwik::hash_list<K, V>::node *new_node = this->get(new_key);
		if (new_node == nullptr) throw std::invalid_argument("Invalid key");

		kwik::list<V>::place_before(node, new_node);
	}

	void move_after(K key, K new_key) {
		kwik::hash_list<K, V>::node *node = this->get(key);
		if (node == nullptr) throw std::invalid_argument("Invalid key");

		kwik::hash_list<K, V>::node *new_node = this->get(new_key);
		if (new_node == nullptr) throw std::invalid_argument("Invalid key");

		kwik::list<V>::place_after(node, new_node);
	}

	void erase(K key) {
		auto got = this->map.find(key);

		if (got != this->map.end()) {
			this->map.erase(key);
			kwik::list<V>::erase(got->second);
		}
	}

	void erase(K key, kwik::hash_list<K, V>::node *node) {
		this->map.erase(key);
		kwik::list<V>::erase(node);
	}

private:
	void emplace_node(K key, kwik::hash_list<K, V>::node *node) {
		auto placed = this->map.emplace(key, node);

		if (!placed.second) {
			throw std::invalid_argument("Key already exists in hash list");
		}
	}
};

#endif
