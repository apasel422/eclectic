extern crate vec_map;

use self::vec_map::VecMap;
use super::Entry;

impl<V> super::Collection for VecMap<V> { collection_methods!{} }
impl<V> super::BaseMap for VecMap<V> { type Key = usize; type Value = V; }
impl<V> super::Map for VecMap<V> { map_methods!{} }
impl<V> super::MapLookup<usize> for VecMap<V> { map_lookup_methods!{usize} }

impl<'a, V: 'a> super::EntryMap<'a> for VecMap<V> {
    type Occupied = vec_map::OccupiedEntry<'a, V>;
    type Vacant = vec_map::VacantEntry<'a, V>;

    fn entry(&'a mut self, key: usize) -> Entry<Self::Occupied, Self::Vacant> {
        match self.entry(key) {
            vec_map::Entry::Occupied(e) => Entry::Occupied(e),
            vec_map::Entry::Vacant(e) => Entry::Vacant(e),
        }
    }
}

impl<'a, V: 'a> super::OccupiedEntry<'a> for vec_map::OccupiedEntry<'a, V> {
    type Key = usize;
    type Value = V;
    occupied_entry_methods!{}
    fn into_mut(self) -> &'a mut V { self.into_mut() }
}

impl<'a, V: 'a> super::VacantEntry<'a> for vec_map::VacantEntry<'a, V> {
    type Key = usize;
    type Value = V;
    fn insert(self, value: V) -> &'a mut V { self.insert(value) }
}
