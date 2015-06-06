use std::collections::BitSet;
use std::collections::vec_map::{self, VecMap};
use super::{Collection};
use super::{Map, MapLookup};
use super::{Entry, EntryMap, OccupiedEntry, VacantEntry};
use super::{Set, SetLookup};

impl<V> Collection for VecMap<V> { collection_methods!{} }
impl<V> Map for VecMap<V> { type Key = usize; type Value = V; map_methods!{usize, V} }
impl<V> MapLookup<usize> for VecMap<V> { type MapValue = V; map_lookup_methods!{usize, V} }

impl<'a, V: 'a> EntryMap<'a> for VecMap<V> {
    type Occupied = vec_map::OccupiedEntry<'a, V>;
    type Vacant = vec_map::VacantEntry<'a, V>;

    fn entry(&'a mut self, key: usize) ->
        Entry<vec_map::OccupiedEntry<'a, V>, vec_map::VacantEntry<'a, V>> {

        match self.entry(key) { // FIXME: all `Entry` enums should be changed to this crate's
            vec_map::Entry::Occupied(e) => Entry::Occupied(e),
            vec_map::Entry::Vacant(e) => Entry::Vacant(e),
        }
    }
}

impl<'a, V: 'a> OccupiedEntry<'a> for vec_map::OccupiedEntry<'a, V> {
    type Value = V;
    occupied_entry_methods!{V}
    fn into_mut(self) -> &'a mut V { self.into_mut() }
}

impl<'a, V: 'a> VacantEntry<'a> for vec_map::VacantEntry<'a, V> {
    type Value = V;
    fn insert(self, value: V) -> &'a mut V { self.insert(value) }
}

impl Collection for BitSet { collection_methods!{} }
impl Set for BitSet { type Item = usize; set_methods!{usize} }
impl SetLookup<usize> for BitSet { set_lookup_methods!{usize} }
