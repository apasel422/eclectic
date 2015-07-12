use std::borrow::Borrow;
use std::collections::btree_map::{self, BTreeMap};
use std::collections::hash_map::{self, HashMap};
use std::collections::{BTreeSet, BinaryHeap, HashSet, LinkedList, VecDeque};
use std::hash::Hash;
use super::{Collection};
use super::{BaseMap, Map, MapLookup};
use super::{Entry, EntryMap, OccupiedEntry, VacantEntry};
use super::{BaseSet, Set, SetLookup};

impl<T> Collection for BinaryHeap<T> where T: Ord { collection_methods!{} }

impl<T> Collection for LinkedList<T> { collection_methods!{} }

impl<T> Collection for Vec<T> { collection_methods!{} }

impl<T> Collection for VecDeque<T> { collection_methods!{} }

impl<K, V> Collection for BTreeMap<K, V> where K: Ord { collection_methods!{} }
impl<K, V> BaseMap for BTreeMap<K, V> where K: Ord { type Key = K; type Value = V; }
impl<K, V> Map for BTreeMap<K, V> where K: Ord { map_methods!{} }
impl<K, V, Q: ?Sized> MapLookup<Q> for BTreeMap<K, V>
    where K: Ord + Borrow<Q>, Q: Ord { map_lookup_methods!{Q} }

impl<'a, K: 'a, V: 'a> EntryMap<'a> for BTreeMap<K, V> where K: Ord {
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

impl<'a, K: 'a, V: 'a> OccupiedEntry<'a> for btree_map::OccupiedEntry<'a, K, V> where K: Ord {
    type Key = K;
    type Value = V;
    occupied_entry_methods!{}
    fn into_mut(self) -> &'a mut V { self.into_mut() }
}

impl<'a, K: 'a, V: 'a> VacantEntry<'a> for btree_map::VacantEntry<'a, K, V> where K: Ord {
    type Key = K;
    type Value = V;
    fn insert(self, value: V) -> &'a mut V { self.insert(value) }
}

impl<K, V> Collection for HashMap<K, V> where K: Eq + Hash { collection_methods!{} }
impl<K, V> BaseMap for HashMap<K, V> where K: Eq + Hash { type Key = K; type Value = V; }
impl<K, V> Map for HashMap<K, V> where K: Eq + Hash { map_methods!{} }
impl<K, V, Q: ?Sized> MapLookup<Q> for HashMap<K, V>
    where K: Eq + Hash + Borrow<Q>, Q: Eq + Hash { map_lookup_methods!{Q} }

impl<'a, K: 'a, V: 'a> EntryMap<'a> for HashMap<K, V> where K: Eq + Hash {
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

impl<'a, K: 'a, V: 'a> OccupiedEntry<'a> for hash_map::OccupiedEntry<'a, K, V> where K: Eq + Hash {
    type Key = K;
    type Value = V;
    occupied_entry_methods!{}
    fn into_mut(self) -> &'a mut V { self.into_mut() }
}

impl<'a, K: 'a, V: 'a> VacantEntry<'a> for hash_map::VacantEntry<'a, K, V> where K: Eq + Hash {
    type Key = K;
    type Value = V;
    fn insert(self, value: V) -> &'a mut V { self.insert(value) }
}

impl<T> Collection for BTreeSet<T> where T: Ord { collection_methods!{} }
impl<T> BaseSet for BTreeSet<T> where T: Ord {type Item = T; }
impl<T> Set for BTreeSet<T> where T: Ord { set_methods!{} }
impl<T, Q: ?Sized> SetLookup<Q> for BTreeSet<T>
    where T: Ord + Borrow<Q>, Q: Ord { set_lookup_methods!{Q} }

impl<T> Collection for HashSet<T> where T: Eq + Hash { collection_methods!{} }
impl<T> BaseSet for HashSet<T> where T: Eq + Hash { type Item = T; }
impl<T> Set for HashSet<T> where T: Eq + Hash { set_methods!{} }
impl<T, Q: ?Sized> SetLookup<Q> for HashSet<T>
    where T: Eq + Hash + Borrow<Q>, Q: Eq + Hash { set_lookup_methods!{Q} }
