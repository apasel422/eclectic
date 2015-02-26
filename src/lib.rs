#![feature(collections)]

#[macro_use] mod macros;
mod std_impls;

pub trait Collection {
    /// Checks if the collection is empty.
    fn is_empty(&self) -> bool { self.len() == 0 }

    /// Returns the number of items in the collection.
    fn len(&self) -> usize;
}

pub trait MutCollection: Collection {
    /// Removes all items from the collection.
    fn clear(&mut self);
}

pub trait Map: Collection where Self: MapLookup<Self::Key> {
    type Key;
    type Value;
}

pub trait MapLookup<Q: ?Sized>: Map {
    /// Checks if the map contains the given key.
    fn contains_key(&self, key: &Q) -> bool { self.get(key).is_some() }

    /// Returns a reference to the value associated with the given key in the map, or `None` if
    /// the map does not contain the key.
    fn get(&self, key: &Q) -> Option<&Self::Value>;
}

pub trait MutMap: MutCollection + Map where Self: MutMapLookup<Self::Key> {
    /// Inserts the given key and value into the map, returning the previous value associated with
    /// the key, or `None` if the map did not already contain the key.
    fn insert(&mut self, key: Self::Key, value: Self::Value) -> Option<Self::Value>;
}

pub trait MutMapLookup<Q: ?Sized>: MutMap + MapLookup<Q> {
    /// Returns a mutable reference to the value associated with the given key in the map, or
    /// `None` if the map does not contain the key.
    fn get_mut(&mut self, key: &Q) -> Option<&mut Self::Value>;

    /// Removes the given key from the map, returning the value associated with it, or `None` if
    /// the map did not contain the key.
    fn remove(&mut self, key: &Q) -> Option<Self::Value>;
}

pub trait Set: Collection where Self: SetLookup<Self::Item> {
    type Item;
}

pub trait SetLookup<Q: ?Sized>: Set {
    /// Checks if the set contains the given item.
    fn contains(&self, item: &Q) -> bool;
}

pub trait MutSet: MutCollection + Set where Self: MutSetLookup<Self::Item> {
    /// Inserts the given item into the set, returning `true` if the set did not already contain
    /// the item.
    fn insert(&mut self, item: Self::Item) -> bool;
}

pub trait MutSetLookup<Q: ?Sized>: MutSet + SetLookup<Q> {
    /// Removes the given item from the map, returning `true` if the set contained the item.
    fn remove(&mut self, item: &Q) -> bool;
}
