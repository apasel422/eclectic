extern crate trie;

use self::trie::map::{self, TrieMap};
use self::trie::TrieSet;
use super::{Collection, Map, MapLookup, Set, SetLookup};
use super::{Entry, EntryMap, OccupiedEntry, VacantEntry};

impl<V> Collection for TrieMap<V> { collection_methods!{} }
impl<V> Map for TrieMap<V> { type Key = usize; type Value = V; map_methods!{usize, V} }
impl<V> MapLookup<usize> for TrieMap<V> { type MapValue = V; map_lookup_methods!{usize, V} }

impl<'a, V: 'a> EntryMap<'a> for TrieMap<V> {
    type Occupied = map::OccupiedEntry<'a, V>;
    type Vacant = map::VacantEntry<'a, V>;

    fn entry(&'a mut self, key: usize) ->
        Entry<map::OccupiedEntry<'a, V>, map::VacantEntry<'a, V>> {

        match self.entry(key) { // FIXME: all `Entry` enums should be changed to this crate's
            map::Entry::Occupied(e) => Entry::Occupied(e),
            map::Entry::Vacant(e) => Entry::Vacant(e),
        }
    }
}

impl<'a, V: 'a> OccupiedEntry<'a> for map::OccupiedEntry<'a, V> {
    type Value = V;
    occupied_entry_methods!{V}
    fn into_mut(self) -> &'a mut V { self.into_mut() }
}

impl<'a, V: 'a> VacantEntry<'a> for map::VacantEntry<'a, V> {
    type Value = V;
    fn insert(self, value: V) -> &'a mut V { self.insert(value) }
}

impl Collection for TrieSet { collection_methods!{} }
impl Set for TrieSet { type Item = usize; set_methods!{usize} }
impl SetLookup<usize> for TrieSet { set_lookup_methods!{usize} }
