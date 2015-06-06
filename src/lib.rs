#![feature(collections)]

#[macro_use] mod macros;
mod std_impls;

/// A collection.
pub trait Collection {
    /// Removes all items from the collection.
    fn clear(&mut self);

    /// Checks if the collection is empty.
    fn is_empty(&self) -> bool { self.len() == 0 }

    /// Returns the number of items in the collection.
    fn len(&self) -> usize;
}

/// A map.
pub trait Map: Collection + MapLookup<<Self as Map>::Key, MapValue=<Self as Map>::Value> {
    /// The map's key type.
    type Key;
    /// The map's value type.
    type Value;

    /// Inserts the given key and value into the map, returning the previous value associated with
    /// the key, or `None` if the map did not already contain the key.
    fn insert(&mut self, key: Self::Key, value: Self::Value) -> Option<Self::Value>;
}

/// A map that supports alternate key lookups.
pub trait MapLookup<Q: ?Sized> {
    type MapValue;

    /// Checks if the map contains the given key.
    fn contains_key(&self, key: &Q) -> bool { self.get(key).is_some() }

    /// Returns a reference to the value associated with the given key in the map, or `None` if
    /// the map does not contain the key.
    fn get(&self, key: &Q) -> Option<&Self::MapValue>;

    /// Returns a mutable reference to the value associated with the given key in the map, or
    /// `None` if the map does not contain the key.
    fn get_mut(&mut self, key: &Q) -> Option<&mut Self::MapValue>;

    /// Removes the given key from the map, returning the value associated with it, or `None` if
    /// the map did not contain the key.
    fn remove(&mut self, key: &Q) -> Option<Self::MapValue>;
}

/// A map that supports the entry API.
pub trait EntryMap<'a>: Map {
    /// The occupied entry type.
    type Occupied: OccupiedEntry<'a, Value=Self::Value>;

    /// The vacant entry type.
    type Vacant: VacantEntry<'a, Value=Self::Value>;

    /// Returns the given key's corresponding entry in the map for in-place manipulation.
    fn entry(&'a mut self, key: Self::Key) -> Entry<Self::Occupied, Self::Vacant>;
}

/// A map entry.
pub enum Entry<O, V> {
    /// An occupied entry.
    Occupied(O),
    /// A vacant entry.
    Vacant(V),
}

impl<'a, O, V> Entry<O, V> where O: OccupiedEntry<'a>, V: VacantEntry<'a, Value=O::Value> {
    /// Returns a mutable reference to the entry's value if it is occupied, or the vacant entry if
    /// it is vacant.
    pub fn get(self) -> Result<&'a mut O::Value, V> {
        match self {
            Entry::Occupied(e) => Ok(e.into_mut()),
            Entry::Vacant(e) => Err(e),
        }
    }

    /// Ensures that a value is in the entry by inserting the default if it is empty, and returns a
    /// mutable reference to the value.
    pub fn or_insert(self, default: O::Value) -> &'a mut O::Value {
        match self {
            Entry::Occupied(e) => e.into_mut(),
            Entry::Vacant(e) => e.insert(default),
        }
    }

    /// Ensures that a value is in the entry by inserting the result of the default function if it
    /// is empty, and returns a mutable reference to the value.
    pub fn or_insert_with<F>(self, default: F) -> &'a mut O::Value where F: FnOnce() -> O::Value {
        match self {
            Entry::Occupied(e) => e.into_mut(),
            Entry::Vacant(e) => e.insert(default()),
        }
    }
}

/// An occupied map entry.
pub trait OccupiedEntry<'a> {
    /// The entry's value type.
    type Value: 'a;

    /// Returns a reference to the entry's value.
    fn get(&self) -> &Self::Value;

    /// Returns a mutable reference to the entry's value.
    fn get_mut(&mut self) -> &mut Self::Value;

    /// Sets the entry's value to the given one, returning the old value.
    fn insert(&mut self, value: Self::Value) -> Self::Value;

    /// Returns a mutable reference to the entry's value with the lifetime of the map.
    fn into_mut(self) -> &'a mut Self::Value;

    /// Removes the entry, returning its value.
    fn remove(self) -> Self::Value;
}

/// A vacant map entry.
pub trait VacantEntry<'a> {
    /// The entry's value type.
    type Value: 'a;

    /// Sets the entry's value to the given one, returning a mutable reference to the value with
    /// the lifetime of the map.
    fn insert(self, value: Self::Value) -> &'a mut Self::Value;
}

/// A set.
pub trait Set: Collection + SetLookup<<Self as Set>::Item> {
    /// The set's item type.
    type Item;

    /// Inserts the given item into the set, returning `true` if the set did not already contain
    /// the item.
    fn insert(&mut self, item: Self::Item) -> bool;
}

/// A set that supports alternate item lookups.
pub trait SetLookup<Q: ?Sized> {
    /// Checks if the set contains the given item.
    fn contains(&self, item: &Q) -> bool;

    /// Removes the given item from the set, returning `true` if the set contained the item.
    fn remove(&mut self, item: &Q) -> bool;
}
