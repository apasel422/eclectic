use std::borrow::Borrow;
use std::collections::btree_map::{self, BTreeMap};
use std::collections::hash_map::{self, HashMap};
use std::collections::vec_map::{self, VecMap};
use std::collections::{BitSet, BTreeSet, HashSet};
use std::hash::Hash;
use super::{Collection, MutCollection};
use super::{Map, MapLookup, MutMap, MutMapLookup};
use super::{Entry, EntryMap, OccupiedEntry, VacantEntry};
use super::{Set, SetLookup, MutSet, MutSetLookup};

impl<K, V> Collection for BTreeMap<K, V> where K: Ord { collection_methods!{} }
impl<K, V> MutCollection for BTreeMap<K, V> where K: Ord { mut_collection_methods!{} }
impl<K, V> Map for BTreeMap<K, V> where K: Ord { type Key = K; type Value = V; }
impl<K, V, Q: ?Sized> MapLookup<Q> for BTreeMap<K, V>
    where K: Ord + Borrow<Q>, Q: Ord { type MapValue = V; map_lookup_methods!{Q, V} }
impl<K, V> MutMap for BTreeMap<K, V> where K: Ord { mut_map_methods!{K, V} }
impl<K, V, Q: ?Sized> MutMapLookup<Q> for BTreeMap<K, V>
    where K: Ord + Borrow<Q>, Q: Ord { mut_map_lookup_methods!{Q, V} }

impl<'a, K, V: 'a> EntryMap<'a> for BTreeMap<K, V> where K: Ord {
    type Occupied = btree_map::OccupiedEntry<'a, K, V>;
    type Vacant = btree_map::VacantEntry<'a, K, V>;

    fn entry(&'a mut self, key: K) ->
        Entry<btree_map::OccupiedEntry<'a, K, V>, btree_map::VacantEntry<'a, K, V>> {

        match self.entry(key) { // FIXME: all `Entry` enums should be changed to this crate's
            btree_map::Entry::Occupied(e) => Entry::Occupied(e),
            btree_map::Entry::Vacant(e) => Entry::Vacant(e),
        }
    }
}

impl<'a, K, V: 'a> OccupiedEntry<'a> for btree_map::OccupiedEntry<'a, K, V> where K: Ord {
    type Value = V;
    occupied_entry_methods!{V}
    fn into_mut(self) -> &'a mut V { self.into_mut() }
}

impl<'a, K, V: 'a> VacantEntry<'a> for btree_map::VacantEntry<'a, K, V> where K: Ord {
    type Value = V;
    fn insert(self, value: V) -> &'a mut V { self.insert(value) }
}

impl<K, V> Collection for HashMap<K, V> where K: Eq + Hash { collection_methods!{} }
impl<K, V> MutCollection for HashMap<K, V> where K: Eq + Hash { mut_collection_methods!{} }
impl<K, V> Map for HashMap<K, V> where K: Eq + Hash { type Key = K; type Value = V; }
impl<K, V, Q: ?Sized> MapLookup<Q> for HashMap<K, V>
    where K: Eq + Hash + Borrow<Q>, Q: Eq + Hash { type MapValue = V; map_lookup_methods!{Q, V} }
impl<K, V> MutMap for HashMap<K, V> where K: Eq + Hash { mut_map_methods!{K, V} }
impl<K, V, Q: ?Sized> MutMapLookup<Q> for HashMap<K, V>
    where K: Eq + Hash + Borrow<Q>, Q: Eq + Hash { mut_map_lookup_methods!{Q, V} }

impl<'a, K, V: 'a> EntryMap<'a> for HashMap<K, V> where K: Eq + Hash {
    type Occupied = hash_map::OccupiedEntry<'a, K, V>;
    type Vacant = hash_map::VacantEntry<'a, K, V>;

    fn entry(&'a mut self, key: K) ->
        Entry<hash_map::OccupiedEntry<'a, K, V>, hash_map::VacantEntry<'a, K, V>> {

        match self.entry(key) { // FIXME: all `Entry` enums should be changed to this crate's
            hash_map::Entry::Occupied(e) => Entry::Occupied(e),
            hash_map::Entry::Vacant(e) => Entry::Vacant(e),
        }
    }
}

impl<'a, K, V: 'a> OccupiedEntry<'a> for hash_map::OccupiedEntry<'a, K, V> where K: Eq + Hash {
    type Value = V;
    occupied_entry_methods!{V}
    fn into_mut(self) -> &'a mut V { self.into_mut() }
}

impl<'a, K, V: 'a> VacantEntry<'a> for hash_map::VacantEntry<'a, K, V> where K: Eq + Hash {
    type Value = V;
    fn insert(self, value: V) -> &'a mut V { self.insert(value) }
}

impl<V> Collection for VecMap<V> { collection_methods!{} }
impl<V> MutCollection for VecMap<V> { mut_collection_methods!{} }
impl<V> Map for VecMap<V> { type Key = usize; type Value = V; }
impl<V> MapLookup<usize> for VecMap<V> { type MapValue = V; map_lookup_methods!{usize, V} }
impl<V> MutMap for VecMap<V> { mut_map_methods!{usize, V} }
impl<V> MutMapLookup<usize> for VecMap<V> { mut_map_lookup_methods!{usize, V} }

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
impl MutCollection for BitSet { mut_collection_methods!{} }
impl Set for BitSet { type Item = usize; }
impl SetLookup<usize> for BitSet { set_lookup_methods!{usize} }
impl MutSet for BitSet { mut_set_methods!{usize} }
impl MutSetLookup<usize> for BitSet { mut_set_lookup_methods!{usize} }

impl<T> Collection for BTreeSet<T> where T: Ord { collection_methods!{} }
impl<T> MutCollection for BTreeSet<T> where T: Ord { mut_collection_methods!{} }
impl<T> Set for BTreeSet<T> where T: Ord { type Item = T; }
impl<T, Q: ?Sized> SetLookup<Q> for BTreeSet<T>
    where T: Ord + Borrow<Q>, Q: Ord { set_lookup_methods!{Q} }
impl<T> MutSet for BTreeSet<T> where T: Ord { mut_set_methods!{T} }
impl<T, Q: ?Sized> MutSetLookup<Q> for BTreeSet<T>
    where T: Ord + Borrow<Q>, Q: Ord { mut_set_lookup_methods!{Q} }

impl<T> Collection for HashSet<T> where T: Eq + Hash { collection_methods!{} }
impl<T> MutCollection for HashSet<T> where T: Eq + Hash { mut_collection_methods!{} }
impl<T> Set for HashSet<T> where T: Eq + Hash { type Item = T; }
impl<T, Q: ?Sized> SetLookup<Q> for HashSet<T>
    where T: Eq + Hash + Borrow<Q>, Q: Eq + Hash { set_lookup_methods!{Q} }
impl<T> MutSet for HashSet<T> where T: Eq + Hash { mut_set_methods!{T} }
impl<T, Q: ?Sized> MutSetLookup<Q> for HashSet<T>
    where T: Eq + Hash + Borrow<Q>, Q: Eq + Hash { mut_set_lookup_methods!{Q} }
