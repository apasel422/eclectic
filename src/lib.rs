//! Collection traits for generic programming.
//!
//! The principal traits in this library are:
//!
//! - [`Collection`]
//!     - [`List`]
//!     - [`Map`]
//!     - [`Set`]
//!     - [`Queue`]
//!         - [`FifoQueue`]
//!         - [`PrioQueue`]
//!     - [`Deque`]
//!         - [`FifoDeque`]
//!         - [`PrioDeque`]
//!
//! When combined with these traits, two marker traits enable the use of additional operations:
//!
//! | Marker        | Operations                                     | Analogous Type |
//! |---------------|------------------------------------------------|----------------|
//! | (none)        | Read-only access to a collection and its items | `&[T]`         |
//! | [`Mutate`]    | Write access to a collection's items           | `&mut [T]`     |
//! | [`AddRemove`] | Insertion and removal of a collection's items  | `&mut Vec<T>`  |
//!
//! Generic code should specify only those bounds that are needed for its operation, but may
//! specify additional bounds for future compatibility. Generic code should also use the collection
//! traits with a `?Sized` bound in order to support slices and trait objects whenever possible.
//!
//! # Examples
//!
//! Insertion sort:
//!
//! ```
//! use eclectic::{List, Mutate};
//!
//! fn insertion_sort<L: ?Sized + List + Mutate>(list: &mut L) where L::Item: Ord {
//!     for i in 1..list.len() { // `len` is defined on `Collection`, a supertrait of `List`
//!         let mut j = i;
//!
//!         while j > 0 && list.get(j) < list.get(j - 1) {
//!             list.swap(j, j - 1); // the `Mutate` bound on `L` enables the use of `List::swap`
//!             j -= 1;
//!         }
//!     }
//! }
//!
//! use std::collections::VecDeque;
//!
//! let mut vec = vec!['c', 'a', 'e', 'd', 'b'];
//! let mut vec_deque: VecDeque<_> = vec.iter().cloned().collect();
//!
//! insertion_sort(&mut vec);
//! assert_eq!(vec, ['a', 'b', 'c', 'd', 'e']);
//!
//! insertion_sort(&mut vec_deque);
//! assert!(vec_deque.iter().eq(&['a', 'b', 'c', 'd', 'e']));
//! ```
//!
//! [`AddRemove`]: trait.AddRemove.html
//! [`Collection`]: trait.Collection.html
//! [`Deque`]: trait.Deque.html
//! [`FifoDeque`]: trait.FifoDeque.html
//! [`FifoQueue`]: trait.FifoQueue.html
//! [`List`]: trait.List.html
//! [`Map`]: map/trait.Map.html
//! [`Mutate`]: trait.Mutate.html
//! [`PrioDeque`]: trait.PrioDeque.html
//! [`PrioQueue`]: trait.PrioQueue.html
//! [`Queue`]: trait.Queue.html
//! [`Set`]: set/trait.Set.html
//!
//! # A Note on Trait Objects
//!
//! A number of trait methods in this crate return a `Box<Iterator>`, which requires unnecessary
//! heap allocation and opaqueness (e.g. erasure of traits like `Clone` and `DoubleEndedIterator`).
//! This is to make up for the (hopefully temporary) inability to define higher-kinded associated
//! types like:
//!
//! ```ignore
//! trait Collection {
//!     type Drain<'a>: 'a + Iterator<Item = Self::Item>;
//!
//!     fn drain<'a>(&'a mut self) -> Self::Drain<'a> where Self: AddRemove;
//! }
//! ```
//!
//! If Rust acquires such types, the iterator- and entry-returning methods will be changed to use
//! them.

#![deny(missing_docs)]
#![cfg_attr(feature = "nightly", feature(
    binary_heap_extras,
    deque_extras,
    set_recovery,
))]

mod impls;

pub use map::Map;
pub use set::Set;

use std::ops::{Range, RangeFrom, RangeFull, RangeTo};

/// A marker that indicates that a collection supports the mutation of its items.
pub trait Mutate {}

/// A marker that indicates that a collection supports the insertion of new items and the removal
/// of existing items.
pub trait AddRemove {}

/// A collection.
///
/// A collection maintains a finite number of items.
pub trait Collection {
    /// The type of the collection's items.
    type Item;

    /// Returns the number of items in the collection.
    fn len(&self) -> usize;

    /// Checks if the collection contains no items.
    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns the number of items the collection can hold without reallocating.
    ///
    /// Node-based collections should report a capacity of `self.len()`.
    fn capacity(&self) -> usize;

    /// Drains the given collection and inserts its items into the collection.
    ///
    /// The exact behavior of this method is unspecified, but it must be equivalent to
    /// `self.extend_object(&mut other.drain())`. `other`'s capacity should remain the same, when
    /// possible.
    fn append(&mut self, other: &mut Self) where Self: Sized + AddRemove {
        self.extend_object(&mut other.drain());
    }

    /// Inserts the items yielded by the given iterator into the collection.
    ///
    /// This method is provided for use with trait objects, and generic code should prefer
    /// [`Extend::extend`], which this method must be equivalent to.
    ///
    /// The exact behavior of this method is unspecified, but may be refined by subtraits.
    ///
    /// Note that this trait cannot extend `Extend` due to object-safety limitations.
    ///
    /// [`Extend::extend`]:
    ///     https://doc.rust-lang.org/stable/std/iter/trait.Extend.html#tymethod.extend
    fn extend_object(&mut self, items: &mut Iterator<Item = Self::Item>) where Self: AddRemove;

    /// Removes all items from the collection.
    fn clear(&mut self) where Self: AddRemove {
        self.drain();
    }

    /// Removes all items from the collection and returns an iterator that yields them.
    ///
    /// All items are removed even if the iterator is not exhausted. However, the behavior of
    /// this method is unspecified if the iterator is leaked (e.g. via [`mem::forget`]).
    ///
    /// The iteration order is unspecified, but subtraits may place a requirement on it.
    ///
    /// `self`'s capacity should remain the same, when possible.
    ///
    /// [`mem::forget`]: https://doc.rust-lang.org/stable/std/mem/fn.forget.html
    fn drain<'a>(&'a mut self) -> Box<Iterator<Item = Self::Item> + 'a> where Self: AddRemove;

    /// Reserves capacity for the given number of additional items to be inserted into the
    /// collection.
    ///
    /// This method may do nothing (e.g. for node-based collections).
    fn reserve(&mut self, additional: usize) where Self: AddRemove;

    /// Shrinks the collection's capacity as much as possible.
    ///
    /// This method may do nothing (e.g. for node-based collections).
    fn shrink_to_fit(&mut self) where Self: AddRemove;
}

/// A collection that supports by-reference iteration.
///
/// Maps are not expected to implement this interface, because they often provide
/// `Iterator<Item = (&Self::Key, &Self::Value)>`.
pub trait Iter: Collection {
    /// Returns an iterator that yields references to the collection's items.
    ///
    /// The iteration order is unspecified, but subtraits may place a requirement on it.
    fn iter<'a>(&'a self) -> Box<Iterator<Item = &'a Self::Item> + 'a>;

    /// Returns an iterator that yields mutable references to the collection's items.
    ///
    /// The iteration order is unspecified, but subtraits may place a requirement on it.
    fn iter_mut<'a>(&'a mut self) -> Box<Iterator<Item = &'a mut Self::Item> + 'a>
        where Self: Mutate;
}

/// A collection that supports draining a range of its items.
pub trait DrainRange<R>: Collection {
    /// Removes all items from the collection that lie in the given range and returns an iterator
    /// that yields them.
    ///
    /// All items in the given range are removed even if the iterator is not exhausted.  However,
    /// the behavior of this method is unspecified if the iterator is leaked (e.g. via
    /// [`mem::forget`]).
    ///
    /// The iteration order is unspecified, but subtraits may place a requirement on it.
    ///
    /// [`mem::forget`]: https://doc.rust-lang.org/stable/std/mem/fn.forget.html
    fn drain_range<'a>(&'a mut self, range: R) -> Box<Iterator<Item = Self::Item> + 'a>
        where Self: AddRemove;
}

/// A list.
///
/// A list is an ordered collection in which each item is located at a corresponding index. The
/// indices are non-negative integers and zero-based.
pub trait List:
    Collection +
    Iter +
    DrainRange<Range<usize>> +
    DrainRange<RangeFrom<usize>> +
    DrainRange<RangeTo<usize>> +
    DrainRange<RangeFull>
{
    /// Returns a reference to the item at the given index in the list.
    ///
    /// Returns `None` if `index >= self.len()`.
    fn get(&self, index: usize) -> Option<&Self::Item>;

    /// Returns a mutable reference to the item at the given index in the list.
    ///
    /// Returns `None` if `index >= self.len()`.
    fn get_mut(&mut self, index: usize) -> Option<&mut Self::Item> where Self: Mutate;

    /// Swaps the items at the given indices in the list.
    ///
    /// # Panics
    ///
    /// Panics if `i >= self.len() || j >= self.len()`.
    fn swap(&mut self, i: usize, j: usize) where Self: Mutate;

    /// Reverses the order of the items in the list.
    fn reverse(&mut self) where Self: Mutate {
        let len = self.len();

        for i in 0..len / 2 {
            self.swap(i, len - i - 1);
        }
    }

    /// Returns a reference to the first item in the list.
    ///
    /// Returns `None` if the list is empty.
    fn first(&self) -> Option<&Self::Item> {
        self.get(0)
    }

    /// Returns a mutable reference to the first item in the list.
    ///
    /// Returns `None` if the list is empty.
    fn first_mut(&mut self) -> Option<&mut Self::Item> where Self: Mutate {
        self.get_mut(0)
    }

    /// Returns a reference to the last item in the list.
    ///
    /// Returns `None` if the list is empty.
    fn last(&self) -> Option<&Self::Item> {
        self.get(self.len().wrapping_sub(1))
    }

    /// Returns a mutable reference to the last item in the list.
    ///
    /// Returns `None` if the list is empty.
    fn last_mut(&mut self) -> Option<&mut Self::Item> where Self: Mutate {
        let len = self.len();
        self.get_mut(len.wrapping_sub(1))
    }

    /// Pushes the given item onto the back of the list.
    fn push(&mut self, item: Self::Item) where Self: AddRemove {
        let len = self.len();
        self.insert(len, item);
    }

    /// Inserts the given item into the list at the given index.
    ///
    /// All items after the given index are shifted one index to the right.
    ///
    /// # Panics
    ///
    /// Panics if `index > self.len()`.
    fn insert(&mut self, index: usize, item: Self::Item) where Self: AddRemove;

    /// Removes the last item in the list and returns it.
    ///
    /// Returns `None` if the list was empty.
    fn pop(&mut self) -> Option<Self::Item> where Self: AddRemove {
        let len = self.len();
        self.remove(len.wrapping_sub(1))
    }

    /// Removes the item at the given index in the list and returns it.
    ///
    /// Returns `None` if `index >= self.len()`.
    ///
    /// All items after the given index are shifted one index to the left.
    fn remove(&mut self, index: usize) -> Option<Self::Item> where Self: AddRemove;

    /// Removes the item at the given index in the list and returns it, replacing it with the last
    /// item in the list.
    ///
    /// Returns `None` if `index >= self.len()`.
    fn swap_remove(&mut self, index: usize) -> Option<Self::Item> where Self: AddRemove;

    /// Ensures that the list's length is no more than the given length by removing the
    /// corresponding number of items from the back.
    ///
    /// Does nothing if `len >= self.len()`.
    fn truncate(&mut self, len: usize) where Self: AddRemove {
        if len == 0 {
            self.clear();
        } else {
            self.drain_range(len..);
        }
    }

    /// Splits the list in two at the given index.
    ///
    /// Returns a new list that contains the items in the range `index..self.len()`.
    ///
    /// After this method returns, `self` contains the items in the range `0..index`. `self`'s
    /// capacity should remain the same, when possible.
    ///
    /// # Panics
    ///
    /// Panics if `index > self.len()`.
    // FIXME(rust-lang/rust#20021): this shouldn't be defaulted
    fn split_off(&mut self, index: usize) -> Self where Self: Sized + AddRemove {
        let _ = index;
        unimplemented!()
    }
}

impl<L: ?Sized + List> DrainRange<RangeFrom<usize>> for L {
    fn drain_range<'a>(&'a mut self, range: RangeFrom<usize>) -> Box<Iterator<Item = L::Item> + 'a>
        where L: AddRemove
    {
        let len = self.len();
        self.drain_range(range.start..len)
    }
}

impl<L: ?Sized + List> DrainRange<RangeTo<usize>> for L {
    fn drain_range<'a>(&'a mut self, range: RangeTo<usize>) -> Box<Iterator<Item = L::Item> + 'a>
        where L: AddRemove
    {
        self.drain_range(0..range.end)
    }
}

impl<L: ?Sized + List> DrainRange<RangeFull> for L {
    fn drain_range<'a>(&'a mut self, _range: RangeFull) -> Box<Iterator<Item = L::Item> + 'a>
        where L: AddRemove
    {
        self.drain()
    }
}

pub mod map {
    //! Maps.

    use super::*;

    /// Map functionality that is independent of an additional type parameter.
    ///
    /// It is unusual to use this trait directly. Consider using [`Map`] instead.
    ///
    /// This trait exists to prevent the ambiguity that would arise if its methods were instead
    /// implemented on [`Map`]. In that scenario, `map.insert(key, value)` would be ambiguous if
    /// the type of `map` were `M` and `M: Map<Q> + Map<R>`.
    ///
    /// [`Map`]: trait.Map.html
    pub trait Base: Collection<Item = (<Self as Base>::Key, <Self as Base>::Value)> {
        /// The type of the map's keys.
        type Key;

        /// The type of the map's values.
        type Value;

        /// Returns an iterator that yields references to the map's keys and references to their
        /// values.
        ///
        /// The iteration order is unspecified, but subtraits may place a requirement on it.
        fn iter<'a>(&'a self) -> Box<Iterator<Item = (&'a Self::Key, &'a Self::Value)> + 'a>;

        /// Returns an iterator that yields references to the map's keys and mutable references to
        /// their values.
        ///
        /// The iteration order is unspecified, but subtraits may place a requirement on it.
        fn iter_mut<'a>(&'a mut self)
            -> Box<Iterator<Item = (&'a Self::Key, &'a mut Self::Value)> + 'a> where Self: Mutate;

        /// Inserts the given key and value into the map without replacing an equivalent key.
        ///
        /// If the map contains a key that is equivalent to the given key, that key is not replaced
        /// with the given key. The value is always replaced, however.
        ///
        /// Returns the equivalent key's value if the map contained one, `None` otherwise.
        fn insert(&mut self, key: Self::Key, value: Self::Value) -> Option<Self::Value>
            where Self: AddRemove;

        /// Returns the entry in the map for the given key.
        fn entry<'a>(&'a mut self, key: Self::Key) -> Entry<'a, Self::Key, Self::Value>
            where Self: AddRemove;
    }

    /// A map.
    ///
    /// A map is a set of keys, each of which is associated with a value.
    ///
    /// The type parameter `Q` represents an "equivalence" type that can be used to look up values
    /// in the map. For example, given a `Map<Key = String>`, it is usually possible to look up
    /// items using a `str`. When omitted, `Q` defaults to `Self::Key`.
    pub trait Map<Q: ?Sized = <Self as Base>::Key>: Base {
        /// Checks if the map contains a key that is equivalent to the given key.
        fn contains_key(&self, key: &Q) -> bool {
            self.get(key).is_some()
        }

        /// Returns a reference to the value of the key in the map that is equivalent to the given
        /// key.
        ///
        /// Returns `None` if the map contains no such key.
        fn get(&self, key: &Q) -> Option<&Self::Value>;

        /// Returns a mutable reference to the value of the key in the map that is equivalent to
        /// the given key.
        ///
        /// Returns `None` if the map contains no such key.
        fn get_mut(&mut self, key: &Q) -> Option<&mut Self::Value> where Self: Mutate;

        /// Removes the key in the map that is equivalent to the given key and returns its value.
        ///
        /// Returns `None` if the map contained no such key.
        fn remove(&mut self, key: &Q) -> Option<Self::Value> where Self: AddRemove;
    }

    /// A map entry.
    pub enum Entry<'a, K: 'a, V: 'a> {
        /// An occupied map entry.
        Occupied(Box<OccupiedEntry<Key = K, Value = V, MutValue = &'a mut V> + 'a>),
        /// A vacant map entry.
        Vacant(Box<VacantEntry<Key = K, Value = V, MutValue = &'a mut V> + 'a>),
    }

    impl<'a, K: 'a, V: 'a> Entry<'a, K, V> {
        /// Ensures that the entry is occupied by inserting it into the map with the given value if
        /// it is vacant.
        ///
        /// Returns a mutable reference to the entry's value with the same lifetime as the map.
        pub fn or_insert(self, value: V) -> &'a mut V {
            match self {
                Entry::Occupied(e) => e.into_mut(),
                Entry::Vacant(e) => e.insert(value),
            }
        }

        /// Ensures that the entry is occupied by inserting it into the map with the result of the
        /// given function if it is vacant.
        ///
        /// Returns a mutable reference to the entry's value with the same lifetime as the map.
        pub fn or_insert_with<F: FnOnce() -> V>(self, f: F) -> &'a mut V {
            match self {
                Entry::Occupied(e) => e.into_mut(),
                Entry::Vacant(e) => e.insert(f()),
            }
        }
    }

    /// An occupied map entry.
    pub trait OccupiedEntry {
        /// The type of the entry's key.
        type Key;

        /// The type of the entry's value.
        type Value;

        /// The type of the mutable reference to the entry's value with the same lifetime as the
        /// map.
        type MutValue;

        /// Returns a reference to the entry's value.
        fn get(&self) -> &Self::Value;

        /// Returns a mutable reference to the entry's value.
        fn get_mut(&mut self) -> &mut Self::Value;

        /// Returns a mutable reference to the entry's value with the same lifetime as the map.
        fn into_mut(self: Box<Self>) -> Self::MutValue;

        /// Removes the entry from the map and returns its value.
        fn remove(self: Box<Self>) -> Self::Value;
    }

    /// A vacant entry.
    pub trait VacantEntry {
        /// The type of the entry's key.
        type Key;

        /// The type of the entry's value.
        type Value;

        /// The type of the mutable reference to the entry's value with the same lifetime as the
        /// map.
        type MutValue;

        /// Inserts the entry into the map with the given value.
        ///
        /// Returns a mutable reference to the entry's value with the same lifetime as the map.
        fn insert(self: Box<Self>, value: Self::Value) -> Self::MutValue;
    }
}

pub mod set {
    //! Sets.

    use super::*;

    /// Set functionality that is independent of an additional type parameter.
    ///
    /// It is unusual to use this trait directly. Consider using [`Set`] instead.
    ///
    /// This trait exists to prevent the ambiguity that would arise if its methods were instead
    /// implemented on [`Set`]. In that scenario, `set.insert(item)` would be ambiguous if the type
    /// of `set` were `S` and `S: Set<Q> + Set<R>`.
    ///
    /// [`Set`]: trait.Set.html
    pub trait Base: Collection + Iter {
        /// Checks if the set is disjoint from the given set.
        ///
        /// `self` is disjoint from `other` if `self` contains none of `other`'s items.
        fn is_disjoint(&self, other: &Self) -> bool where Self: Sized;

        /// Checks if the set is a subset of the given set.
        ///
        /// `self` is a subset of `other` if `other` contains all of `self`'s items.
        fn is_subset(&self, other: &Self) -> bool where Self: Sized;

        /// Checks if the set is a superset of the given set.
        ///
        /// `self` is a superset of `other` if `self` contains all of `other`'s items.
        fn is_superset(&self, other: &Self) -> bool where Self: Sized {
            other.is_subset(self)
        }

        /// Inserts the given item into the set without replacement.
        ///
        /// If the set contains an item that is equivalent to the given item, that item is not
        /// replaced with the given item.
        ///
        /// Returns `true` if the given item was inserted into the set, `false` otherwise.
        fn insert(&mut self, item: Self::Item) -> bool where Self: AddRemove;

        /// Inserts the given item into the set with replacement.
        ///
        /// If the set contains an item that is equivalent to the given item, that item is replaced
        /// with the given item.
        ///
        /// Returns the item that was replaced, or `None` if the set did not contain an equivalent
        /// item.
        #[cfg(feature = "nightly")]
        fn replace(&mut self, item: Self::Item) -> Option<Self::Item> where Self: AddRemove;
    }

    /// A set.
    ///
    /// A set is a collection that prohibits duplicate items according to some criteria.
    ///
    /// The type parameter `Q` represents an "equivalence" type that can be used to look up items
    /// in the set. For example, given a `Set<Item = String>`, it is usually possible to look up
    /// items using a `str`. When omitted, `Q` defaults to `Self::Item`.
    pub trait Set<Q: ?Sized = <Self as Collection>::Item>: Base {
        /// Checks if the set contains an item that is equivalent to the given item.
        #[cfg(not(feature = "nightly"))]
        fn contains(&self, item: &Q) -> bool;

        /// Checks if the set contains an item that is equivalent to the given item.
        #[cfg(feature = "nightly")]
        fn contains(&self, item: &Q) -> bool {
            self.get(item).is_some()
        }

        /// Returns a reference to the item in the set that is equivalent to the given item.
        ///
        /// Returns `None` if the set contains no such item.
        #[cfg(feature = "nightly")]
        fn get(&self, item: &Q) -> Option<&Self::Item>;

        /// Removes the item in the set that is equivalent to the given item.
        ///
        /// Returns `true` if the set contained such an item, `false` otherwise.
        #[cfg(feature = "nightly")]
        fn remove(&mut self, item: &Q) -> bool where Self: AddRemove {
            self.take(item).is_some()
        }

        /// Removes the item in the set that is equivalent to the given item.
        ///
        /// Returns `true` if the set contained such an item, `false` otherwise.
        #[cfg(not(feature = "nightly"))]
        fn remove(&mut self, item: &Q) -> bool where Self: AddRemove;

        /// Removes the item in the set that is equivalent to the given item and returns it.
        ///
        /// Returns `None` if the set contained no such item.
        #[cfg(feature = "nightly")]
        fn take(&mut self, item: &Q) -> Option<Self::Item> where Self: AddRemove;
    }
}

/// A queue.
pub trait Queue: Collection + Iter {
    /// Pushes the given item onto the queue.
    ///
    /// For FIFO queues, this pushes the item onto the back of the queue. For other queues, the
    /// location of the newly inserted item is unspecified.
    fn push(&mut self, item: Self::Item) where Self: AddRemove;

    /// Returns a reference to the item at the front of the queue.
    ///
    /// Returns `None` if the queue is empty.
    fn front(&self) -> Option<&Self::Item>;

    /// Removes the item at the front of the queue and returns it.
    ///
    /// Returns `None` if the queue was empty.
    fn pop_front(&mut self) -> Option<Self::Item> where Self: AddRemove;
}

/// A first-in, first-out queue.
pub trait FifoQueue: Queue {
    /// Returns a mutable reference to the item at the front of the queue.
    ///
    /// Returns `None` if the queue is empty.
    fn front_mut(&mut self) -> Option<&mut Self::Item> where Self: Mutate;
}

/// A priority queue.
pub trait PrioQueue: Queue {
    /// Pushes the given item onto the queue, then removes the item at the front of the queue and
    /// returns it.
    fn push_pop_front(&mut self, item: Self::Item) -> Self::Item where Self: AddRemove {
        self.push(item);
        self.pop_front().expect("queue was empty after a `push`")
    }

    /// Removes the item at the front of the queue, then pushes the given item onto the queue.
    ///
    /// Returns the item that was removed, or `None` if the queue was empty.
    fn replace_front(&mut self, item: Self::Item) -> Option<Self::Item> where Self: AddRemove {
        let front = self.pop_front();
        self.push(item);
        front
    }
}

/// A double-ended queue.
pub trait Deque: Queue {
    /// Returns a reference to the item at the back of the deque.
    ///
    /// Returns `None` if the deque is empty.
    fn back(&self) -> Option<&Self::Item>;

    /// Removes the item at the back of the deque and returns it.
    ///
    /// Returns `None` if the deque was empty.
    fn pop_back(&mut self) -> Option<Self::Item> where Self: AddRemove;
}

/// A double-ended first-in, first-out queue.
pub trait FifoDeque: FifoQueue + Deque {
    /// Pushes the given item onto the front of the deque.
    fn push_front(&mut self, item: Self::Item) where Self: AddRemove;

    /// Returns a mutable reference to the item at the back of the deque.
    ///
    /// Returns `None` if the deque is empty.
    fn back_mut(&mut self) -> Option<&mut Self::Item> where Self: Mutate;
}

/// A double-ended priority queue.
pub trait PrioDeque: PrioQueue + Deque {
    /// Pushes the given item onto the deque, then removes the item at the back of the deque and
    /// returns it.
    fn push_pop_back(&mut self, item: Self::Item) -> Self::Item where Self: AddRemove {
        self.push(item);
        self.pop_back().expect("deque was empty after a `push`")
    }

    /// Removes the item at the back of the deque, then pushes the given item onto the deque.
    ///
    /// Returns the item that was removed, or `None` if the deque was empty.
    fn replace_back(&mut self, item: Self::Item) -> Option<Self::Item> where Self: AddRemove {
        let back = self.pop_back();
        self.push(item);
        back
    }
}

#[allow(dead_code)]
fn assert_object_safe() {
    let _: &Mutate;
    let _: &AddRemove;

    let _: &Collection<Item = String>;

    let _: &List<Item = String>;

    let _: &Map<str, Item = (String, i32), Key = String, Value = i32>;
    let _: &map::OccupiedEntry<Key = String, Value = i32, MutValue = &mut i32>;
    let _: &map::VacantEntry<Key = String, Value = i32, MutValue = &mut i32>;

    let _: &Set<str, Item = String>;

    let _: &Queue<Item = String>;
    let _: &Deque<Item = String>;

    let _: &FifoQueue<Item = String>;
    let _: &FifoDeque<Item = String>;

    let _: &PrioQueue<Item = String>;
    let _: &PrioDeque<Item = String>;
}
