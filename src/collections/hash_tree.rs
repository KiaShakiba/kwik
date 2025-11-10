use std::{
	borrow::Borrow,
	cmp::{self, Ordering},
	collections::HashMap,
	hash::{BuildHasher, Hash, Hasher, RandomState},
	mem::MaybeUninit,
	ptr::{self, NonNull},
};

/// A hash AVL tree.
pub struct HashTree<T, S = RandomState> {
	map: HashMap<DataRef<T>, NonNull<Entry<T>>, S>,
	root: *mut Entry<T>,
}

struct Entry<T> {
	data: MaybeUninit<T>,

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
	#[inline]
	pub fn insert(&mut self, data: T) -> Option<T> {
		let maybe_old_entry = self.map.remove(&DataRef::from_ref(&data));

		if let Some(old_entry) = maybe_old_entry {
			self.root = remove_entry(self.root, old_entry.as_ptr());
			reset_entry(old_entry.as_ptr());
		}

		let entry = Entry::new(data);
		let entry_ptr = entry.as_ptr();

		self.root = insert_entry(self.root, entry_ptr);

		let data_ref = DataRef::from_entry_ptr(entry_ptr);
		self.map.insert(data_ref, entry);

		maybe_old_entry
			.map(|old_entry| Entry::<T>::into_data(old_entry.as_ptr()))
	}

	/// Returns a reference to the entry which has the corresponding
	/// hash of that of the supplied key or `None` if such an entry
	/// does not exist.
	///
	/// # Examples
	/// ```
	/// use kwik::collections::HashTree;
	///
	/// let mut tree = HashTree::<u64>::default();
	///
	/// tree.insert(1);
	/// tree.insert(2);
	///
	/// assert_eq!(tree.get(&1), Some(&1));
	/// assert_eq!(tree.get(&3), None);
	/// ```
	#[inline]
	pub fn get<K>(&self, key: &K) -> Option<&T>
	where
		T: Borrow<K>,
		K: Eq + Hash,
	{
		let entry = self.map.get(KeyWrapper::from_ref(key))?;
		let entry_ptr = entry.as_ptr();
		let data = unsafe { (*entry_ptr).data.assume_init_ref() };

		Some(data)
	}

	/// Returns a reference to the left (smaller) child entry of that
	/// which has the corresponding hash of the supplied key or `None`
	/// if such an entry does not exist.
	///
	/// # Examples
	/// ```
	/// use kwik::collections::HashTree;
	///
	/// let mut tree = HashTree::<u64>::default();
	///
	/// tree.insert(1);
	/// tree.insert(2);
	/// tree.insert(3);
	///
	/// assert_eq!(tree.left(&2), Some(&1));
	/// assert_eq!(tree.left(&1), None);
	/// assert_eq!(tree.left(&3), None);
	/// ```
	#[inline]
	pub fn left<K>(&self, key: &K) -> Option<&T>
	where
		T: Borrow<K>,
		K: Eq + Hash,
	{
		let entry = self.map.get(KeyWrapper::from_ref(key))?;
		let entry_ptr = entry.as_ptr();
		let left_ptr = unsafe { (*entry_ptr).left };

		if left_ptr.is_null() {
			return None;
		}

		let data = unsafe { (*left_ptr).data.assume_init_ref() };

		Some(data)
	}

	/// Returns a reference to the right (larger) child entry of that
	/// which has the corresponding hash of the supplied key or `None`
	/// if such an entry does not exist.
	///
	/// # Examples
	/// ```
	/// use kwik::collections::HashTree;
	///
	/// let mut tree = HashTree::<u64>::default();
	///
	/// tree.insert(1);
	/// tree.insert(2);
	/// tree.insert(3);
	///
	/// assert_eq!(tree.right(&2), Some(&3));
	/// assert_eq!(tree.right(&1), None);
	/// assert_eq!(tree.right(&3), None);
	/// ```
	#[inline]
	pub fn right<K>(&self, key: &K) -> Option<&T>
	where
		T: Borrow<K>,
		K: Eq + Hash,
	{
		let entry = self.map.get(KeyWrapper::from_ref(key))?;
		let entry_ptr = entry.as_ptr();
		let right_ptr = unsafe { (*entry_ptr).right };

		if right_ptr.is_null() {
			return None;
		}

		let data = unsafe { (*right_ptr).data.assume_init_ref() };

		Some(data)
	}

	/// Updates the entry which has the corresponding hash of that
	/// of the supplied key. Does nothing if such an entry does
	/// not exist.
	///
	/// # Examples
	/// ```
	/// use kwik::collections::HashTree;
	///
	/// let mut tree = HashTree::<u64>::default();
	///
	/// tree.insert(1);
	/// assert_eq!(tree.get(&1), Some(&1));
	///
	/// tree.update(&1, |value| *value += 1);
	///
	/// assert_eq!(tree.get(&1), None);
	/// assert_eq!(tree.get(&2), Some(&2));
	/// ```
	#[inline]
	pub fn update<K, F>(&mut self, key: &K, mut f: F)
	where
		T: Borrow<K>,
		K: Eq + Hash,
		F: FnMut(&mut T),
	{
		let Some(entry) = self.map.remove(KeyWrapper::from_ref(key)) else {
			return;
		};

		let entry_ptr = entry.as_ptr();
		let data = unsafe { &mut *(*entry_ptr).data.as_mut_ptr() };

		// updating the entry may change its postition in the tree, so we
		// have to remove it and later re-insert it
		self.root = remove_entry(self.root, entry_ptr);
		reset_entry(entry_ptr);

		f(data);

		let data_ref = DataRef::from_ref(data);

		self.root = insert_entry(self.root, entry_ptr);

		// updating the entry may have modified its resulting hash, so we
		// have to remove and re-insert it
		self.map.insert(data_ref, entry);
	}

	/// Removes and returns the entry which has the corresponding
	/// hash of that of the supplied key or `None` if such an entry
	/// does not exist.
	///
	/// # Examples
	/// ```
	/// use kwik::collections::HashTree;
	///
	/// let mut tree = HashTree::<u64>::default();
	///
	/// tree.insert(1);
	/// tree.insert(2);
	///
	/// assert_eq!(tree.remove(&1), Some(1));
	/// assert_eq!(tree.remove(&3), None);
	/// ```
	#[inline]
	pub fn remove<K>(&mut self, key: &K) -> Option<T>
	where
		T: Borrow<K>,
		K: Eq + Hash,
	{
		let entry = self.map.remove(KeyWrapper::from_ref(key))?;
		let entry_ptr = entry.as_ptr();

		self.root = remove_entry(self.root, entry_ptr);
		Some(Entry::<T>::into_data(entry_ptr))
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
		let entry = Entry {
			data: MaybeUninit::new(data),

			left: ptr::null_mut(),
			right: ptr::null_mut(),

			height: 1,
		};

		let boxed = Box::new(entry);

		unsafe { NonNull::new_unchecked(Box::into_raw(boxed)) }
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
			unsafe { (*self.left).height }
		} else {
			0
		};

		let right_height = if !self.right.is_null() {
			unsafe { (*self.right).height }
		} else {
			0
		};

		self.height = cmp::max(left_height, right_height) + 1;
	}

	fn into_data(entry_ptr: *mut Entry<T>) -> T {
		unsafe {
			let entry = *Box::from_raw(entry_ptr);
			entry.data.assume_init()
		}
	}
}

/// inserts a new entry into the tree, returning the root
fn insert_entry<T>(root: *mut Entry<T>, entry: *mut Entry<T>) -> *mut Entry<T>
where
	T: Ord,
{
	if root.is_null() {
		return entry;
	}

	let cmp = unsafe {
		(*entry)
			.data
			.assume_init_ref()
			.cmp((*root).data.assume_init_ref())
	};

	match cmp {
		Ordering::Less => unsafe {
			let new_left = insert_entry((*root).left, entry);
			(*root).set_left(new_left);

			let balanced = balance_entry(root);
			NonNull::new(balanced).unwrap().as_ptr()
		},

		Ordering::Greater => unsafe {
			let new_right = insert_entry((*root).right, entry);
			(*root).set_right(new_right);

			let balanced = balance_entry(root);
			NonNull::new(balanced).unwrap().as_ptr()
		},

		Ordering::Equal => unsafe {
			let new_left = (*root).left;
			let new_right = (*root).right;

			(*entry).set_left(new_left);
			(*entry).set_right(new_right);

			entry
		},
	}
}

/// removes an entry from the tree, returning the root
fn remove_entry<T>(root: *mut Entry<T>, entry: *mut Entry<T>) -> *mut Entry<T>
where
	T: Ord,
{
	if root.is_null() {
		return root;
	}

	let cmp = unsafe {
		let entry_ref = entry.as_ref().unwrap();

		entry_ref
			.data
			.assume_init_ref()
			.cmp((*root).data.assume_init_ref())
	};

	match cmp {
		Ordering::Less => unsafe {
			let new_left = remove_entry((*root).left, entry);
			(*root).set_left(new_left);

			let balanced = balance_entry(root);
			NonNull::new(balanced).unwrap().as_ptr()
		},

		Ordering::Greater => unsafe {
			let new_right = remove_entry((*root).right, entry);
			(*root).set_right(new_right);

			let balanced = balance_entry(root);
			NonNull::new(balanced).unwrap().as_ptr()
		},

		Ordering::Equal => unsafe {
			let left = (*root).left;
			let right = (*root).right;

			if left.is_null() || right.is_null() {
				if !left.is_null() {
					return left;
				}

				if !right.is_null() {
					return right;
				}

				ptr::null_mut()
			} else {
				let right_min = find_min(right);

				(*right_min).right = remove_entry(right, right_min);
				(*right_min).left = (*root).left;

				right_min
			}
		},
	}
}

fn reset_entry<T>(entry: *mut Entry<T>) {
	unsafe {
		(*entry).left = ptr::null_mut();
		(*entry).right = ptr::null_mut();
		(*entry).height = 1;
	}
}

/// returns the smallest entry in the tree
fn find_min<T>(root: *mut Entry<T>) -> *mut Entry<T>
where
	T: Ord,
{
	if root.is_null() {
		return root;
	}

	let mut current = root;

	loop {
		let left = unsafe { (*current).left };

		if left.is_null() {
			return current;
		}

		current = left;
	}
}

fn balance_entry<T>(entry: *mut Entry<T>) -> *mut Entry<T> {
	let factor = balance_factor(entry);

	if factor > 1 {
		let left_factor = unsafe { balance_factor((*entry).left) };

		if left_factor > 0 {
			return ll_rotate(entry);
		} else {
			return lr_rotate(entry);
		};
	}

	if factor < -1 {
		let right_factor = unsafe { balance_factor((*entry).right) };

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

	let left = unsafe { (*entry).left };
	let right = unsafe { (*entry).right };

	let left_height = if !left.is_null() {
		unsafe { (*left).height }
	} else {
		0
	};

	let right_height = if !right.is_null() {
		unsafe { (*right).height }
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
	fn from_ref(data: &T) -> Self {
		DataRef {
			data,
		}
	}

	fn from_entry_ptr(entry_ptr: *mut Entry<T>) -> Self {
		let data_ptr = unsafe { (*entry_ptr).data.as_ptr() };

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
		unsafe { (*self.data).hash(state) }
	}
}

impl<T> PartialEq for DataRef<T>
where
	T: PartialEq,
{
	fn eq(&self, other: &Self) -> bool {
		unsafe { (*self.data).eq(&*other.data) }
	}
}

impl<T> Eq for DataRef<T> where T: Eq {}

impl<K> KeyWrapper<K> {
	fn from_ref(key: &K) -> &Self {
		unsafe { &*(key as *const K as *const KeyWrapper<K>) }
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

impl<K> Eq for KeyWrapper<K> where K: Eq {}

impl<K, T> Borrow<KeyWrapper<K>> for DataRef<T>
where
	T: Borrow<K>,
{
	fn borrow(&self) -> &KeyWrapper<K> {
		let data_ref = unsafe { &*self.data }.borrow();

		KeyWrapper::from_ref(data_ref)
	}
}

#[cfg(test)]
mod tests {
	use crate::collections::hash_tree::{Entry, HashTree};

	#[test]
	fn it_inserts_correctly() {
		let mut tree = HashTree::<u64>::default();

		// insert 1
		assert_eq!(tree.insert(1), None);

		let (root_data, root_height) = get_entry_info(tree.root);
		assert_eq!(root_data, 1);
		assert_eq!(root_height, 1);

		let (l, r) = get_entry_children(tree.root);
		assert!(l.is_null());
		assert!(r.is_null());

		// insert 2
		assert_eq!(tree.insert(2), None);

		let (root_data, root_height) = get_entry_info(tree.root);
		assert_eq!(root_data, 1);
		assert_eq!(root_height, 2);

		let (l, r) = get_entry_children(tree.root);
		assert!(l.is_null());
		assert!(!r.is_null());

		let (r_data, r_height) = get_entry_info(r);
		assert_eq!(r_data, 2);
		assert_eq!(r_height, 1);

		// insert 3
		assert_eq!(tree.insert(3), None);

		let (root_data, root_height) = get_entry_info(tree.root);
		assert_eq!(root_data, 2);
		assert_eq!(root_height, 2);

		let (l, r) = get_entry_children(tree.root);
		assert!(!l.is_null());
		assert!(!r.is_null());

		let (l_data, l_height) = get_entry_info(l);
		assert_eq!(l_data, 1);
		assert_eq!(l_height, 1);

		let (r_data, r_height) = get_entry_info(r);
		assert_eq!(r_data, 3);
		assert_eq!(r_height, 1);

		// insert 4
		assert_eq!(tree.insert(4), None);

		let (root_data, root_height) = get_entry_info(tree.root);
		assert_eq!(root_data, 2);
		assert_eq!(root_height, 3);

		let (l, r) = get_entry_children(tree.root);
		assert!(!l.is_null());
		assert!(!r.is_null());

		let (l_data, l_height) = get_entry_info(l);
		assert_eq!(l_data, 1);
		assert_eq!(l_height, 1);

		let (r_data, r_height) = get_entry_info(r);
		assert_eq!(r_data, 3);
		assert_eq!(r_height, 2);

		let (rl, rr) = get_entry_children(r);
		assert!(rl.is_null());
		assert!(!rr.is_null());

		let (rr_data, rr_height) = get_entry_info(rr);
		assert_eq!(rr_data, 4);
		assert_eq!(rr_height, 1);

		// insert 5
		assert_eq!(tree.insert(5), None);

		let (root_data, root_height) = get_entry_info(tree.root);
		assert_eq!(root_data, 2);
		assert_eq!(root_height, 3);

		let (l, r) = get_entry_children(tree.root);
		assert!(!l.is_null());
		assert!(!r.is_null());

		let (l_data, l_height) = get_entry_info(l);
		assert_eq!(l_data, 1);
		assert_eq!(l_height, 1);

		let (r_data, r_height) = get_entry_info(r);
		assert_eq!(r_data, 4);
		assert_eq!(r_height, 2);

		let (rl, rr) = get_entry_children(r);
		assert!(!rl.is_null());
		assert!(!rr.is_null());

		let (rl_data, rl_height) = get_entry_info(rl);
		assert_eq!(rl_data, 3);
		assert_eq!(rl_height, 1);

		let (rr_data, rr_height) = get_entry_info(rr);
		assert_eq!(rr_data, 5);
		assert_eq!(rr_height, 1);

		// insert 6
		assert_eq!(tree.insert(6), None);

		let (root_data, root_height) = get_entry_info(tree.root);
		assert_eq!(root_data, 4);
		assert_eq!(root_height, 3);

		let (l, r) = get_entry_children(tree.root);
		assert!(!l.is_null());
		assert!(!r.is_null());

		let (l_data, l_height) = get_entry_info(l);
		assert_eq!(l_data, 2);
		assert_eq!(l_height, 2);

		let (ll, lr) = get_entry_children(l);
		assert!(!ll.is_null());
		assert!(!lr.is_null());

		let (ll_data, ll_height) = get_entry_info(ll);
		assert_eq!(ll_data, 1);
		assert_eq!(ll_height, 1);

		let (lr_data, lr_height) = get_entry_info(lr);
		assert_eq!(lr_data, 3);
		assert_eq!(lr_height, 1);

		let (r_data, r_height) = get_entry_info(r);
		assert_eq!(r_data, 5);
		assert_eq!(r_height, 2);

		let (rl, rr) = get_entry_children(r);
		assert!(rl.is_null());
		assert!(!rr.is_null());

		let (rr_data, rr_height) = get_entry_info(rr);
		assert_eq!(rr_data, 6);
		assert_eq!(rr_height, 1);

		// re-insert 1
		assert_eq!(tree.insert(1), Some(1));

		let (root_data, root_height) = get_entry_info(tree.root);
		assert_eq!(root_data, 4);
		assert_eq!(root_height, 3);

		let (l, r) = get_entry_children(tree.root);
		assert!(!l.is_null());
		assert!(!r.is_null());

		let (l_data, l_height) = get_entry_info(l);
		assert_eq!(l_data, 2);
		assert_eq!(l_height, 2);

		let (ll, lr) = get_entry_children(l);
		assert!(!ll.is_null());
		assert!(!lr.is_null());

		let (ll_data, ll_height) = get_entry_info(ll);
		assert_eq!(ll_data, 1);
		assert_eq!(ll_height, 1);

		let (lr_data, lr_height) = get_entry_info(lr);
		assert_eq!(lr_data, 3);
		assert_eq!(lr_height, 1);

		let (r_data, r_height) = get_entry_info(r);
		assert_eq!(r_data, 5);
		assert_eq!(r_height, 2);

		let (rl, rr) = get_entry_children(r);
		assert!(rl.is_null());
		assert!(!rr.is_null());

		let (rr_data, rr_height) = get_entry_info(rr);
		assert_eq!(rr_data, 6);
		assert_eq!(rr_height, 1);

		// re-insert 4
		assert_eq!(tree.insert(4), Some(4));

		let (root_data, root_height) = get_entry_info(tree.root);
		assert_eq!(root_data, 3);
		assert_eq!(root_height, 3);

		let (l, r) = get_entry_children(tree.root);
		assert!(!l.is_null());
		assert!(!r.is_null());

		let (l_data, l_height) = get_entry_info(l);
		assert_eq!(l_data, 2);
		assert_eq!(l_height, 2);

		let (ll, lr) = get_entry_children(l);
		assert!(!ll.is_null());
		assert!(lr.is_null());

		let (ll_data, ll_height) = get_entry_info(ll);
		assert_eq!(ll_data, 1);
		assert_eq!(ll_height, 1);

		let (r_data, r_height) = get_entry_info(r);
		assert_eq!(r_data, 5);
		assert_eq!(r_height, 2);

		let (rl, rr) = get_entry_children(r);
		assert!(!rl.is_null());
		assert!(!rr.is_null());

		let (rl_data, rl_height) = get_entry_info(rl);
		assert_eq!(rl_data, 4);
		assert_eq!(rl_height, 1);

		let (rr_data, rr_height) = get_entry_info(rr);
		assert_eq!(rr_data, 6);
		assert_eq!(rr_height, 1);
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
			assert_eq!((*root).data.assume_init(), 2);

			let (left, right) = get_entry_children(root);
			assert_eq!((*left).data.assume_init(), 1);
			assert_eq!((*right).data.assume_init(), 3);

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
			assert_eq!((*root).data.assume_init(), 2);

			let (left, right) = get_entry_children(root);
			assert_eq!((*left).data.assume_init(), 3);
			assert_eq!((*right).data.assume_init(), 1);

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
			assert_eq!((*root).data.assume_init(), 2);

			let (left, right) = get_entry_children(root);
			assert_eq!((*left).data.assume_init(), 3);
			assert_eq!((*right).data.assume_init(), 1);

			assert_eq!((*entry_one).height, 1);
			assert_eq!((*entry_two).height, 2);
			assert_eq!((*entry_three).height, 1);
		}
	}

	#[test]
	fn it_reorders_updates() {
		let mut tree = HashTree::<u64>::default();

		tree.insert(1);
		tree.insert(2);
		tree.insert(3);

		// before update
		let (data, height) = get_entry_info(tree.root);
		assert_eq!(data, 2);
		assert_eq!(height, 2);

		let (l, r) = get_entry_children(tree.root);
		assert!(!l.is_null());
		assert!(!r.is_null());

		let (l_data, l_height) = get_entry_info(l);
		assert_eq!(l_data, 1);
		assert_eq!(l_height, 1);

		let (r_data, r_height) = get_entry_info(r);
		assert_eq!(r_data, 3);
		assert_eq!(r_height, 1);

		// after update to leaf
		tree.update(&2, |value| *value = 4);

		let (data, height) = get_entry_info(tree.root);
		assert_eq!(data, 3);
		assert_eq!(height, 2);

		let (l, r) = get_entry_children(tree.root);
		assert!(!l.is_null());
		assert!(!r.is_null());

		let (l_data, l_height) = get_entry_info(l);
		assert_eq!(l_data, 1);
		assert_eq!(l_height, 1);

		let (r_data, r_height) = get_entry_info(r);
		assert_eq!(r_data, 4);
		assert_eq!(r_height, 1);

		// after update to root
		tree.update(&4, |value| *value = 2);

		let (data, height) = get_entry_info(tree.root);
		assert_eq!(data, 2);
		assert_eq!(height, 2);

		let (l, r) = get_entry_children(tree.root);
		assert!(!l.is_null());
		assert!(!r.is_null());

		let (l_data, l_height) = get_entry_info(l);
		assert_eq!(l_data, 1);
		assert_eq!(l_height, 1);

		let (r_data, r_height) = get_entry_info(r);
		assert_eq!(r_data, 3);
		assert_eq!(r_height, 1);
	}

	fn get_entry_children<T>(
		entry: *mut Entry<T>,
	) -> (*mut Entry<T>, *mut Entry<T>) {
		let left = unsafe { (*entry).left };
		let right = unsafe { (*entry).right };

		(left, right)
	}

	fn get_entry_info<T>(entry: *mut Entry<T>) -> (T, usize) {
		let data = unsafe { (*entry).data.assume_init_read() };
		let height = unsafe { (*entry).height };

		(data, height)
	}
}
