#ifndef _LIST_HPP_
#define _LIST_HPP_

#include <iostream>
#include <string>

namespace kwik {
	template <typename T>
	class list;
};

template <typename T>
class kwik::list {
public:
	struct node {
		T data;

		struct node *next;
		struct node *prev;

		node(T data, struct node *next = nullptr, struct node *prev = nullptr) :
			data(data), next(next), prev(prev) {}
	};

private:
	list<T>::node *list_head = nullptr;
	list<T>::node *list_tail = nullptr;

	size_t list_size = 0;

public:
	~list() {
		list<T>::node *node = this->list_head;

		while (node != nullptr) {
			list<T>::node *next = node->next;
			delete node;
			node = next;
		}
	}

	list<T>::node * head() { return this->list_head; }
	list<T>::node * tail() { return this->list_tail; }
	size_t size() { return this->list_size; }

	void push_front(T data) {
		list<T>::node *node = list<T>::new_node(data);
		this->push_front(node);
	}

	void push_front(list<T>::node *node) {
		if (this->list_size == 0) {
			this->list_head = this->list_tail = node;
		} else {
			node->next = this->list_head;
			this->list_head->prev = node;
			this->list_head = node;
		}

		this->list_size++;
	}

	void push_back(T data) {
		list<T>::node *node = list<T>::new_node(data);
		this->push_back(node);
	}

	void push_back(list<T>::node *node) {
		if (this->list_size == 0) {
			this->list_head = this->list_tail = node;
		} else {
			node->prev = this->list_tail;
			this->list_tail->next = node;
			this->list_tail = node;
		}

		this->list_size++;
	}

	void move_front(list<T>::node *node) {
		if (this->list_head == node) return;

		this->dislodge(node);

		node->next = this->list_head;
		this->list_head->prev = node;
		this->list_head = node;
	}

	void move_back(list<T>::node *node) {
		if (this->list_tail == node) return;

		this->dislodge(node);

		node->prev = this->list_tail;
		this->list_tail->next = node;
		this->list_tail = node;
	}

	void place_before(list<T>::node *node, list<T>::node *new_node) {
		if (new_node->next == nullptr && new_node->prev == nullptr) this->list_size++;
		if (node->prev != nullptr) node->prev->next = new_node;

		this->dislodge(new_node);

		new_node->next = node;
		new_node->prev = node->prev;

		node->prev = new_node;

		if (this->list_head == node) this->list_head = new_node;
	}

	void place_after(list<T>::node *node, list<T>::node *new_node) {
		if (new_node->next == nullptr && new_node->prev == nullptr) this->list_size++;
		if (node->next != nullptr) node->next->prev = new_node;

		this->dislodge(new_node);

		new_node->next = node->next;
		new_node->prev = node;

		node->next = new_node;

		if (this->list_tail == node) this->list_tail = new_node;
	}

	void erase(list<T>::node *node) {
		this->dislodge(node);
		delete node;
		this->list_size--;
	}

	void print() {
		list<T>::node *node = this->list_head;

		std::cout << "kwik::list[" << this->list_size << "]<";

		while (node != nullptr) {
			std::cout << node->data;

			if (node->next != nullptr) {
				std::cout << ", ";
			}

			node = node->next;
		}

		std::cout << '>' << std::endl;
	}

	static list<T>::node * new_node(T data) {
		return new list<T>::node(data);
	}

private:
	void dislodge(list<T>::node *node) {
		if (this->list_head == node) this->list_head = node->next;
		if (this->list_tail == node) this->list_tail = node->prev;

		if (node->next != nullptr) node->next->prev = node->prev;
		if (node->prev != nullptr) node->prev->next = node->next;

		node->next = nullptr;
		node->prev = nullptr;
	}
};

#endif
