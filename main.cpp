#include <iostream>
#include <kwik.hpp>

int main(int argc, char **argv) {
	kwik::avl_tree<int> tree;
	for (int i=0; i<12; i++) tree.insert(i);
	std::cout << "height:\t" << tree.height() << std::endl;
	std::cout << "root:\t" << tree.root()->data << std::endl;
	tree.print();

	return 0;
}
