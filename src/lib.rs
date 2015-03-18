#![feature(collections)]
#![feature(std_misc)]

#[macro_use] mod macros;
#[cfg(feature = "collect_impls")] mod collect_impls;
mod std_impls;

pub trait Collection {
    /// Removes all items from the collection.
    fn clear(&mut self);

    /// Checks if the collection is empty.
    fn is_empty(&self) -> bool { self.len() == 0 }

    /// Returns the number of items in the collection.
    fn len(&self) -> usize;
}

pub trait Map: Collection + MapLookup<<Self as Map>::Key, MapValue=<Self as Map>::Value> {
    /// The map's key type.
    type Key;
    /// The map's value type.
    type Value;

    /// Inserts the given key and value into the map, returning the previous value associated with
    /// the key, or `None` if the map did not already contain the key.
    fn insert(&mut self, key: Self::Key, value: Self::Value) -> Option<Self::Value>;
}

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

pub trait EntryMap<'a>: Map {
    type Occupied: OccupiedEntry<'a, Value=Self::Value>;
    type Vacant: VacantEntry<'a, Value=Self::Value>;

    /// Returns the given key's corresponding entry in the map for in-place manipulation.
    fn entry(&'a mut self, key: Self::Key) -> Entry<Self::Occupied, Self::Vacant>;
}

pub enum Entry<O, V> {
    Occupied(O),
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
}

pub trait OccupiedEntry<'a> {
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

pub trait VacantEntry<'a> {
    type Value: 'a;

    /// Sets the entry's value to the given one, returning a mutable reference to the value with
    /// the lifetime of the map.
    fn insert(self, value: Self::Value) -> &'a mut Self::Value;
}

pub trait Set: Collection + SetLookup<<Self as Set>::Item> {
    type Item;

    /// Inserts the given item into the set, returning `true` if the set did not already contain
    /// the item.
    fn insert(&mut self, item: Self::Item) -> bool;
}

pub trait SetLookup<Q: ?Sized> {
    /// Checks if the set contains the given item.
    fn contains(&self, item: &Q) -> bool;

    /// Removes the given item from the map, returning `true` if the set contained the item.
    fn remove(&mut self, item: &Q) -> bool;
}
