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

mod std_impls;

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

/// A collection that can be created.
///
/// All collections that implement `Default` should implement this trait and override its methods
/// where beneficial.
pub trait New: Sized + Default {
    /// Creates a new collection with the given capacity hint.
    ///
    /// Code that needs to create a collection should prefer this method to
    /// `<Self as Default>::default` when the capacity of the collection is known ahead of time.
    ///
    /// The default implementation calls `<Self as Default>::default`. Collections should override
    /// this method if they are able to make use of the capacity hint.
    fn with_capacity_hint(capacity: usize) -> Self {
        let _ = capacity;
        Self::default()
    }
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
    pub trait Get<Q: ?Sized = <Self as Base>::Key>: Base {
        /// Checks if the map contains the given key.
        fn contains_key(&self, key: &Q) -> bool { self.get(key).is_some() }

        /// Returns a reference to the value in the map corresponding to the given key.
        fn get(&self, key: &Q) -> Option<&Self::Value>;
    }

    /// A map that supports mutable lookups using keys of type `&Q`.
    pub trait GetMut<Q: ?Sized = <Self as Base>::Key>: Get<Q> {
        /// Returns a mutable reference to the value in the map corresponding to the given key.
        fn get_mut(&mut self, key: &Q) -> Option<&mut Self::Value>;
    }

    /// A map that supports removals using keys of type `&Q`.
    pub trait Remove<Q: ?Sized = <Self as Base>::Key>: GetMut<Q> {
        /// Removes the given key from the map and returns its corresponding value, if any.
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
    pub trait Map: Len + Clear + Insert + Remove {}

    impl<M: ?Sized> Map for M where M: Len + Clear + Insert + Remove {}

    /// A map that supports efficient in-place manipulation.
    ///
    /// `'a` is the lifetime of the map.
    pub trait EntryMap<'a>: Base {
        /// The type of the map's occupied entries.
        type OccupiedEntry: OccupiedEntry<'a, Map = Self>;

        /// The type of the map's vacant entries.
        type VacantEntry: VacantEntry<'a, Map = Self>;

        /// Returns the entry in the map corresponding to the given key.
        fn entry(&'a mut self, key: Self::Key) -> Entry<Self>;
    }

    /// An occupied map entry.
    ///
    /// `'a` is the lifetime of the map.
    pub trait OccupiedEntry<'a>: Sized {
        /// The entry's map type.
        type Map: ?Sized + EntryMap<'a, OccupiedEntry = Self>;

        /// Returns a reference to the entry's value.
        fn get(&self) -> &<Self::Map as Base>::Value;

        /// Returns a mutable reference to the entry's value.
        fn get_mut(&mut self) -> &mut <Self::Map as Base>::Value;

        /// Returns a mutable reference to the entry's value with the same lifetime as the map.
        fn into_mut(self) -> &'a mut <Self::Map as Base>::Value;

        /// Replaces the entry's value with the given one and returns the old value.
        fn insert(&mut self, value: <Self::Map as Base>::Value) -> <Self::Map as Base>::Value {
            ::std::mem::replace(self.get_mut(), value)
        }

        /// Removes the entry from the map and returns its value.
        fn remove(self) -> <Self::Map as Base>::Value;
    }

    /// A vacant map entry.
    ///
    /// `'a` is the lifetime of the map.
    pub trait VacantEntry<'a>: Sized {
        /// The entry's map type.
        type Map: ?Sized + EntryMap<'a, VacantEntry = Self>;

        /// Inserts the entry into the map with the given value and returns a mutable reference to
        /// it with the same lifetime as the map.
        fn insert(self, value: <Self::Map as Base>::Value) -> &'a mut <Self::Map as Base>::Value;
    }

    /// A map entry.
    #[derive(Debug)]
    pub enum Entry<'a, M: ?Sized> where M: EntryMap<'a> {
        /// An occupied map entry.
        Occupied(M::OccupiedEntry),
        /// A vacant map entry.
        Vacant(M::VacantEntry),
    }

    impl<'a, M: ?Sized> Entry<'a, M> where M: EntryMap<'a> {
        /// Ensures the entry is occupied by inserting it with the given default value if it is
        /// vacant.
        pub fn or_insert(self, default: M::Value) -> &'a mut M::Value {
            match self {
                Entry::Occupied(e) => e.into_mut(),
                Entry::Vacant(e) => e.insert(default),
            }
        }

        /// Ensures the entry is occupied by inserting it with the result of the given function if
        /// it is vacant.
        pub fn or_insert_with<F>(self, f: F) -> &'a mut M::Value where F: FnOnce() -> M::Value {
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
        /// The set's item type.
        type Item;
    }

    /// A set that supports lookups using items of type `&Q`.
    pub trait Contains<Q: ?Sized = <Self as Base>::Item>: Base {
        /// Checks if the set contains the given item.
        fn contains(&self, item: &Q) -> bool;
    }

    /// A set that supports removals using items of type `&Q`.
    pub trait Remove<Q: ?Sized = <Self as Base>::Item>: Contains<Q> {
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
    pub trait Set: Len + Clear + Insert + Remove {}

    impl<S: ?Sized> Set for S where S: Len + Clear + Insert + Remove {}
}
