extern crate linear_map;

use self::linear_map::LinearMap;
use std::borrow::Borrow;
use super::Entry;

impl<K, V> super::Collection for LinearMap<K, V> where K: Eq {
    fn clear(&mut self) { self.clear(); }
    fn is_empty(&self) -> bool { self.is_empty() }
    fn len(&self) -> usize { self.len() }
}

impl<K, V> super::BaseMap for LinearMap<K, V> where K: Eq {
    type Key = K;
    type Value = V;
}

impl<K, V, Q: ?Sized> super::MapLookup<Q> for LinearMap<K, V> where K: Eq + Borrow<Q>, Q: Eq {
    fn contains_key(&self, key: &Q) -> bool { self.contains_key(key) }
    fn get(&self, key: &Q) -> Option<&V> { self.get(key) }
    fn get_mut(&mut self, key: &Q) -> Option<&mut V> { self.get_mut(key) }
    fn remove(&mut self, key: &Q) -> Option<V> { self.remove(key) }
}

impl<K, V> super::Map for LinearMap<K, V> where K: Eq {
    fn insert(&mut self, key: K, value: V) -> Option<V> { self.insert(key, value) }
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
    fn get(&self) -> &V { self.get() }
    fn get_mut(&mut self) -> &mut V { self.get_mut() }
    fn insert(&mut self, value: V) -> V { self.insert(value) }
    fn into_mut(self) -> &'a mut V { self.into_mut() }
    fn remove(self) -> V { self.remove() }
}

impl<'a, K: 'a, V: 'a> super::VacantEntry<'a> for linear_map::VacantEntry<'a, K, V> {
    type Key = K;
    type Value = V;
    fn insert(self, value: V) -> &'a mut V { self.insert(value) }
}
