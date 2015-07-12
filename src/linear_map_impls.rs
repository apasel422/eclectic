extern crate linear_map;

use self::linear_map::LinearMap;
use std::borrow::Borrow;
use super::Entry;

impl<K, V> super::Collection for LinearMap<K, V> where K: Eq {
    collection_methods!{}
}

impl<K, V> super::BaseMap for LinearMap<K, V> where K: Eq {
    type Key = K;
    type Value = V;
}

impl<K, V, Q: ?Sized> super::MapLookup<Q> for LinearMap<K, V> where K: Eq + Borrow<Q>, Q: Eq {
    map_lookup_methods!{Q, V}
}

impl<K, V> super::Map for LinearMap<K, V> where K: Eq {
    map_methods!{K, V}
}

impl<'a, K: 'a, V: 'a> super::EntryMap<'a> for LinearMap<K, V> where K: Eq {
    type Occupied = linear_map::OccupiedEntry<'a, K, V>;
    type Vacant = linear_map::VacantEntry<'a, K, V>;

    fn entry(&'a mut self, key: K) -> Entry<Self::Occupied, Self::Vacant> {
        match self.entry(key) {
            linear_map::Entry::Occupied(e) => Entry::Occupied(e),
            linear_map::Entry::Vacant(e) => Entry::Vacant(e),
        }
    }
}

impl<'a, K: 'a, V: 'a> super::OccupiedEntry<'a> for linear_map::OccupiedEntry<'a, K, V> {
    type Key = K;
    type Value = V;
    occupied_entry_methods!{V}
    fn into_mut(self) -> &'a mut V { self.into_mut() }
}

impl<'a, K: 'a, V: 'a> super::VacantEntry<'a> for linear_map::VacantEntry<'a, K, V> {
    type Key = K;
    type Value = V;
    fn insert(self, value: V) -> &'a mut V { self.insert(value) }
}
