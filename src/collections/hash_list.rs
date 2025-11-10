/*
 * Copyright (c) Kia Shakiba
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

use std::{
	borrow::Borrow,
	collections::HashMap,
	fmt::{self, Debug, Formatter},
	hash::{BuildHasher, Hash, Hasher, RandomState},
	iter::{FromIterator, FusedIterator},
	marker::PhantomData,
	mem::MaybeUninit,
	ptr::{self, NonNull},
};

use serde::{
	de::{Deserialize, Deserializer, SeqAccess, Visitor},
	ser::{Serialize, SerializeSeq, Serializer},
};

/// A hash list where each entry is stored in a doubly-linked list.
pub struct HashList<T, S = RandomState> {
	map: HashMap<DataRef<T>, NonNull<Entry<T>>, S>,

	head: *mut Entry<T>,
	tail: *mut Entry<T>,
}

struct Entry<T> {
	data: MaybeUninit<T>,

	prev: *mut Entry<T>,
	next: *mut Entry<T>,
}

struct DataRef<T> {
	data: *const T,
}

#[repr(transparent)]
struct KeyWrapper<K>(K);

pub struct Iter<'a, T, S> {
	// we hold a reference to the list to ensure the entries have
	// correct lifetimes and to inform the size hint
	list: &'a HashList<T, S>,

	head: *mut Entry<T>,
	tail: *mut Entry<T>,
}

pub struct IntoIter<T, S> {
	list: HashList<T, S>,
}

impl<T, S> HashList<T, S>
where
	T: Eq + Hash,
	S: BuildHasher,
{
	/// Returns a references to the front entry of the list, or `None` if
	/// the list is empty.
	///
	/// # Examples
	/// ```
	/// use kwik::collections::HashList;
	///
	/// let mut list = HashList::<u64>::default();
	///
	/// list.push_back(1);
	/// list.push_back(2);
	/// list.push_back(3);
	///
	/// assert_eq!(list.front(), Some(&1));
	/// ```
	#[inline]
	pub fn front(&self) -> Option<&T> {
		if self.head.is_null() {
			return None;
		}

		let data = unsafe { (*self.head).data.assume_init_ref() };

		Some(data)
	}

	/// Returns a references to the back entry of the list, or `None` if
	/// the list is empty.
	///
	/// # Examples
	/// ```
	/// use kwik::collections::HashList;
	///
	/// let mut list = HashList::<u64>::default();
	///
	/// list.push_back(1);
	/// list.push_back(2);
	/// list.push_back(3);
	///
	/// assert_eq!(list.back(), Some(&3));
	/// ```
	#[inline]
	pub fn back(&self) -> Option<&T> {
		if self.tail.is_null() {
			return None;
		}

		let data = unsafe { (*self.tail).data.assume_init_ref() };

		Some(data)
	}

	/// Returns `true` if the hash list contains an entry with the corresponding
	/// hash of that of the supplied key.
	///
	/// # Examples
	/// ```
	/// use kwik::collections::HashList;
	///
	/// let mut list = HashList::<u64>::default();
	///
	/// list.push_back(1);
	/// list.push_back(2);
	/// list.push_back(3);
	///
	/// assert!(list.contains(&2));
	/// assert!(!list.contains(&4));
	/// ```
	#[inline]
	pub fn contains<K>(&self, key: &K) -> bool
	where
		T: Borrow<K>,
		K: Eq + Hash,
	{
		self.map.contains_key(KeyWrapper::from_ref(key))
	}

	/// Prepends an entry to the hash list.
	///
	/// If the hash list did not have this entry, `None` is returned.
	///
	/// If the hash list did have this entry, the new entry is inserted
	/// at the front of the hash list and the old entry is returned.
	///
	/// # Examples
	/// ```
	/// use kwik::collections::HashList;
	///
	/// let mut list = HashList::<u64>::default();
	///
	/// assert_eq!(list.push_front(1), None);
	/// assert_eq!(list.push_front(2), None);
	/// assert_eq!(list.push_front(3), None);
	/// assert_eq!(list.push_front(2), Some(2));
	/// ```
	#[inline]
	pub fn push_front(&mut self, data: T) -> Option<T> {
		let maybe_old_data =
			self.map
				.remove(&DataRef::from_ref(&data))
				.map(|old_entry| {
					let old_entry_ptr = old_entry.as_ptr();
					self.detach(old_entry_ptr);
					Entry::<T>::into_data(old_entry_ptr)
				});

		let entry = Entry::<T>::new(data);
		let entry_ptr = entry.as_ptr();

		self.attach_front(entry_ptr);

		let data_ref = DataRef::from_entry_ptr(entry_ptr);
		self.map.insert(data_ref, entry);

		maybe_old_data
	}

	/// Appends an entry to the hash list.
	///
	/// If the hash list did not have this entry, `None` is returned.
	///
	/// If the hash list did have this entry, the new entry is inserted
	/// at the back of the hash list and the old entry is returned.
	///
	/// # Examples
	/// ```
	/// use kwik::collections::HashList;
	///
	/// let mut list = HashList::<u64>::default();
	///
	/// assert_eq!(list.push_back(1), None);
	/// assert_eq!(list.push_back(2), None);
	/// assert_eq!(list.push_back(3), None);
	/// assert_eq!(list.push_back(2), Some(2));
	/// ```
	#[inline]
	pub fn push_back(&mut self, data: T) -> Option<T> {
		let maybe_old_data =
			self.map
				.remove(&DataRef::from_ref(&data))
				.map(|old_entry| {
					let old_entry_ptr = old_entry.as_ptr();
					self.detach(old_entry_ptr);
					Entry::<T>::into_data(old_entry_ptr)
				});

		let entry = Entry::<T>::new(data);
		let entry_ptr = entry.as_ptr();

		self.attach_back(entry_ptr);

		let data_ref = DataRef::from_entry_ptr(entry_ptr);
		self.map.insert(data_ref, entry);

		maybe_old_data
	}

	/// Moves and the entry which has the corresponding hash of that of
	/// the supplied key to the front of the hash list if one exists.
	///
	/// If such an entry does not exist, nothing happens.
	///
	/// # Examples
	/// ```
	/// use kwik::collections::HashList;
	///
	/// let mut list = HashList::<u64>::default();
	///
	/// list.push_back(1);
	/// list.push_back(2);
	/// list.push_back(3);
	///
	/// list.move_front(&2);
	/// ```
	#[inline]
	pub fn move_front<K>(&mut self, key: &K)
	where
		T: Borrow<K>,
		K: Eq + Hash,
	{
		let Some(entry_ref) = self.map.get(KeyWrapper::from_ref(key)) else {
			return;
		};

		let entry_ptr = entry_ref.as_ptr();

		if ptr::eq(self.head, entry_ptr) {
			// the entry is already at the head of the list, so don't do
			// unnecessary operations on it
			return;
		}

		self.detach(entry_ptr);
		self.attach_front(entry_ptr);
	}

	/// Moves and the entry which has the corresponding hash of that of
	/// the supplied key to the back of the hash list if one exists.
	///
	/// If such an entry does not exist, nothing happens.
	///
	/// # Examples
	/// ```
	/// use kwik::collections::HashList;
	///
	/// let mut list = HashList::<u64>::default();
	///
	/// list.push_back(1);
	/// list.push_back(2);
	/// list.push_back(3);
	///
	/// list.move_back(&2);
	/// ```
	#[inline]
	pub fn move_back<K>(&mut self, key: &K)
	where
		T: Borrow<K>,
		K: Eq + Hash,
	{
		let Some(entry_ref) = self.map.get(KeyWrapper::from_ref(key)) else {
			return;
		};

		let entry_ptr = entry_ref.as_ptr();

		if ptr::eq(self.tail, entry_ptr) {
			// the entry is already at the tail of the list, so don't do
			// unnecessary operations on it
			return;
		}

		self.detach(entry_ptr);
		self.attach_back(entry_ptr);
	}

	/// Removes the first entry and returns it, or `None` if the hash list is
	/// empty.
	///
	/// # Examples
	/// ```
	/// use kwik::collections::HashList;
	///
	/// let mut list = HashList::<u64>::default();
	///
	/// list.push_back(1);
	/// list.push_back(2);
	///
	/// assert_eq!(list.pop_front(), Some(1));
	/// assert_eq!(list.pop_front(), Some(2));
	/// assert_eq!(list.pop_front(), None);
	/// ```
	#[inline]
	pub fn pop_front(&mut self) -> Option<T> {
		if self.head.is_null() {
			return None;
		}

		let entry_ptr = self.head;
		self.detach(entry_ptr);

		let data_ref = DataRef::from_entry_ptr(entry_ptr);
		self.map.remove(&data_ref).unwrap();

		Some(Entry::<T>::into_data(entry_ptr))
	}

	/// Removes the first entry and returns it, or `None` if the hash list is
	/// empty.
	///
	/// # Examples
	/// ```
	/// use kwik::collections::HashList;
	///
	/// let mut list = HashList::<u64>::default();
	///
	/// list.push_back(1);
	/// list.push_back(2);
	///
	/// assert_eq!(list.pop_back(), Some(2));
	/// assert_eq!(list.pop_back(), Some(1));
	/// assert_eq!(list.pop_back(), None);
	/// ```
	#[inline]
	pub fn pop_back(&mut self) -> Option<T> {
		if self.tail.is_null() {
			return None;
		}

		let entry_ptr = self.tail;
		self.detach(entry_ptr);

		let data_ref = DataRef::from_entry_ptr(entry_ptr);
		self.map.remove(&data_ref).unwrap();

		Some(Entry::<T>::into_data(entry_ptr))
	}

	/// Returns a reference to the entry which has the corresponding
	/// hash of that of the supplied key or `None` if such an entry
	/// does not exist.
	///
	/// # Examples
	/// ```
	/// use kwik::collections::HashList;
	///
	/// let mut list = HashList::<u64>::default();
	///
	/// list.push_back(1);
	/// list.push_back(2);
	///
	/// assert_eq!(list.get(&1), Some(&1));
	/// assert_eq!(list.get(&3), None);
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

	/// Returns a reference to the entry before that which has the
	/// corresponding hash of the supplied key or `None` if such
	/// an entry does not exist.
	///
	/// # Examples
	/// ```
	/// use kwik::collections::HashList;
	///
	/// let mut list = HashList::<u64>::default();
	///
	/// list.push_back(1);
	/// list.push_back(2);
	///
	/// assert_eq!(list.before(&2), Some(&1));
	/// assert_eq!(list.before(&1), None);
	/// ```
	#[inline]
	pub fn before<K>(&self, key: &K) -> Option<&T>
	where
		T: Borrow<K>,
		K: Eq + Hash,
	{
		let entry = self.map.get(KeyWrapper::from_ref(key))?;
		let entry_ptr = entry.as_ptr();
		let prev_ptr = unsafe { (*entry_ptr).prev };

		if prev_ptr.is_null() {
			return None;
		}

		let data = unsafe { (*prev_ptr).data.assume_init_ref() };

		Some(data)
	}

	/// Returns a reference to the entry after that which has the
	/// corresponding hash of the supplied key or `None` if such
	/// an entry does not exist.
	///
	/// # Examples
	/// ```
	/// use kwik::collections::HashList;
	///
	/// let mut list = HashList::<u64>::default();
	///
	/// list.push_back(1);
	/// list.push_back(2);
	///
	/// assert_eq!(list.after(&1), Some(&2));
	/// assert_eq!(list.after(&2), None);
	/// ```
	#[inline]
	pub fn after<K>(&self, key: &K) -> Option<&T>
	where
		T: Borrow<K>,
		K: Eq + Hash,
	{
		let entry = self.map.get(KeyWrapper::from_ref(key))?;
		let entry_ptr = entry.as_ptr();
		let next_ptr = unsafe { (*entry_ptr).next };

		if next_ptr.is_null() {
			return None;
		}

		let data = unsafe { (*next_ptr).data.assume_init_ref() };

		Some(data)
	}

	/// Updates the entry which has the corresponding hash of that
	/// of the supplied key. Does nothing if such an entry does
	/// not exist.
	///
	/// # Examples
	/// ```
	/// use kwik::collections::HashList;
	///
	/// let mut list = HashList::<u64>::default();
	///
	/// list.push_back(1);
	/// assert_eq!(list.get(&1), Some(&1));
	///
	/// list.update(&1, |value| *value += 1);
	///
	/// assert_eq!(list.get(&1), None);
	/// assert_eq!(list.get(&2), Some(&2));
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

		f(data);

		let data_ref = DataRef::from_ref(data);

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
	/// use kwik::collections::HashList;
	///
	/// let mut list = HashList::<u64>::default();
	///
	/// list.push_back(1);
	/// list.push_back(2);
	///
	/// assert_eq!(list.remove(&1), Some(1));
	/// assert_eq!(list.remove(&3), None);
	/// ```
	#[inline]
	pub fn remove<K>(&mut self, key: &K) -> Option<T>
	where
		T: Borrow<K>,
		K: Eq + Hash,
	{
		let entry = self.map.remove(KeyWrapper::from_ref(key))?;
		let entry_ptr = entry.as_ptr();

		self.detach(entry_ptr);
		Some(Entry::<T>::into_data(entry_ptr))
	}

	/// Clears the hash list, removing all entries.
	///
	/// # Examples
	/// ```
	/// use kwik::collections::HashList;
	///
	/// let mut list = HashList::<u64>::default();
	///
	/// list.push_back(1);
	/// list.push_back(2);
	/// list.push_back(3);
	///
	/// list.clear();
	///
	/// assert_eq!(list.len(), 0);
	/// ```
	#[inline]
	pub fn clear(&mut self) {
		while self.pop_front().is_some() {}
	}

	fn attach_front(&mut self, entry_ptr: *mut Entry<T>) {
		unsafe {
			(*entry_ptr).next = self.head;
		}

		if !self.head.is_null() {
			unsafe {
				(*self.head).prev = entry_ptr;
			}
		}

		self.head = entry_ptr;

		if self.tail.is_null() {
			self.tail = entry_ptr;
		}
	}

	fn attach_back(&mut self, entry_ptr: *mut Entry<T>) {
		unsafe {
			(*entry_ptr).prev = self.tail;
		}

		if !self.tail.is_null() {
			unsafe {
				(*self.tail).next = entry_ptr;
			}
		}

		self.tail = entry_ptr;

		if self.head.is_null() {
			self.head = entry_ptr;
		}
	}

	fn detach(&mut self, entry_ptr: *mut Entry<T>) {
		let prev = unsafe { (*entry_ptr).prev };
		let next = unsafe { (*entry_ptr).next };

		if !prev.is_null() {
			unsafe {
				(*prev).next = next;
			}
		}

		if !next.is_null() {
			unsafe {
				(*next).prev = prev;
			}
		}

		if ptr::eq(self.head, entry_ptr) {
			self.head = next;
		}

		if ptr::eq(self.tail, entry_ptr) {
			self.tail = prev;
		}

		unsafe {
			(*entry_ptr).next = ptr::null_mut();
			(*entry_ptr).prev = ptr::null_mut();
		}
	}
}

impl<T, S> HashList<T, S> {
	/// Creates a new hash list with the supplied hasher.
	///
	/// # Examples
	/// ```
	/// use std::hash::RandomState;
	/// use kwik::collections::HashList;
	///
	/// let s = RandomState::new();
	/// let list = HashList::<u64, RandomState>::with_hasher(s);
	/// ```
	#[inline]
	pub fn with_hasher(hasher: S) -> Self {
		HashList {
			map: HashMap::with_hasher(hasher),

			head: ptr::null_mut(),
			tail: ptr::null_mut(),
		}
	}

	/// Returns `true` if the hash list contains no entries.
	///
	/// # Examples
	/// ```
	/// use kwik::collections::HashList;
	///
	/// let list = HashList::<u64>::default();
	/// assert!(list.is_empty());
	/// ```
	#[inline]
	pub fn is_empty(&self) -> bool {
		self.map.is_empty()
	}

	/// Returns the number of entries in the hash list.
	///
	/// # Examples
	/// ```
	/// use kwik::collections::HashList;
	///
	/// let list = HashList::<u64>::default();
	/// assert_eq!(list.len(), 0);
	/// ```
	#[inline]
	pub fn len(&self) -> usize {
		self.map.len()
	}

	/// Returns an iterator over the hash list.
	///
	/// # Examples
	/// ```
	/// use kwik::collections::HashList;
	///
	/// let list = HashList::<u64>::default();
	///
	/// // add entries to list
	///
	/// for entry in list.iter() {
	///     // use entry
	/// }
	/// ```
	#[inline]
	pub fn iter(&self) -> Iter<'_, T, S> {
		Iter {
			list: self,

			head: self.head,
			tail: self.tail,
		}
	}
}

impl<T> HashList<T, RandomState> {
	/// Creates a new hash list.
	///
	/// # Examples
	/// ```
	/// use kwik::collections::HashList;
	///
	/// let list = HashList::<u64>::new();
	/// ```
	#[inline]
	pub fn new() -> Self {
		HashList::with_hasher(RandomState::new())
	}
}

impl<T, S> Default for HashList<T, S>
where
	S: Default,
{
	fn default() -> Self {
		HashList::<T, S>::with_hasher(S::default())
	}
}

impl<T, S> PartialEq for HashList<T, S>
where
	T: PartialEq,
{
	fn eq(&self, other: &Self) -> bool {
		self.len() == other.len() && self.iter().eq(other.iter())
	}
}

impl<T, S> Eq for HashList<T, S> where T: Eq {}

impl<T> Entry<T> {
	fn new(data: T) -> NonNull<Self> {
		let entry = Entry {
			data: MaybeUninit::new(data),

			prev: ptr::null_mut(),
			next: ptr::null_mut(),
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

impl<'a, T, S> Iterator for Iter<'a, T, S> {
	type Item = &'a T;

	fn next(&mut self) -> Option<Self::Item> {
		if self.head.is_null() {
			return None;
		}

		let prev = unsafe { (*self.head).prev };

		// the head pointer may have passed the tail pointer
		// if using a double ended iterator
		if ptr::eq(prev, self.tail) {
			return None;
		}

		let data = unsafe { (*self.head).data.assume_init_ref() };

		unsafe {
			self.head = (*self.head).next;
		}

		Some(data)
	}

	fn size_hint(&self) -> (usize, Option<usize>) {
		(self.list.len(), Some(self.list.len()))
	}
}

impl<T, S> DoubleEndedIterator for Iter<'_, T, S> {
	fn next_back(&mut self) -> Option<Self::Item> {
		if self.tail.is_null() {
			return None;
		}

		let next = unsafe { (*self.tail).next };

		// the tail pointer may have passed the head pointer
		// if using a double ended iterator
		if ptr::eq(next, self.head) {
			return None;
		}

		let data = unsafe { (*self.tail).data.assume_init_ref() };

		unsafe {
			self.tail = (*self.tail).prev;
		}

		Some(data)
	}
}

impl<'a, T, S> IntoIterator for &'a HashList<T, S>
where
	T: Eq + Hash,
	S: BuildHasher,
{
	type Item = &'a T;
	type IntoIter = Iter<'a, T, S>;

	fn into_iter(self) -> Self::IntoIter {
		self.iter()
	}
}

impl<T, S> Iterator for IntoIter<T, S>
where
	T: Eq + Hash,
	S: BuildHasher,
{
	type Item = T;

	fn next(&mut self) -> Option<Self::Item> {
		self.list.pop_front()
	}
}

impl<T, S> DoubleEndedIterator for IntoIter<T, S>
where
	T: Eq + Hash,
	S: BuildHasher,
{
	fn next_back(&mut self) -> Option<Self::Item> {
		self.list.pop_back()
	}
}

impl<T, S> ExactSizeIterator for Iter<'_, T, S>
where
	T: Eq + Hash,
	S: BuildHasher,
{
}

impl<T, S> ExactSizeIterator for IntoIter<T, S>
where
	T: Eq + Hash,
	S: BuildHasher,
{
}

impl<T, S> FusedIterator for Iter<'_, T, S>
where
	T: Eq + Hash,
	S: BuildHasher,
{
}

impl<T, S> FusedIterator for IntoIter<T, S>
where
	T: Eq + Hash,
	S: BuildHasher,
{
}

impl<T, S> IntoIterator for HashList<T, S>
where
	T: Eq + Hash,
	S: BuildHasher,
{
	type Item = T;
	type IntoIter = IntoIter<T, S>;

	fn into_iter(self) -> Self::IntoIter {
		IntoIter {
			list: self,
		}
	}
}

impl<T, S> FromIterator<T> for HashList<T, S>
where
	T: Eq + Hash,
	S: Default + BuildHasher,
{
	fn from_iter<I>(iter: I) -> Self
	where
		I: IntoIterator<Item = T>,
	{
		let mut list = HashList::<T, S>::default();

		for value in iter {
			list.push_back(value);
		}

		list
	}
}

impl<T, S> Extend<T> for HashList<T, S>
where
	T: Eq + Hash,
	S: BuildHasher,
{
	fn extend<I>(&mut self, iter: I)
	where
		I: IntoIterator<Item = T>,
	{
		for value in iter {
			self.push_back(value);
		}
	}
}

impl<T, S> Hash for HashList<T, S>
where
	T: Eq + Hash,
	S: BuildHasher,
{
	fn hash<H>(&self, state: &mut H)
	where
		H: Hasher,
	{
		self.len().hash(state);

		for value in self {
			value.hash(state);
		}
	}
}

impl<T, S> Debug for HashList<T, S>
where
	T: Eq + Hash + Debug,
	S: BuildHasher,
{
	fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
		fmt.write_str("HashList(")?;
		fmt.debug_list().entries(self).finish()?;
		fmt.write_str(")")?;

		Ok(())
	}
}

impl<T, S> Drop for HashList<T, S> {
	fn drop(&mut self) {
		self.map.drain().for_each(|(_, entry)| unsafe {
			let mut entry = *Box::from_raw(entry.as_ptr());
			ptr::drop_in_place(entry.data.as_mut_ptr());
		});
	}
}

impl<T, S> Serialize for HashList<T, S>
where
	T: Eq + Hash + Serialize,
	S: BuildHasher,
{
	fn serialize<Se>(&self, serializer: Se) -> Result<Se::Ok, Se::Error>
	where
		Se: Serializer,
	{
		let mut seq = serializer.serialize_seq(Some(self.len()))?;

		for value in self {
			seq.serialize_element(value)?;
		}

		seq.end()
	}
}

struct HashListVisitor<T, S> {
	marker: PhantomData<(T, S)>,
}

impl<'de, T, S> Visitor<'de> for HashListVisitor<T, S>
where
	T: Eq + Hash + Deserialize<'de>,
	S: Default + BuildHasher,
{
	type Value = HashList<T, S>;

	fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
		formatter.write_str("a hash list")
	}

	fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
	where
		A: SeqAccess<'de>,
	{
		let mut list = HashList::<T, S>::default();

		while let Some(value) = seq.next_element()? {
			list.push_back(value);
		}

		Ok(list)
	}
}

impl<'de, T, S> Deserialize<'de> for HashList<T, S>
where
	T: Eq + Hash + Deserialize<'de>,
	S: Default + BuildHasher,
{
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: Deserializer<'de>,
	{
		let visitor = HashListVisitor {
			marker: PhantomData,
		};

		deserializer.deserialize_seq(visitor)
	}
}

unsafe impl<T, S> Send for HashList<T, S> {}
unsafe impl<T, S> Sync for HashList<T, S> {}

#[cfg(test)]
mod tests {
	use std::{
		borrow::Borrow,
		hash::{Hash, Hasher},
	};

	use droptest::{DropGuard, DropRegistry, assert_drop, assert_no_drop};
	use serde_test::{Token, assert_tokens};

	use crate::collections::HashList;

	struct DroppableObject<'a> {
		id: u64,
		guard: DropGuard<'a, ()>,
	}

	#[test]
	fn it_pushes_front_correctly() {
		let mut list = HashList::<u32>::default();
		let values = vec![1, 2, 3, 4];

		for value in &values {
			list.push_front(*value);
		}

		for (index, value) in list.iter().enumerate() {
			assert_eq!(*value, values[values.len() - index - 1]);
		}
	}

	#[test]
	fn it_pushes_back_correctly() {
		let mut list = HashList::<u32>::default();
		let values = vec![1, 2, 3, 4];

		for value in &values {
			list.push_back(*value);
		}

		for (index, value) in list.iter().enumerate() {
			assert_eq!(*value, values[index]);
		}
	}

	#[test]
	fn it_moves_front_correctly() {
		let mut list = HashList::<u32>::default();
		let values = vec![1, 2, 3, 4];

		for value in &values {
			list.push_back(*value);
		}

		list.move_front(&2);

		let expected = [2, 1, 3, 4];

		for (index, value) in list.iter().enumerate() {
			assert_eq!(*value, expected[index]);
		}
	}

	#[test]
	fn it_moves_back_correctly() {
		let mut list = HashList::<u32>::default();
		let values = vec![1, 2, 3, 4];

		for value in &values {
			list.push_back(*value);
		}

		list.move_back(&2);

		let expected = [1, 3, 4, 2];

		for (index, value) in list.iter().enumerate() {
			assert_eq!(*value, expected[index]);
		}
	}

	#[test]
	fn it_pops_front_correctly() {
		let mut list = HashList::<u32>::default();
		let mut values = vec![1, 2, 3, 4];

		for value in &values {
			list.push_back(*value);
		}

		assert_eq!(list.pop_front(), Some(1));
		values.remove(0);

		for (index, value) in list.iter().enumerate() {
			assert_eq!(*value, values[index]);
		}
	}

	#[test]
	fn it_pops_back_correctly() {
		let mut list = HashList::<u32>::default();
		let mut values = vec![1, 2, 3, 4];

		for value in &values {
			list.push_back(*value);
		}

		assert_eq!(list.pop_back(), Some(4));
		values.remove(3);

		for (index, value) in list.iter().enumerate() {
			assert_eq!(*value, values[index]);
		}
	}

	#[test]
	fn it_iterates_correctly() {
		let list: HashList<u32> = [1, 2, 3].into_iter().collect();

		let mut iter = list.iter();
		assert_eq!(iter.next(), Some(&1));
		assert_eq!(iter.next(), Some(&2));
		assert_eq!(iter.next(), Some(&3));
		assert_eq!(iter.next(), None);

		let mut iter = list.into_iter();
		assert_eq!(iter.next(), Some(1));
		assert_eq!(iter.next(), Some(2));
		assert_eq!(iter.next(), Some(3));
		assert_eq!(iter.next(), None);
	}

	#[test]
	fn it_reverse_iterates_correctly() {
		let list: HashList<u32> = [1, 2, 3, 4, 5, 6].into_iter().collect();

		let mut iter = list.iter();
		assert_eq!(iter.next(), Some(&1));
		assert_eq!(iter.next_back(), Some(&6));
		assert_eq!(iter.next_back(), Some(&5));
		assert_eq!(iter.next(), Some(&2));
		assert_eq!(iter.next(), Some(&3));
		assert_eq!(iter.next(), Some(&4));
		assert_eq!(iter.next(), None);
		assert_eq!(iter.next_back(), None);

		let mut iter = list.into_iter();
		assert_eq!(iter.next(), Some(1));
		assert_eq!(iter.next_back(), Some(6));
		assert_eq!(iter.next_back(), Some(5));
		assert_eq!(iter.next(), Some(2));
		assert_eq!(iter.next(), Some(3));
		assert_eq!(iter.next(), Some(4));
		assert_eq!(iter.next(), None);
		assert_eq!(iter.next_back(), None);
	}

	#[test]
	fn it_drops_removed_object() {
		let registry = DropRegistry::default();
		let mut list = HashList::<DroppableObject>::default();

		let object1 = DroppableObject::new(&registry, 1);
		let object2 = DroppableObject::new(&registry, 2);

		let object1_guard_id = object1.guard.id();
		let object2_guard_id = object2.guard.id();

		list.push_front(object1);
		list.push_front(object2);
		list.remove(&1);

		assert_drop!(registry, object1_guard_id);
		assert_no_drop!(registry, object2_guard_id);
	}

	#[test]
	fn it_drops_front_popped_object() {
		let registry = DropRegistry::default();
		let mut list = HashList::<DroppableObject>::default();

		let object1 = DroppableObject::new(&registry, 1);
		let object2 = DroppableObject::new(&registry, 2);

		let object1_guard_id = object1.guard.id();
		let object2_guard_id = object2.guard.id();

		list.push_front(object1);
		list.push_front(object2);
		list.pop_front();

		assert_no_drop!(registry, object1_guard_id);
		assert_drop!(registry, object2_guard_id);
	}

	#[test]
	fn it_drops_back_popped_object() {
		let registry = DropRegistry::default();
		let mut list = HashList::<DroppableObject>::default();

		let object1 = DroppableObject::new(&registry, 1);
		let object2 = DroppableObject::new(&registry, 2);

		let object1_guard_id = object1.guard.id();
		let object2_guard_id = object2.guard.id();

		list.push_front(object1);
		list.push_front(object2);
		list.pop_back();

		assert_drop!(registry, object1_guard_id);
		assert_no_drop!(registry, object2_guard_id);
	}

	#[test]
	fn it_drops_front_replaced_object() {
		let registry = DropRegistry::default();
		let mut list = HashList::<DroppableObject>::default();

		let object1 = DroppableObject::new(&registry, 1);
		let object2 = DroppableObject::new(&registry, 1);

		let object1_guard_id = object1.guard.id();
		let object2_guard_id = object2.guard.id();

		list.push_front(object1);
		list.push_front(object2);

		assert_drop!(registry, object1_guard_id);
		assert_no_drop!(registry, object2_guard_id);
	}

	#[test]
	fn it_drops_back_replaced_object() {
		let registry = DropRegistry::default();
		let mut list = HashList::<DroppableObject>::default();

		let object1 = DroppableObject::new(&registry, 1);
		let object2 = DroppableObject::new(&registry, 1);

		let object1_guard_id = object1.guard.id();
		let object2_guard_id = object2.guard.id();

		list.push_back(object1);
		list.push_back(object2);

		assert_drop!(registry, object1_guard_id);
		assert_no_drop!(registry, object2_guard_id);
	}

	#[test]
	fn it_ser_de_empty() {
		let list = HashList::<u32>::default();

		assert_tokens(&list, &[
			Token::Seq {
				len: Some(0),
			},
			Token::SeqEnd,
		]);
	}

	#[test]
	fn it_ser_de() {
		let list: HashList<u32> = [1, 2, 3, 4, 5, 6].into_iter().collect();

		assert_tokens(&list, &[
			Token::Seq {
				len: Some(6),
			},
			Token::U32(1),
			Token::U32(2),
			Token::U32(3),
			Token::U32(4),
			Token::U32(5),
			Token::U32(6),
			Token::SeqEnd,
		]);
	}

	impl<'a> DroppableObject<'a> {
		fn new(registry: &'a DropRegistry, id: u64) -> Self {
			DroppableObject {
				id,
				guard: registry.new_guard(),
			}
		}
	}

	impl PartialEq for DroppableObject<'_> {
		fn eq(&self, other: &Self) -> bool {
			self.id == other.id
		}
	}

	impl Eq for DroppableObject<'_> {}

	impl Borrow<u64> for DroppableObject<'_> {
		fn borrow(&self) -> &u64 {
			&self.id
		}
	}

	impl Hash for DroppableObject<'_> {
		fn hash<H>(&self, state: &mut H)
		where
			H: Hasher,
		{
			self.id.hash(state)
		}
	}
}
