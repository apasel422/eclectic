//! Collection traits for generic programming.

#![deny(missing_docs)]
#![feature(deque_extras)]
#![feature(linked_list_extras)]
#![feature(set_recovery)]

mod std_impls;

pub use collection::Collection;
pub use list::List;
pub use map::Map;
pub use seq::{Deque, Queue, Stack};
pub use set::Set;

pub mod collection {
    //! Collections.

    /// A collection.
    pub trait Collection {
        /// The type of the collection's items.
        type Item;

        /// Checks if the collection contains zero items.
        ///
        /// This is equivalent to `self.len() == 0`, but may be optimized.
        fn is_empty(&self) -> bool {
            self.len() == 0
        }

        /// Returns the number of items in the collection.
        fn len(&self) -> usize;
    }

    /// A collection that supports insertion.
    pub trait Insert: Collection {
        /// Moves all items from the given collection into the collection.
        fn append(&mut self, other: &mut Self) where Self: Sized;
    }

    /// A collection that supports removal.
    pub trait Remove: Collection {
        /// Removes all items from the collection.
        fn clear(&mut self);
    }
}

pub mod list {
    //! Lists.

    use super::*;

    /// A list.
    pub trait List: Collection {
        /// Returns a reference to the item at the given index in the list.
        ///
        /// Returns `None` if `index >= self.len()`.
        fn get(&self, index: usize) -> Option<&Self::Item>;

        /// Returns a mutable reference to the item at the given index in the list.
        ///
        /// Returns `None` if `index >= self.len()`.
        fn get_mut(&mut self, index: usize) -> Option<&mut Self::Item>;

        /// Swaps the items at the given indices in the list.
        ///
        /// # Panics
        ///
        /// Panics if `i >= self.len() || j >= self.len()`.
        fn swap(&mut self, i: usize, j: usize);
    }

    /// A list that supports insertion.
    pub trait Insert: List {
        /// Inserts the given item into the list at the given index.
        ///
        /// All items after the given index are shifted one index to the right.
        ///
        /// # Panics
        ///
        /// Panics if `index > self.len()`.
        fn insert(&mut self, index: usize, item: Self::Item);

        /// Pushes the given item onto the back of the list.
        ///
        /// This is equivalent to `self.insert(self.len(), item)`, but may be optimized.
        fn push(&mut self, item: Self::Item) {
            let len = self.len();
            self.insert(len, item);
        }
    }

    /// A list that supports removal.
    pub trait Remove: List {
        /// Removes the last item in the list and returns it.
        ///
        /// Returns `None` if the list is empty.
        ///
        /// This is equivalent to `self.remove(self.len() - 1)`, but may be optimized.
        fn pop(&mut self) -> Option<Self::Item> {
            match self.len() {
                0 => None,
                len => self.remove(len - 1),
            }
        }

        /// Removes the item at the given index in the list and returns it.
        ///
        /// Returns `None` if `index >= self.len()`.
        fn remove(&mut self, index: usize) -> Option<Self::Item>;

        /// Splits the list in two at the given index.
        ///
        /// After this method returns, `self` contains the items `0..index` and the returned list
        /// contains the items `index..self.len()`.
        ///
        /// # Panics
        ///
        /// Panics if `index > self.len()`.
        fn split_off(&mut self, index: usize) -> Self where Self: Sized;

        /// Removes the item at the given index in the list and returns it, replacing it with the
        /// last item in the list.
        ///
        /// Returns `None` if `index >= self.len()`.
        fn swap_remove(&mut self, index: usize) -> Option<Self::Item>;

        /// Removes all items in the list starting at the given index.
        ///
        /// This is equivalent to calling `self.pop()` the corresponding number of times, but may
        /// be optimized.
        fn truncate(&mut self, index: usize);
    }
}

pub mod map {
    //! Maps.

    use super::*;

    /// A map.
    ///
    /// A map is a collection that associates keys with values, where the keys are distinguished
    /// according to some uniqueness criteria.
    pub trait Map: Collection<Item = (<Self as Map>::Key, <Self as Map>::Value)> {
        /// The type of the map's keys.
        type Key;

        /// The type of the map's values.
        type Value;

        /// Returns the entry in the map corresponding to the given key.
        fn entry<'a>(&'a mut self, key: Self::Key) -> Entry<'a, Self::Key, Self::Value>
            where Self: Insert + Remove;
    }

    /// A map that supports retrievals using keys of type `&Q`.
    pub trait Get<Q: ?Sized = <Self as Map>::Key>: Map {
        /// Checks if the map contains a key that is equivalent to the given key.
        ///
        /// This is equivalent to `self.get(key).is_some()`, but may be optimized.
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
        fn get_mut(&mut self, key: &Q) -> Option<&mut Self::Value>;
    }

    /// A map that supports removals using keys of type `&Q`.
    pub trait Remove<Q: ?Sized = <Self as Map>::Key>: Get<Q> + collection::Remove {
        /// Removes the key in the map that is equivalent to the given key and returns its value.
        ///
        /// Returns `None` if the map contains no such key.
        fn remove(&mut self, key: &Q) -> Option<Self::Value>;
    }

    /// A map that supports insertion.
    pub trait Insert: Map + collection::Insert {
        /// Inserts the given key and value into the map without replacing an equivalent key.
        ///
        /// Returns the equivalent key's value if the map contained one, `None` otherwise.
        fn insert(&mut self, key: Self::Key, value: Self::Value) -> Option<Self::Value>;
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

        /// Replaces the entry's value with the given one and returns the old value.
        fn insert(&mut self, value: Self::Value) -> Self::Value {
            ::std::mem::replace(self.get_mut(), value)
        }

        /// Removes the entry from the map and returns its value.
        fn remove(self: Box<Self>) -> Self::Value;
    }

    /// A vacant map entry.
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
        /// Returns a mutable reference to the value with the same lifetime as the map.
        fn insert(self: Box<Self>, value: Self::Value) -> Self::MutValue;
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
        /// Returns a mutable reference to the value with the same lifetime as the map.
        pub fn or_insert(self, default: V) -> &'a mut V {
            match self {
                Entry::Occupied(e) => e.into_mut(),
                Entry::Vacant(e) => e.insert(default),
            }
        }

        /// Ensures that the entry is occupied by inserting it into the map with the result of the
        /// given function if it is vacant.
        ///
        /// Returns a mutable reference to the value with the same lifetime as the map.
        pub fn or_insert_with<F: FnOnce() -> V>(self, f: F) -> &'a mut V {
            match self {
                Entry::Occupied(e) => e.into_mut(),
                Entry::Vacant(e) => e.insert(f()),
            }
        }
    }

    #[cfg(test)]
    pub fn count<M, I>(items: I) -> M
    where
        M: Default + map::Insert<Value = usize> + map::Remove,
        I: IntoIterator<Item = M::Key>,
    {
        let mut map = M::default();
        for item in items { *map.entry(item).or_insert(0) += 1; }
        map
    }
}

pub mod seq {
    //! Sequences.

    use super::*;

    /// A sequence that supports insertion.
    pub trait PushBack: collection::Insert {
        /// Pushes the given item onto the back of the sequence.
        fn push_back(&mut self, item: Self::Item);
    }

    /// A queue.
    pub trait Queue: PushBack + collection::Remove {
        /// Returns a reference to the item at the front of the queue.
        ///
        /// Returns `None` if the queue is empty.
        fn front(&self) -> Option<&Self::Item>;

        /// Returns a mutable reference to the item at the front of the queue.
        ///
        /// Returns `None` if the queue is empty.
        fn front_mut(&mut self) -> Option<&mut Self::Item>;

        /// Removes the item at the front of the queue and returns it.
        ///
        /// Returns `None` if the queue is empty.
        fn pop_front(&mut self) -> Option<Self::Item>;
    }

    /// A stack.
    pub trait Stack: PushBack + collection::Remove {
        /// Returns a reference to the item at the back of the stack.
        ///
        /// Returns `None` if the stack is empty.
        fn back(&self) -> Option<&Self::Item>;

        /// Returns a mutable reference to the item at the back of the stack.
        ///
        /// Returns `None` if the stack is empty.
        fn back_mut(&mut self) -> Option<&mut Self::Item>;

        /// Removes the item at the back of the stack and returns it.
        ///
        /// Returns `None` if the stack is empty.
        fn pop_back(&mut self) -> Option<Self::Item>;
    }

    /// A deque (double-ended queue).
    pub trait Deque: Queue + Stack {
        /// Pushes the given item onto the front of the deque.
        fn push_front(&mut self, item: Self::Item);
    }
}

pub mod set {
    //! Sets.

    use super::*;

    /// A set.
    ///
    /// A set is a collection whose items are distinguished according to some uniqueness criteria.
    pub trait Set: Collection {
        /// Checks if the set is disjoint from the given set.
        fn is_disjoint(&self, other: &Self) -> bool where Self: Sized;

        /// Checks if the set is a subset of the given set.
        fn is_subset(&self, other: &Self) -> bool where Self: Sized;

        /// Checks if the set is a superset of the given set.
        fn is_superset(&self, other: &Self) -> bool where Self: Sized {
            other.is_subset(self)
        }
    }

    /// A set that supports retrievals using items of type `&Q`.
    pub trait Get<Q: ?Sized = <Self as Collection>::Item>: Set {
        /// Checks if the set contains an item that is equivalent to the given item.
        ///
        /// This is equivalent to `self.get(item).is_some()`, but may be optimized.
        fn contains(&self, item: &Q) -> bool {
            self.get(item).is_some()
        }

        /// Returns a reference to the item in the set that is equivalent to the given item.
        ///
        /// Returns `None` if the set contains no such item.
        fn get(&self, item: &Q) -> Option<&Self::Item>;
    }

    /// A set that supports removal using items of type `&Q`.
    pub trait Remove<Q: ?Sized = <Self as Collection>::Item>: Get<Q> + collection::Remove {
        /// Removes the item in the set that is equivalent to the given item.
        ///
        /// Returns `true` if the set contained such an item, `false` otherwise.
        ///
        /// This is equivalent to `self.take(item).is_some()`, but may be optimized.
        fn remove(&mut self, item: &Q) -> bool {
            self.take(item).is_some()
        }

        /// Removes the item in the set that is equivalent to the given item and returns it.
        ///
        /// Returns `None` if the set contained no such item.
        fn take(&mut self, item: &Q) -> Option<Self::Item>;
    }

    /// A set that supports insertion.
    pub trait Insert: Set + collection::Insert {
        /// Inserts the given item into the set without replacing an equivalent item.
        ///
        /// Returns `true` if the set did not contain an equivalent item, `false` otherwise.
        fn insert(&mut self, item: Self::Item) -> bool;

        /// Inserts the given item into the set, replacing an equivalent item.
        ///
        /// Returns the equivalent item if the set contained one, `None` otherwise.
        fn replace(&mut self, item: Self::Item) -> Option<Self::Item>;
    }
}

#[allow(dead_code)]
fn assert_object_safe() {
    let _: Collection<Item = String>;
    let _: collection::Insert<Item = String>;
    let _: collection::Remove<Item = String>;

    let _: &List<Item = String>;
    let _: &list::Insert<Item = String>;
    let _: &list::Remove<Item = String>;

    let _: &Map<Item = (String, i32), Key = String, Value = i32>;
    let _: &map::Insert<Item = (String, i32), Key = String, Value = i32>;
    let _: &map::Get<str, Item = (String, i32), Key = String, Value = i32>;
    let _: &map::Remove<str, Item = (String, i32), Key = String, Value = i32>;
    let _: &map::OccupiedEntry<Key = String, Value = i32, MutValue = &mut i32>;
    let _: &map::VacantEntry<Key = String, Value = i32, MutValue = &mut i32>;

    let _: &seq::PushBack<Item = String>;
    let _: &Queue<Item = String>;
    let _: &Stack<Item = String>;
    let _: &Deque<Item = String>;

    let _: &Set<Item = String>;
    let _: &set::Insert<Item = String>;
    let _: &set::Get<str, Item = String>;
    let _: &set::Remove<str, Item = String>;
}
