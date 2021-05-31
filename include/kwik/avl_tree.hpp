#ifndef _AVL_TREE_HPP_
#define _AVL_TREE_HPP_

#include <iostream>
#include <kwik/binary_tree.hpp>

namespace kwik {
	template <typename T>
	class avl_tree;
};

template <typename T>
class kwik::avl_tree : public kwik::binary_tree<T> {
using kwik::binary_tree<T>::binary_tree;

private:
	using node = typename kwik::binary_tree<T>::node;

private:
	int64_t difference(avl_tree<T>::node *node) {
		return node != nullptr ? this->height(node->left) - this->height(node->right) : 0;
	}

	avl_tree<T>::node * balance(avl_tree<T>::node *root) {
		int64_t factor = this->difference(root);

		if (factor > 1) {
			return this->difference(root->left) > 0 ?
				this->ll_rotate(root) :
				this->lr_rotate(root);
		}

		if (factor < -1) {
			return this->difference(root->right) > 0 ?
				this->rl_rotate(root) :
				this->rr_rotate(root);
		}

		return root;
	}

	avl_tree<T>::node * rr_rotate(avl_tree<T>::node *root) {
		avl_tree<T>::node *parent = root->right;
		root->right = parent->left;
		parent->left = root;

		root->height = std::max(
			this->height(root->left),
			this->height(root->right)
		) + 1;

		parent->height = std::max(
			this->height(parent->left),
			this->height(parent->right)
		) + 1;

		return parent;
	}

	avl_tree<T>::node * ll_rotate(avl_tree<T>::node *root) {
		avl_tree<T>::node *parent = root->left;
		root->left = parent->right;
		parent->right = root;

		root->height = std::max(
			this->height(root->left),
			this->height(root->right)
		) + 1;

		parent->height = std::max(
			this->height(parent->left),
			this->height(parent->right)
		) + 1;

		return parent;
	}

	avl_tree<T>::node * lr_rotate(avl_tree<T>::node *root) {
		root->left = rr_rotate(root->left);
		return ll_rotate(root);
	}

	avl_tree<T>::node * rl_rotate(avl_tree<T>::node *root) {
		root->right = ll_rotate(root->right);
		return rr_rotate(root);
	}
};

#endif
