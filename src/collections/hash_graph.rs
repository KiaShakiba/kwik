use std::{
	borrow::Borrow,
	collections::HashMap,
	hash::{BuildHasher, Hash, Hasher, RandomState},
	mem::MaybeUninit,
	ptr::{self, NonNull},
};

use num_traits::AsPrimitive;

pub struct HashGraph<T, S = RandomState> {
	map: HashMap<DataRef<T>, NonNull<Entry<T>>, S>,
}

struct Entry<T> {
	data:  MaybeUninit<T>,
	conns: Vec<Connection<T>>,
}

struct Connection<T> {
	to:     NonNull<Entry<T>>,
	weight: f64,
}

struct DataRef<T> {
	data: *const T,
}

#[repr(transparent)]
struct KeyWrapper<K>(K);

impl<T, S> HashGraph<T, S>
where
	T: Eq + Hash,
	S: BuildHasher,
{
	/// Inserts a disconnected entry into the hash graph.
	///
	/// If the hash graph did not have this entry, `None` is returned.
	///
	/// If the hash graph did have this entry, the new entry is inserted
	/// and the old entry is returned.
	///
	/// # Examples
	/// ```
	/// use kwik::collections::HashGraph;
	///
	/// let mut graph = HashGraph::<u64>::default();
	///
	/// assert_eq!(graph.insert(1), None);
	/// assert_eq!(graph.insert(2), None);
	/// assert_eq!(graph.insert(3), None);
	/// assert_eq!(graph.insert(2), Some(2));
	/// ```
	#[inline]
	pub fn insert(&mut self, data: T) -> Option<T> {
		let maybe_old_entry = self.map.remove(&DataRef::from_ref(&data));

		let entry = Entry::new(data);
		let entry_ptr = entry.as_ptr();

		let data_ref = DataRef::from_entry_ptr(entry_ptr);
		self.map.insert(data_ref, entry);

		maybe_old_entry.map(|old_entry| Entry::<T>::into_data(old_entry.as_ptr()))
	}

	/// Connects two entries in the hash graph.
	///
	/// If the hash graph does not contain either entry, or the entries
	/// are already connected with the same weight, nothing is updated.
	///
	/// If the entries exist and are connected with a different weight,
	/// the weight is updated.
	///
	/// # Examples
	/// ```
	/// use kwik::collections::HashGraph;
	///
	/// let mut graph = HashGraph::<u64>::default();
	///
	/// graph.insert(1);
	/// graph.insert(2);
	/// graph.insert(3);
	///
	/// graph.connect(&1, &2, 1);
	/// ```
	#[inline]
	pub fn connect<K1, K2>(&mut self, from: &K1, to: &K2, weight: impl AsPrimitive<f64>)
	where
		T: Borrow<K1> + Borrow<K2>,
		K1: Eq + Hash,
		K2: Eq + Hash,
	{
		let Some((from_ref, to_ref)) = self
			.map
			.get(KeyWrapper::from_ref(from))
			.zip(self.map.get(KeyWrapper::from_ref(to)))
		else {
			return;
		};

		let from_ptr = from_ref.as_ptr();
		let to_ptr = to_ref.as_ptr();

		let from_conns = unsafe { &mut (*from_ptr).conns };

		let maybe_conn = from_conns
			.iter_mut()
			.find(|conn| ptr::eq(conn.to.as_ptr(), to_ptr));

		if let Some(conn) = maybe_conn {
			conn.weight = weight.as_();
		} else {
			from_conns.push(Connection::new(*to_ref, weight));
		}
	}

	/// Returns `true` if the supplied entries are connected in the graph.
	///
	/// If the hash graph does not contain either entry, `false` is returned.
	///
	/// # Examples
	/// ```
	/// use kwik::collections::HashGraph;
	///
	/// let mut graph = HashGraph::<u64>::default();
	///
	/// graph.insert(1);
	/// graph.insert(2);
	/// graph.insert(3);
	///
	/// graph.connect(&1, &2, 1);
	///
	/// assert!(graph.is_connected(&1, &2));
	/// assert!(!graph.is_connected(&1, &3));
	/// assert!(!graph.is_connected(&1, &4));
	/// ```
	#[inline]
	pub fn is_connected<K1, K2>(&self, from: &K1, to: &K2) -> bool
	where
		T: Borrow<K1> + Borrow<K2>,
		K1: Eq + Hash,
		K2: Eq + Hash,
	{
		let Some((from_ref, to_ref)) = self
			.map
			.get(KeyWrapper::from_ref(from))
			.zip(self.map.get(KeyWrapper::from_ref(to)))
		else {
			return false;
		};

		let from_ptr = from_ref.as_ptr();
		let to_ptr = to_ref.as_ptr();

		let from_conns = unsafe { &mut (*from_ptr).conns };

		from_conns
			.iter()
			.any(|conn| ptr::eq(conn.to.as_ptr(), to_ptr))
	}

	/// Returns the shortest path from entry `from` to entry `to`.
	///
	/// If no path exists, `None` is returned.
	///
	/// # Examples
	/// ```
	/// use kwik::collections::HashGraph;
	///
	/// let mut graph = HashGraph::<u64>::default();
	///
	/// graph.insert(1);
	/// graph.insert(2);
	/// graph.insert(3);
	///
	/// graph.connect(&1, &2, 1);
	/// graph.connect(&2, &3, 1);
	///
	/// assert_eq!(graph.path(&1, &3), vec![&1, &2, &3]);
	/// ```
	pub fn path<K1, K2>(&self, from: &K1, to: &K2) -> Option<Vec<&T>>
	where
		T: Borrow<K1> + Borrow<K2>,
		K1: Eq + Hash,
		K2: Eq + Hash,
	{
		let (from_ref, to_ref) = self
			.map
			.get(KeyWrapper::from_ref(from))
			.zip(self.map.get(KeyWrapper::from_ref(to)))?;

		let from_ptr = from_ref.as_ptr();
		let to_ptr = to_ref.as_ptr();

		todo!();
	}
}

impl<T, S> HashGraph<T, S> {
	/// Creates a new hash graph with the supplied hasher.
	///
	/// # Examples
	/// ```
	/// use std::hash::RandomState;
	/// use kwik::collections::HashGraph;
	///
	/// let s = RandomState::new();
	/// let graph = HashGraph::<u64, RandomState>::with_hasher(s);
	/// ```
	#[inline]
	pub fn with_hasher(hasher: S) -> Self {
		HashGraph {
			map: HashMap::with_hasher(hasher),
		}
	}

	/// Returns `true` if the hash graph contains no entries.
	///
	/// # Examples
	/// ```
	/// use kwik::collections::HashGraph;
	///
	/// let graph = HashGraph::<u64>::default();
	/// assert!(graph.is_empty());
	/// ```
	#[inline]
	pub fn is_empty(&self) -> bool {
		self.map.is_empty()
	}

	/// Returns the number of entries in the hash graph.
	///
	/// # Examples
	/// ```
	/// use kwik::collections::HashGraph;
	///
	/// let graph = HashGraph::<u64>::default();
	/// assert_eq!(graph.len(), 0);
	/// ```
	#[inline]
	pub fn len(&self) -> usize {
		self.map.len()
	}
}

impl<T> HashGraph<T, RandomState> {
	/// Creates a new hash graph.
	///
	/// # Examples
	/// ```
	/// use kwik::collections::HashGraph;
	///
	/// let graph = HashGraph::<u64>::new();
	/// ```
	#[inline]
	pub fn new() -> Self {
		HashGraph::with_hasher(RandomState::new())
	}
}

impl<T, S> Default for HashGraph<T, S>
where
	S: Default,
{
	fn default() -> Self {
		HashGraph::<T, S>::with_hasher(S::default())
	}
}

// impl<T, S> PartialEq for HashGraph<T, S>
// where
// 	T: PartialEq,
// {
// 	fn eq(&self, other: &Self) -> bool {
// 		self.len() == other.len() && self.iter().eq(other.iter())
// 	}
// }

// impl<T, S> Eq for HashGraph<T, S> where T: Eq {}

impl<T> Entry<T> {
	fn new(data: T) -> NonNull<Self> {
		let entry = Entry {
			data:  MaybeUninit::new(data),
			conns: Vec::new(),
		};

		let boxed = Box::new(entry);
		unsafe { NonNull::new_unchecked(Box::into_raw(boxed)) }
	}

	fn into_data(entry_ptr: *mut Entry<T>) -> T {
		unsafe {
			let entry = *Box::from_raw(entry_ptr);
			entry.data.assume_init()
		}
	}
}

impl<T> Connection<T> {
	fn new(to: NonNull<Entry<T>>, weight: impl AsPrimitive<f64>) -> Self {
		Connection {
			to,
			weight: weight.as_(),
		}
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
			data: data_ptr
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
