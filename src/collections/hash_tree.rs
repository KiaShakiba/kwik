use std::{
	borrow::Borrow,
	cmp::{self, Ordering},
	ptr::{self, NonNull},
	hash::{Hash, Hasher, BuildHasher, RandomState},
	collections::HashMap,
};

/// A hash AVL tree.
pub struct HashTree<T, S = RandomState> {
	map: HashMap<DataRef<T>, NonNull<Entry<T>>, S>,
	root: *mut Entry<T>,
}

struct Entry<T> {
	data: NonNull<T>,

	left: *mut Entry<T>,
	right: *mut Entry<T>,

	height: usize,
}

struct DataRef<T> {
	data: *const T,
}

#[repr(transparent)]
struct KeyWrapper<K>(K);

impl<T, S> HashTree<T, S>
where
	T: Eq + Ord + Hash,
	S: BuildHasher,
{
	/// Inserts an entry into the hash tree.
	///
	/// If the hash tree did not have this entry, `None` is returned.
	///
	/// If the hash tree did have this entry, the new entry is inserted
	/// and the old entry is returned.
	///
	/// # Examples
	/// ```
	/// use kwik::collections::HashTree;
	///
	/// let mut tree = HashTree::<u64>::default();
	///
	/// assert_eq!(tree.insert(1), None);
	/// assert_eq!(tree.insert(2), None);
	/// assert_eq!(tree.insert(3), None);
	/// assert_eq!(tree.insert(2), Some(2));
	/// ```
	pub fn insert(&mut self, data: T) -> Option<T> {
		let maybe_old_entry = self.map
			.remove(&DataRef { data: &data })
			.map(|old_entry| {
				let old_entry_ptr = old_entry.as_ptr();

				unsafe {
					(*old_entry_ptr).data.read()
				}
			});

		let entry = Entry::new(data);
		let entry_ptr = entry.as_ptr();

		let new_root = insert_entry(self.root, entry);
		self.root = new_root.as_ptr();

		let data_ptr = unsafe {
			(*entry_ptr).data.as_ptr()
		};

		let data_ref = DataRef {
			data: data_ptr,
		};

		self.map.insert(data_ref, entry);

		maybe_old_entry
	}
}

impl<T, S> HashTree<T, S> {
	/// Creates a new hash tree with the supplied hasher.
	///
	/// # Examples
	/// ```
	/// use std::hash::RandomState;
	/// use kwik::collections::HashTree;
	///
	/// let s = RandomState::new();
	/// let tree = HashTree::<u64, RandomState>::with_hasher(s);
	/// ```
	pub fn with_hasher(hasher: S) -> Self {
		HashTree {
			map: HashMap::with_hasher(hasher),
			root: ptr::null_mut(),
		}
	}

	/// Returns `true` if the hash tree contains no entries.
	///
	/// # Examples
	/// ```
	/// use kwik::collections::HashTree;
	///
	/// let tree = HashTree::<u64>::default();
	/// assert!(tree.is_empty());
	/// ```
	pub fn is_empty(&self) -> bool {
		self.map.is_empty()
	}

	/// Returns the number of entries in the hash tree.
	///
	/// # Examples
	/// ```
	/// use kwik::collections::HashTree;
	///
	/// let tree = HashTree::<u64>::default();
	/// assert_eq!(tree.len(), 0);
	/// ```
	pub fn len(&self) -> usize {
		self.map.len()
	}
}

impl<T> HashTree<T, RandomState> {
	/// Creates a new hash tree.
	///
	/// # Examples
	/// ```
	/// use kwik::collections::HashTree;
	///
	/// let tree = HashTree::<u64>::new();
	/// ```
	pub fn new() -> Self {
		HashTree::with_hasher(RandomState::new())
	}
}

impl<T, S> Default for HashTree<T, S>
where
	S: Default,
{
	fn default() -> Self {
		HashTree::<T, S>::with_hasher(S::default())
	}
}

impl<T> Entry<T> {
	fn new(data: T) -> NonNull<Self> {
		let data_ptr = init_data_ptr(data);
		Entry::<T>::from_data_ptr(data_ptr)
	}

	fn from_data_ptr(data_ptr: NonNull<T>) -> NonNull<Self> {
		let entry = Entry {
			data: data_ptr,

			left: ptr::null_mut(),
			right: ptr::null_mut(),

			height: 0,
		};

		let boxed = Box::new(entry);

		unsafe {
			NonNull::new_unchecked(Box::into_raw(boxed))
		}
	}

	fn set_left(&mut self, left: *mut Entry<T>) {
		self.left = left;
		self.refresh_height();
	}

	fn set_right(&mut self, right: *mut Entry<T>) {
		self.right = right;
		self.refresh_height();
	}

	fn refresh_height(&mut self) {
		let left_height = if !self.left.is_null() {
			unsafe {
				(*self.left).height
			}
		} else {
			0
		};

		let right_height = if !self.right.is_null() {
			unsafe {
				(*self.right).height
			}
		} else {
			0
		};

		self.height = cmp::max(left_height, right_height) + 1;
	}
}

fn init_data_ptr<T>(data: T) -> NonNull<T> {
	let boxed_data = Box::new(data);

	unsafe {
		NonNull::new_unchecked(Box::into_raw(boxed_data))
	}
}

/// inserts a new entry into the tree, returning the root
fn insert_entry<T>(
	root: *mut Entry<T>,
	mut entry: NonNull<Entry<T>>,
) -> NonNull<Entry<T>>
where
	T: Ord,
{
	if root.is_null() {
		return entry;
	}

	let cmp = unsafe {
		let entry_ref = entry.as_ref();
		(*root).data.as_ref().cmp(entry_ref.data.as_ref())
	};

	match cmp {
		Ordering::Less => unsafe {
			let new_left = insert_entry((*root).left, entry);
			(*root).set_left(new_left.as_ptr());

			let balanced = balance_entry(root);
			NonNull::new(balanced).unwrap()
		},

		Ordering::Greater => unsafe {
			let new_right = insert_entry((*root).right, entry);
			(*root).set_right(new_right.as_ptr());

			let balanced = balance_entry(root);
			NonNull::new(balanced).unwrap()
		},

		Ordering::Equal => unsafe {
			let new_left = (*root).left;
			let new_right = (*root).right;

			entry.as_mut().set_left(new_left);
			entry.as_mut().set_right(new_right);

			entry
		},
	}
}

fn balance_entry<T>(entry: *mut Entry<T>) -> *mut Entry<T> {
	println!("\n***\nbalancing");
	let factor = balance_factor(entry);
	println!("factor: {factor}");

	if factor > 1 {
		let left_factor = unsafe {
			balance_factor((*entry).left)
		};

		println!("left_factor: {left_factor}");

		if left_factor > 0 {
			return ll_rotate(entry);
		} else {
			return lr_rotate(entry);
		};
	}

	if factor < -1 {
		let right_factor = unsafe {
			balance_factor((*entry).right)
		};

		if right_factor > 0 {
			return rl_rotate(entry);
		} else {
			return rr_rotate(entry);
		}
	}

	entry
}

fn balance_factor<T>(entry: *mut Entry<T>) -> i64 {
	if entry.is_null() {
		return 0;
	}

	let left = unsafe {
		(*entry).left
	};

	let right = unsafe {
		(*entry).right
	};

	let left_height = if !left.is_null() {
		unsafe {
			(*left).height
		}
	} else {
		0
	};

	let right_height = if !right.is_null() {
		unsafe {
			(*right).height
		}
	} else {
		0
	};

	left_height as i64 - right_height as i64
}

fn rr_rotate<T>(old_root: *mut Entry<T>) -> *mut Entry<T> {
	if old_root.is_null() {
		return old_root;
	}

	unsafe {
		let new_root = (*old_root).right;

		if new_root.is_null() {
			return old_root;
		}

		(*old_root).right = (*new_root).left;
		(*new_root).left = old_root;

		(*old_root).refresh_height();
		(*new_root).refresh_height();

		new_root
	}
}

fn ll_rotate<T>(old_root: *mut Entry<T>) -> *mut Entry<T> {
	if old_root.is_null() {
		return old_root;
	}

	unsafe {
		let new_root = (*old_root).left;

		if new_root.is_null() {
			return old_root;
		}

		(*old_root).left = (*new_root).right;
		(*new_root).right = old_root;

		(*old_root).refresh_height();
		(*new_root).refresh_height();

		new_root
	}
}

fn lr_rotate<T>(old_root: *mut Entry<T>) -> *mut Entry<T> {
	if old_root.is_null() {
		return old_root;
	}

	unsafe {
		(*old_root).left = rr_rotate((*old_root).left);
		ll_rotate(old_root)
	}
}

fn rl_rotate<T>(old_root: *mut Entry<T>) -> *mut Entry<T> {
	if old_root.is_null() {
		return old_root;
	}

	unsafe {
		(*old_root).right = ll_rotate((*old_root).right);
		rr_rotate(old_root)
	}
}

impl<T> DataRef<T> {
	fn new(entry_ptr: *mut Entry<T>) -> Self {
		let data_ptr = unsafe {
			(*entry_ptr).data.as_ptr()
		};

		DataRef {
			data: data_ptr,
		}
	}
}

impl<T> Hash for DataRef<T>
where
	T: Hash,
{
	fn hash<H>(&self, state: &mut H)
	where
		H: Hasher,
	{
		unsafe {
			(*self.data).hash(state)
		}
	}
}

impl<T> PartialEq for DataRef<T>
where
	T: PartialEq,
{
	fn eq(&self, other: &Self) -> bool {
		unsafe {
			(*self.data).eq(&*other.data)
		}
	}
}

impl<T> Eq for DataRef<T>
where
	T: Eq,
{}

impl<K> KeyWrapper<K> {
	fn from_ref(key: &K) -> &Self {
		unsafe {
			&*(key as *const K as *const KeyWrapper<K>)
		}
	}
}

impl<K> Hash for KeyWrapper<K>
where
	K: Hash,
{
	fn hash<H>(&self, state: &mut H)
	where
		H: Hasher,
	{
		self.0.hash(state)
	}
}

impl<K> PartialEq for KeyWrapper<K>
where
	K: PartialEq,
{
	fn eq(&self, other: &Self) -> bool {
		self.0.eq(&other.0)
	}
}

impl<K> Eq for KeyWrapper<K>
where
	K: Eq,
{}

impl<K, T> Borrow<KeyWrapper<K>> for DataRef<T>
where
	T: Borrow<K>,
{
	fn borrow(&self) -> &KeyWrapper<K> {
		let data_ref = unsafe {
			&*self.data
		}.borrow();

		KeyWrapper::from_ref(data_ref)
	}
}

#[cfg(test)]
mod tests {
	use crate::collections::hash_tree::{HashTree, Entry};

	#[test]
	fn it_inserts_correctly() {
		let mut tree = HashTree::<u64>::default();

		assert_eq!(tree.insert(1), None);
		assert_eq!(tree.insert(2), None);
		assert_eq!(tree.insert(3), None);
		assert_eq!(tree.insert(4), None);
		assert_eq!(tree.insert(5), None);
		assert_eq!(tree.insert(1), Some(1));

		assert!(!tree.root.is_null());

		let (root_data, root_height) = get_entry_info(tree.root);
		assert_eq!(root_data, 2);
		assert_eq!(root_height, 3);

		let (left, right) = get_entry_children(tree.root);
		assert!(!left.is_null());
		assert!(!right.is_null());

		let (left_data, left_height) = get_entry_info(left);
		assert_eq!(left_data, 1);
		assert_eq!(left_height, 1);

		let (right_data, right_height) = get_entry_info(right);
		assert_eq!(right_data, 4);
		assert_eq!(right_height, 2);
	}

	#[test]
	fn it_balances_null_root() {
		use std::ptr;
		use crate::collections::hash_tree::balance_entry;

		balance_entry::<u64>(ptr::null_mut());
	}

	#[test]
	fn it_calculates_balance_factor_correctly() {
		use crate::collections::hash_tree::balance_factor;

		let entry_one = Entry::new(1).as_ptr();
		let entry_two = Entry::new(2).as_ptr();
		let entry_three = Entry::new(3).as_ptr();

		unsafe {
			(*entry_one).right = entry_two;
			(*entry_two).right = entry_three;

			(*entry_one).height = 3;
			(*entry_two).height = 2;
			(*entry_three).height = 1;
		}

		let factor = balance_factor(entry_one);
		assert_eq!(factor, -2);
	}

	#[test]
	fn it_rr_rotates_null_root() {
		use std::ptr;
		use crate::collections::hash_tree::rr_rotate;

		rr_rotate::<u64>(ptr::null_mut());
	}

	#[test]
	fn it_rr_rotates_null_children() {
		use crate::collections::hash_tree::rr_rotate;

		let entry = Entry::new(0).as_ptr();
		rr_rotate(entry);
	}

	#[test]
	fn it_ll_rotates_null_root() {
		use std::ptr;
		use crate::collections::hash_tree::ll_rotate;

		ll_rotate::<u64>(ptr::null_mut());
	}

	#[test]
	fn it_ll_rotates_null_children() {
		use crate::collections::hash_tree::ll_rotate;

		let entry = Entry::new(0).as_ptr();
		ll_rotate(entry);
	}

	#[test]
	fn it_lr_rotates_null_root() {
		use std::ptr;
		use crate::collections::hash_tree::lr_rotate;

		lr_rotate::<u64>(ptr::null_mut());
	}

	#[test]
	fn it_lr_rotates_null_children() {
		use crate::collections::hash_tree::lr_rotate;

		let entry = Entry::new(0).as_ptr();
		lr_rotate(entry);
	}

	#[test]
	fn it_rl_rotates_null_root() {
		use std::ptr;
		use crate::collections::hash_tree::rl_rotate;

		rl_rotate::<u64>(ptr::null_mut());
	}

	#[test]
	fn it_rl_rotates_null_children() {
		use crate::collections::hash_tree::rl_rotate;

		let entry = Entry::new(0).as_ptr();
		rl_rotate(entry);
	}

	#[test]
	fn it_rr_rotates_correctly() {
		use crate::collections::hash_tree::rr_rotate;

		let entry_one = Entry::new(1).as_ptr();
		let entry_two = Entry::new(2).as_ptr();
		let entry_three = Entry::new(3).as_ptr();

		unsafe {
			(*entry_one).right = entry_two;
			(*entry_two).right = entry_three;

			(*entry_one).height = 3;
			(*entry_two).height = 2;
			(*entry_three).height = 1;

			let root = rr_rotate(entry_one);
			assert_eq!((*root).data.read(), 2);

			let (left, right) = get_entry_children(root);
			assert_eq!((*left).data.read(), 1);
			assert_eq!((*right).data.read(), 3);

			assert_eq!((*entry_one).height, 1);
			assert_eq!((*entry_two).height, 2);
			assert_eq!((*entry_three).height, 1);
		}
	}

	#[test]
	fn it_ll_rotates_correctly() {
		use crate::collections::hash_tree::ll_rotate;

		let entry_one = Entry::new(1).as_ptr();
		let entry_two = Entry::new(2).as_ptr();
		let entry_three = Entry::new(3).as_ptr();

		unsafe {
			(*entry_one).left = entry_two;
			(*entry_two).left = entry_three;

			(*entry_one).height = 3;
			(*entry_two).height = 2;
			(*entry_three).height = 1;

			let root = ll_rotate(entry_one);
			assert_eq!((*root).data.read(), 2);

			let (left, right) = get_entry_children(root);
			assert_eq!((*left).data.read(), 3);
			assert_eq!((*right).data.read(), 1);

			assert_eq!((*entry_one).height, 1);
			assert_eq!((*entry_two).height, 2);
			assert_eq!((*entry_three).height, 1);
		}
	}

	#[test]
	fn it_lr_rotates_correctly() {
		use crate::collections::hash_tree::lr_rotate;

		let entry_one = Entry::new(1).as_ptr();
		let entry_two = Entry::new(2).as_ptr();
		let entry_three = Entry::new(3).as_ptr();

		unsafe {
			(*entry_one).left = entry_two;
			(*entry_two).left = entry_three;

			(*entry_one).height = 3;
			(*entry_two).height = 2;
			(*entry_three).height = 1;

			let root = lr_rotate(entry_one);
			assert_eq!((*root).data.read(), 2);

			let (left, right) = get_entry_children(root);
			assert_eq!((*left).data.read(), 3);
			assert_eq!((*right).data.read(), 1);

			assert_eq!((*entry_one).height, 1);
			assert_eq!((*entry_two).height, 2);
			assert_eq!((*entry_three).height, 1);
		}
	}

	fn get_entry_children<T>(entry: *mut Entry<T>) -> (*mut Entry<T>, *mut Entry<T>) {
		let left = unsafe {
			(*entry).left
		};

		let right = unsafe {
			(*entry).right
		};

		(left, right)
	}

	fn get_entry_info<T>(entry: *mut Entry<T>) -> (T, usize) {
		let data = unsafe {
			(*entry).data.read()
		};

		let height = unsafe {
			(*entry).height
		};

		(data, height)
	}
}
