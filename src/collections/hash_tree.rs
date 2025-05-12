use std::{
	cmp::Ordering,
	ptr::{self, NonNull},
	hash::{Hash, BuildHasher, RandomState},
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
		unsafe {
			let data_ptr = init_data_ptr(data);
			self.root = insert_entry(self.root, data_ptr);
		}

		todo!();
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
		let data_ptr = unsafe {
			init_data_ptr(data)
		};

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
}

unsafe fn init_data_ptr<T>(data: T) -> NonNull<T> {
	let boxed_data = Box::new(data);

	unsafe {
		NonNull::new_unchecked(Box::into_raw(boxed_data))
	}
}

/// inserts a new entry into the tree, returning the root
unsafe fn insert_entry<T>(
	root: *mut Entry<T>,
	data_ptr: NonNull<T>,
) -> *mut Entry<T>
where
	T: Ord,
{
	if root.is_null() {
		return Entry::from_data_ptr(data_ptr).as_ptr();
	}

	let cmp = unsafe {
		(*root).data.read().cmp(&data_ptr.read())
	};

	match cmp {
		Ordering::Less => unsafe {
			(*root).left = insert_entry((*root).left, data_ptr);
		},

		Ordering::Greater => unsafe {
			(*root).right = insert_entry((*root).right, data_ptr);
		},

		Ordering::Equal => {
			todo!();
		},
	};

	todo!();
}
