//! Collection traits for generic programming.

#![forbid(missing_docs)]

#[macro_use] mod macros;
mod std_impls;

/// A collection.
pub trait Collection {
    /// Removes all items from the collection.
    ///
    /// After this method returns, `self.len() == 0`.
    fn clear(&mut self);

    /// Checks if the collection is empty.
    ///
    /// This method returns `true` iff `self.len() == 0`.
    fn is_empty(&self) -> bool { self.len() == 0 }

    /// Returns the number of items in the collection.
    ///
    /// This method returns `0` iff `self.is_empty()`.
    fn len(&self) -> usize;
}

/// A trait that declares a map's key and value types.
///
/// It is unusual to bound types by this trait directly. Consider using [`Map`](trait.Map.html)
/// instead.
pub trait BaseMap: Collection {
    /// The map's key type.
    type Key;

    /// The map's value type.
    type Value;
}

/// A map.
pub trait Map: MapLookup<<Self as BaseMap>::Key> {
    /// Inserts the given key and value into the map and returns the previous value, if any,
    /// corresponding to the given key.
    fn insert(&mut self, key: Self::Key, value: Self::Value) -> Option<Self::Value>;
}

/// A map that supports lookups using keys of type `&Q`.
pub trait MapLookup<Q: ?Sized>: BaseMap {
    /// Checks if the map contains the given key.
    fn contains_key(&self, key: &Q) -> bool { self.get(key).is_some() }

    /// Returns a reference to the value in the map, if any, corresponding to the given key.
    fn get(&self, key: &Q) -> Option<&Self::Value>;

    /// Returns a mutable reference to the value in the map, if any, corresponding to the given
    fn get_mut(&mut self, key: &Q) -> Option<&mut Self::Value>;

    /// Removes the given key from the map and returns its corresponding value, if any.
    fn remove(&mut self, key: &Q) -> Option<Self::Value>;
}

/// A map that supports efficient in-place manipulation of its entries.
pub trait EntryMap<'a>: Map {
    /// The map's occupied entry type.
    type Occupied: OccupiedEntry<'a, Key = Self::Key, Value = Self::Value>;

    /// The map's vacant entry type.
    type Vacant: VacantEntry<'a, Key = Self::Key, Value = Self::Value>;

    /// Returns the entry in the map corresponding to the given key.
    ///
    /// # Examples
    ///
    /// Count the unique items yielded by an iterator:
    ///
    /// ```
    /// use eclectic::EntryMap;
    ///
    /// fn count<I, M>(items: I) -> M
    /// where
    ///     M: Default + for<'a> EntryMap<'a, Value = usize>,
    ///     I: IntoIterator<Item = M::Key>,
    /// {
    ///     let mut counts = M::default();
    ///
    ///     for item in items {
    ///         *counts.entry(item).or_insert(0) += 1;
    ///     }
    ///
    ///     counts
    /// }
    /// ```
    ///
    /// Group values into a multimap according to keys yielded by an iterator:
    ///
    /// ```
    /// use eclectic::EntryMap;
    ///
    /// fn group<I, V, M>(key_values: I) -> M
    /// where
    ///     M: Default + for<'a> EntryMap<'a, Value = Vec<V>>,
    ///     I: IntoIterator<Item = (M::Key, V)>,
    /// {
    ///     let mut groups = M::default();
    ///
    ///     for (key, value) in key_values {
    ///         groups.entry(key).or_insert(vec![]).push(value);
    ///     }
    ///
    ///     groups
    /// }
    /// ```
    fn entry(&'a mut self, key: Self::Key) -> Entry<Self::Occupied, Self::Vacant>;
}

/// A map entry.
///
/// See [`EntryMap::entry`](trait.EntryMap.html#tymethod.entry) for an example.
pub enum Entry<O, V> {
    /// An occupied entry.
    Occupied(O),
    /// A vacant entry.
    Vacant(V),
}

impl<'a, O, V> Entry<O, V>
where
    O: OccupiedEntry<'a>,
    V: VacantEntry<'a, Key = O::Key, Value = O::Value>
{
    /// Ensures that the entry is occupied by inserting the given default value if it is vacant,
    /// returning a mutable reference to the value.
    pub fn or_insert(self, default: O::Value) -> &'a mut O::Value {
        match self {
            Entry::Occupied(e) => e.into_mut(),
            Entry::Vacant(e) => e.insert(default),
        }
    }

    /// Ensures that the entry is occupied by inserting the the result of the given function if it
    /// is vacant, returning a mutable reference to the value.
    pub fn or_insert_with<F>(self, default: F) -> &'a mut O::Value where F: FnOnce() -> O::Value {
        match self {
            Entry::Occupied(e) => e.into_mut(),
            Entry::Vacant(e) => e.insert(default()),
        }
    }
}

/// An occupied map entry.
pub trait OccupiedEntry<'a> {
    /// The entry's key type.
    type Key: 'a;

    /// The entry's value type.
    type Value: 'a;

    /// Returns a reference to the entry's value.
    fn get(&self) -> &Self::Value;

    /// Returns a mutable reference to the entry's value.
    fn get_mut(&mut self) -> &mut Self::Value;

    /// Sets the entry's value to the given value and returns the previous one.
    fn insert(&mut self, value: Self::Value) -> Self::Value;

    /// Returns a mutable reference to the entry's value with the lifetime of the map.
    fn into_mut(self) -> &'a mut Self::Value;

    /// Removes the entry from the map and returns its value.
    fn remove(self) -> Self::Value;
}

/// A vacant map entry.
pub trait VacantEntry<'a> {
    /// The entry's key type.
    type Key: 'a;

    /// The entry's value type.
    type Value: 'a;

    /// Inserts the entry into the map with the given value, returning a mutable reference to the
    /// value with the lifetime of the map.
    fn insert(self, value: Self::Value) -> &'a mut Self::Value;
}

/// A trait that declares a set's item type.
///
/// It is unusual to bound types by this trait directly. Consider using [`Set`](trait.Set.html)
/// instead.
pub trait BaseSet: Collection {
    /// The set's item type.
    type Item;
}

/// A set.
pub trait Set: SetLookup<<Self as BaseSet>::Item> {
    /// Inserts the given item into the set and returns `true` if the set did not already contain
    /// it.
    fn insert(&mut self, item: Self::Item) -> bool;
}

/// A set that supports alternate item lookups.
pub trait SetLookup<Q: ?Sized>: BaseSet {
    /// Checks if the set contains the given item.
    fn contains(&self, item: &Q) -> bool;

    /// Removes the given item from the set and returns `true` if the set contained it.
    fn remove(&mut self, item: &Q) -> bool;
}
