#ifndef _LIST_HPP_
#define _LIST_HPP_

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

		node(T data, struct node *next, struct node *prev) :
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
		list<T>::node *node = this->new_node(data);
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
		list<T>::node *node = this->new_node(data);
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
		this->dislodge(node);

		node->next = this->list_head;
		this->list_head->prev = node;
		this->list_head = node;
	}

	void move_back(list<T>::node *node) {
		this->dislodge(node);

		node->prev = this->list_tail;
		this->list_tail->next = node;
		this->list_tail = node;
	}

	void erase(list<T>::node *node) {
		this->dislodge(node);
		delete node;
		this->list_size--;
	}

	list<T>::node * new_node(T data) {
		list<T>::node *node = new list<T>::node(data, nullptr, nullptr);

		node->data = data;
		node->next = nullptr;
		node->prev = nullptr;

		return node;
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
