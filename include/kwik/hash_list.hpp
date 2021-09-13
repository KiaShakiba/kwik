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
		this->map.emplace(key, node);
		kwik::list<V>::push_front(node);
	}

	void push_front(K key, kwik::hash_list<K, V>::node *node) {
		this->map.emplace(key, node);
		kwik::list<V>::push_front(node);
	}

	void push_back(K key, V data) {
		hash_list<K, V>::node *node = hash_list<K, V>::new_node(data);
		this->map.emplace(key, node);
		kwik::list<V>::push_back(node);
	}

	void push_back(K key, kwik::hash_list<K, V>::node *node) {
		this->map.emplace(key, node);
		kwik::list<V>::push_back(node);
	}

	void erase(K key) {
		auto got = this->map.find(key);

		if (got != this->map.end()) {
			this->map.erase(key);
			kwik::list<V>::erase(got->second);
		}
	}
};

#endif
