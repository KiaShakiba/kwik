#ifndef _BINARY_TREE_HPP_
#define _BINARY_TREE_HPP_

#include <functional>

namespace kwik {
	template <typename T>
	class binary_tree;
};

template <typename T>
class kwik::binary_tree {
public:
	struct node {
		T data;

		struct node *parent;
		struct node *left;
		struct node *right;

		uint64_t height;

		node(
			T data,
			struct node *parent = nullptr,
			struct node *left = nullptr,
			struct node *right = nullptr
		) : data(data), parent(parent), left(left), right(right), height(1) {}
	};

	typedef std::function<int(T, T)> comparator;

private:
	binary_tree<T>::node *tree_root = nullptr;
	size_t tree_size = 0;

	binary_tree<T>::comparator compare = [](T a, T b) {
		return
			a < b ? -1 :
			a > b ? 1 :
			0;
	};

public:
	binary_tree(binary_tree<T>::comparator compare = nullptr) {
		if (compare != nullptr) this->compare = compare;
	}

	~binary_tree() {
		this->destroy(this->tree_root);
	}

	void insert(T data) {
		binary_tree<T>::node *node = binary_tree<T>::new_node(data);
		this->insert(node);
	}

	void insert(binary_tree<T>::node *node) {
		this->tree_size++;
		this->tree_root = insert(this->tree_root, node);
	}

	void remove(T data) {
		binary_tree<T>::node *node = this->find(this->tree_root, data);
		this->remove(node);
	}

	void remove(binary_tree<T>::node *node) {
		if (node == nullptr) return;

		this->tree_size--;

		binary_tree<T>::node *promote = nullptr;

		if (node->left != nullptr && node->right != nullptr) {
			promote = node->left->height > node->right->height ?
				this->find_max(node->left) : this->find_min(node->right);
		} else if (node->left != nullptr || node->right != nullptr) {
			promote = node->left == nullptr ? node->right : node->left;
		}

		if (promote != nullptr) promote->parent = node->parent;
		if (node->left != promote) promote->left = node->left;
		if (node->right != promote) promote->right = node->right;

		this->parent_unlink(promote);
		this->parent_relink(node, promote);
		this->parent_refresh_height(promote != nullptr ? promote : node->parent);

		if (this->tree_root == node) this->tree_root = promote;

		delete node;
	}

	void print() {
		std::cout << "kwik::binary_tree[" << this->tree_size << "]<";
		this->print(this->tree_root, this->find_min(this->tree_root));
		std::cout << '>' << std::endl;
	}

	static binary_tree<T>::node * new_node(T data) {
		return new binary_tree<T>::node(data);
	}

private:
	binary_tree<T>::node * insert(binary_tree<T>::node *root, binary_tree<T>::node *node) {
		if (root == nullptr) return node;

		int cmp = this->compare(root->data, node->data);

		if (cmp == 0) {
			this->tree_size--;
			delete node;
		}

		node->parent = root;

		if (cmp > 0) {
			if (root->left != nullptr) node->parent = root->left;
			root->left = this->insert(root->left, node);
		}

		if (cmp < 0) {
			if (root->right != nullptr) node->parent = root->right;
			root->right = this->insert(root->right, node);
		}

		uint64_t left_height = root->left != nullptr ? root->left->height : 0;
		uint64_t right_height = root->right != nullptr ? root->right->height : 0;

		root->height = std::max(left_height, right_height) + 1;

		return root;
	}

	void parent_relink(binary_tree<T>::node *root, binary_tree<T>::node *node) {
		if (root == nullptr || root->parent == nullptr) return;
		if (root->parent->left == root) root->parent->left = node;
		if (root->parent->right == root) root->parent->right = node;
	}

	void parent_unlink(binary_tree<T>::node *node) {
		this->parent_relink(node, nullptr);
	}

	void parent_refresh_height(binary_tree<T>::node *node) {
		if (node == nullptr) return;

		uint64_t left_height = node->left != nullptr ? node->left->height : 0;
		uint64_t right_height = node->right != nullptr ? node->right->height : 0;

		node->height = std::max(left_height, right_height) + 1;

		if (node->parent != nullptr) this->parent_refresh_height(node->parent);
	}

	binary_tree<T>::node * find(binary_tree<T>::node *root, T data) {
		if (root == nullptr) return nullptr;

		int cmp = this->compare(root->data, data);

		if (cmp == 0) return root;
		if (cmp > 0) return this->find(root->left, data);
		return this->find(root->right, data);
	}

	binary_tree<T>::node * find_min(binary_tree<T>::node *node) {
		if (node == nullptr) return nullptr;
		if (node->left == nullptr) return node;

		return this->find_min(node->left);
	}

	binary_tree<T>::node * find_max(binary_tree<T>::node *node) {
		if (node == nullptr) return nullptr;
		if (node->right == nullptr) return node;

		return this->find_max(node->right);
	}

	void destroy(binary_tree<T>::node *root) {
		if (root == nullptr) return;

		this->destroy(root->left);
		this->destroy(root->right);

		delete root;
	}

	void print(binary_tree<T>::node *root, binary_tree<T>::node *no_comma = nullptr) {
		if (root == nullptr) return;
		if (root->left != nullptr) this->print(root->left, no_comma);
		if (no_comma != root) std::cout << ", ";

		std::cout << root->data;

		if (root->right != nullptr) this->print(root->right, no_comma);
	}
};

#endif
