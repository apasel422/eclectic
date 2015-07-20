//! Collection traits for generic programming.
//!
//! Code that uses these traits should use them as generally as possible. For example, consider the
//! following function that fills a map with default values for some keys:
//!
//! ```
//! use eclectic::map::*;
//!
//! fn fill<M, K>(keys: K) -> M
//! where
//!     M: Default + Map,
//!     K: IntoIterator<Item = M::Key>,
//!     M::Value: Default,
//! {
//!     let mut map = M::default();
//!     for key in keys { map.insert(key, M::Value::default()); }
//!     map
//! }
//! ```
//!
//! `Map` is a convenient shorthand for code that uses multiple map APIs, but this code only ever
//! calls `insert`, so it could be rewritten more generally as:
//!
//! ```
//! use eclectic::map;
//!
//! fn fill<M, K>(keys: K) -> M
//! where
//!     M: Default + map::Insert,
//!     K: IntoIterator<Item = M::Key>,
//!     M::Value: Default,
//! {
//!     let mut map = M::default();
//!     for key in keys { map.insert(key, M::Value::default()); }
//!     map
//! }
//! ```

#[cfg(feature = "linear-map")]
mod linear_map_impls;

#[cfg(feature = "linked-hash-map")]
mod linked_hash_map_impls;

#[cfg(feature = "std_impls")]
mod std_impls;

#[cfg(feature = "trie")]
mod trie_impls;

#[cfg(feature = "vec_map")]
mod vec_map_impls;

/// A collection that contains a finite number of items.
pub trait Len {
    /// Checks if the collection is empty.
    fn is_empty(&self) -> bool { self.len() == 0 }

    /// Returns the number of items in the collection.
    fn len(&self) -> usize;
}

/// A collection that can be cleared.
pub trait Clear {
    /// Removes all items from the collection.
    fn clear(&mut self);
}

pub mod map {
    //! Maps.
    //!
    //! A map is a collection that associates keys with corresponding values, where the keys are
    //! distinguished according to some uniqueness criteria.

    use super::*;

    /// A trait that declares a map's key and value types.
    ///
    /// It is unusual to bound a type by this trait directly.
    pub trait Base {
        /// The map's key type.
        type Key;

        /// The map's value type.
        type Value;
    }

    /// A map that supports lookups using keys of type `&Q`.
    pub trait Get<Q: ?Sized>: Base {
        /// Checks if the map contains the given key.
        fn contains_key(&self, key: &Q) -> bool { self.get(key).is_some() }

        /// Returns a reference to the value in the map corresponding to the given key.
        fn get(&self, key: &Q) -> Option<&Self::Value>;
    }

    /// A map that supports mutable lookups using keys of type `&Q`.
    pub trait GetMut<Q: ?Sized>: Get<Q> {
        /// Returns a mutable reference to the value in the map corresponding to the given key.
        fn get_mut(&mut self, key: &Q) -> Option<&mut Self::Value>;
    }

    /// A map that supports removals using keys of type `&Q`.
    pub trait Remove<Q: ?Sized>: GetMut<Q> {
        /// Removes the given key from them map and returns its corresponding value, if any.
        fn remove(&mut self, key: &Q) -> Option<Self::Value>;
    }

    /// A map that supports insertion.
    pub trait Insert: Base + Extend<(<Self as Base>::Key, <Self as Base>::Value)> {
        /// Inserts the given key and value into the map and returns the previous value
        /// corresponding to the given key, if any.
        // TODO: document replacement requirements
        fn insert(&mut self, key: Self::Key, value: Self::Value) -> Option<Self::Value>;
    }

    /// A map.
    ///
    /// This trait is implemented for all types that implement `{Len, Clear, Insert, Remove}`.
    pub trait Map: Len + Clear + Insert + Remove<<Self as Base>::Key> {}

    impl<M: ?Sized> Map for M where M: Len + Clear + Insert + Remove<<M as Base>::Key> {}

    /// A map that supports efficient in-place manipulation.
    ///
    /// `'a` is the lifetime of the map.
    pub trait EntryMap<'a>: Base {
        /// The type of the map's occupied entries.
        type OccupiedEntry: OccupiedEntry<'a, Key = Self::Key, Value = Self::Value>;

        /// The type of the map's vacant entries.
        type VacantEntry: VacantEntry<'a, Key = Self::Key, Value = Self::Value>;

        /// Returns the entry in the map corresponding to the given key.
        fn entry(&'a mut self, key: Self::Key) -> Entry<Self::OccupiedEntry, Self::VacantEntry>;
    }

    /// An occupied map entry.
    ///
    /// `'a` is the lifetime of the map.
    pub trait OccupiedEntry<'a> {
        /// The type of the map's keys.
        type Key: 'a;

        /// The type of the map's values.
        type Value: 'a;

        /// Returns a reference to the entry's value.
        fn get(&self) -> &Self::Value;

        /// Returns a mutable reference to the entry's value.
        fn get_mut(&mut self) -> &mut Self::Value;

        /// Returns a mutable reference to the entry's value with the same lifetime as the map.
        fn into_mut(self) -> &'a mut Self::Value;

        /// Replaces the entry's value with the given one and returns the old value.
        fn insert(&mut self, value: Self::Value) -> Self::Value {
            ::std::mem::replace(self.get_mut(), value)
        }

        /// Removes the entry from the map and returns its value.
        fn remove(self) -> Self::Value;
    }

    /// A vacant map entry.
    ///
    /// `'a` is the lifetime of the map.
    pub trait VacantEntry<'a> {
        /// The type of the map's keys.
        type Key: 'a;

        /// The type of the map's values.
        type Value: 'a;

        /// Inserts the entry into the map with the given value and returns a mutable reference to
        /// it with the same lifetime as the map.
        fn insert(self, value: Self::Value) -> &'a mut Self::Value;
    }

    /// A map entry.
    #[derive(Debug)]
    pub enum Entry<OE, VE> {
        /// An occupied map entry.
        Occupied(OE),
        /// A vacant map entry.
        Vacant(VE),
    }

    impl<'a, OE, VE> Entry<OE, VE>
    where
        OE: OccupiedEntry<'a>,
        VE: VacantEntry<'a, Value = OE::Value>,
    {
        /// Ensures the entry is occupied by inserting it with the given default value if it is
        /// vacant.
        pub fn or_insert(self, default: VE::Value) -> &'a mut OE::Value {
            match self {
                Entry::Occupied(e) => e.into_mut(),
                Entry::Vacant(e) => e.insert(default),
            }
        }

        /// Ensures the entry is occupied by inserting it with the result of the given function if
        /// it is vacant.
        pub fn or_insert_with<F>(self, f: F) -> &'a mut OE::Value where F: FnOnce() -> VE::Value {
            match self {
                Entry::Occupied(e) => e.into_mut(),
                Entry::Vacant(e) => e.insert(f()),
            }
        }
    }

    #[cfg(test)]
    pub fn count<M, I>(items: I) -> M
    where
        M: Default + for<'a> map::EntryMap<'a, Value = usize>,
        I: IntoIterator<Item = M::Key>,
    {
        let mut map = M::default();
        for item in items { *map.entry(item).or_insert(0) += 1; }
        map
    }
}

pub mod set {
    //! Sets.
    //!
    //! A set is a collection whose items are distinguished according to some uniqueness criteria.

    use super::*;

    /// A trait that declares a set's item type.
    ///
    /// It is unusual to bound a type by this trait directly.
    pub trait Base {
        type Item;
    }

    /// A set that supports lookups using items of type `&Q`.
    pub trait Contains<Q: ?Sized>: Base {
        /// Checks if the set contains the given item.
        fn contains(&self, item: &Q) -> bool;
    }

    /// A set that supports removals using items of type `&Q`.
    pub trait Remove<Q: ?Sized>: Contains<Q> {
        /// Removes the given item from the set and returns `true` if the set contained it.
        fn remove(&mut self, item: &Q) -> bool;
    }

    /// A set that supports insertion.
    pub trait Insert: Base + Extend<<Self as Base>::Item> {
        /// Inserts the given item into the set and returns `true` if the set did not contain it.
        // TODO: document replacement requirements
        fn insert(&mut self, item: Self::Item) -> bool;
    }

    /// A set.
    ///
    /// This trait is implemented for all types that implement `{Len, Clear, Insert, Remove}`.
    pub trait Set: Len + Clear + Insert + Remove<<Self as Base>::Item> {}

    impl<S: ?Sized> Set for S where S: Len + Clear + Insert + Remove<<S as Base>::Item> {}
}
