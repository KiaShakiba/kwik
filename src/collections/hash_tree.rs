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
		if self.root.is_null() {
			let entry = Entry::<T>::new(data);
			let entry_ptr = entry.as_ptr();

			self.root = entry_ptr;
			return None;
		}

		unsafe {
			let data_ptr = init_data_ptr(data);
			(*self.root).insert(data_ptr)
		}
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

impl<T> Entry<T>
where
	T: Ord,
{
	fn insert(&mut self, data_ptr: NonNull<T>) -> Option<T> {
		let cmp = unsafe {
			self.data.read().cmp(&data_ptr.read())
		};

		match cmp {
			Ordering::Less if self.left.is_null() => {
				let entry = Entry::<T>::from_data_ptr(data_ptr);
				self.left = entry.as_ptr();
			},

			Ordering::Less => return unsafe {
				(*self.left).insert(data_ptr)
			},

			Ordering::Greater if self.right.is_null() => {
				let entry = Entry::<T>::from_data_ptr(data_ptr);
				self.right = entry.as_ptr();
			},

			Ordering::Greater => return unsafe {
				(*self.right).insert(data_ptr)
			},

			Ordering::Equal => {
				// the old data might actually be different than the new
				// data, even though their cmp is equal
				let old_data = unsafe {
					self.data.read()
				};

				self.data = data_ptr;
				return Some(old_data);
			},
		}

		None
	}
}
