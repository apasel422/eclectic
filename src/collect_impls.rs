extern crate compare;
extern crate collect;

use self::compare::Compare;
use self::collect::{TreeMap, TreeSet, TrieSet};
use self::collect::enum_set::{CLike, EnumSet};
use self::collect::proto::linear_map::LinearMap;
use self::collect::trie_map::{self, TrieMap};
use super::{Collection};
use super::{Map, MapLookup};
use super::{Entry, EntryMap, OccupiedEntry, VacantEntry};
use super::{Set, SetLookup};

impl<K, V> Collection for LinearMap<K, V> where K: Eq { collection_methods!{} }
impl<K, V> Map for LinearMap<K, V> where K: Eq { type Key = K; type Value = V; map_methods!{K, V} }
impl<K, V> MapLookup<K> for LinearMap<K, V> where K: Eq { type MapValue = V; map_lookup_methods!{K, V} }

impl<K, V, C> Collection for TreeMap<K, V, C> where C: Compare<K> { collection_methods!{} }
impl<K, V, C> Map for TreeMap<K, V, C> where C: Compare<K> { type Key = K; type Value = V; map_methods!{K, V} }
impl<K, V, C, Q: ?Sized> MapLookup<Q> for TreeMap<K, V, C>
    where C: Compare<K> + Compare<Q, K> { type MapValue = V; map_lookup_methods!{Q, V} }

impl<V> Collection for TrieMap<V> { collection_methods!{} }
impl<V> Map for TrieMap<V> { type Key = usize; type Value = V; map_methods!{usize, V} }
impl<V> MapLookup<usize> for TrieMap<V> { type MapValue = V; map_lookup_methods!{usize, V} }

impl<'a, V: 'a> EntryMap<'a> for TrieMap<V> {
    type Occupied = trie_map::OccupiedEntry<'a, V>;
    type Vacant = trie_map::VacantEntry<'a, V>;

    fn entry(&'a mut self, key: usize) ->
        Entry<trie_map::OccupiedEntry<'a, V>, trie_map::VacantEntry<'a, V>> {

        match self.entry(key) { // FIXME: all `Entry` enums should be changed to this crate's
            trie_map::Entry::Occupied(e) => Entry::Occupied(e),
            trie_map::Entry::Vacant(e) => Entry::Vacant(e),
        }
    }
}

impl<'a, V: 'a> OccupiedEntry<'a> for trie_map::OccupiedEntry<'a, V> {
    type Value = V;
    occupied_entry_methods!{V}
    fn into_mut(self) -> &'a mut V { self.into_mut() }
}

impl<'a, V: 'a> VacantEntry<'a> for trie_map::VacantEntry<'a, V> {
    type Value = V;
    fn insert(self, value: V) -> &'a mut V { self.insert(value) }
}

impl<T> Collection for EnumSet<T> where T: CLike { collection_methods!{} }
impl<T> Set for EnumSet<T> where T: CLike { type Item = T; set_methods!{T} }
impl<T> SetLookup<T> for EnumSet<T> where T: CLike { set_lookup_methods!{T} }

impl<T, C> Collection for TreeSet<T, C> where C: Compare<T> { collection_methods!{} }
impl<T, C> Set for TreeSet<T, C> where C: Compare<T> { type Item = T; set_methods!{T} }
impl<T, C, Q: ?Sized> SetLookup<Q> for TreeSet<T, C>
    where C: Compare<T> + Compare<Q, T> { set_lookup_methods!{Q} }

impl Collection for TrieSet { collection_methods!{} }
impl Set for TrieSet { type Item = usize; set_methods!{usize} }
impl SetLookup<usize> for TrieSet { set_lookup_methods!{usize} }
