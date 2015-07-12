use std::borrow::Borrow;
use std::collections::btree_map::{self, BTreeMap};
use std::collections::hash_map::{self, HashMap};
use std::collections::{BTreeSet, BinaryHeap, HashSet, LinkedList, VecDeque};
use std::hash::Hash;
use super::*;

impl<T> Collection for BinaryHeap<T> where T: Ord {
    fn clear(&mut self) { self.clear(); }
    fn is_empty(&self) -> bool { self.is_empty() }
    fn len(&self) -> usize { self.len() }
}

impl<T> Collection for LinkedList<T> {
    fn clear(&mut self) { self.clear(); }
    fn is_empty(&self) -> bool { self.is_empty() }
    fn len(&self) -> usize { self.len() }
}

impl<T> Collection for Vec<T> {
    fn clear(&mut self) { self.clear(); }
    fn is_empty(&self) -> bool { self.is_empty() }
    fn len(&self) -> usize { self.len() }
}

impl<T> Collection for VecDeque<T> {
    fn clear(&mut self) { self.clear(); }
    fn is_empty(&self) -> bool { self.is_empty() }
    fn len(&self) -> usize { self.len() }
}

impl<K, V> Collection for BTreeMap<K, V> where K: Ord {
    fn clear(&mut self) { self.clear(); }
    fn is_empty(&self) -> bool { self.is_empty() }
    fn len(&self) -> usize { self.len() }
}

impl<K, V> BaseMap for BTreeMap<K, V> where K: Ord {
    type Key = K;
    type Value = V;
}

impl<K, V> Map for BTreeMap<K, V> where K: Ord {
    fn insert(&mut self, key: K, value: V) -> Option<V> { self.insert(key, value) }
}

impl<K, V, Q: ?Sized> MapLookup<Q> for BTreeMap<K, V> where K: Ord + Borrow<Q>, Q: Ord {
    fn contains_key(&self, key: &Q) -> bool { self.contains_key(key) }
    fn get(&self, key: &Q) -> Option<&V> { self.get(key) }
    fn get_mut(&mut self, key: &Q) -> Option<&mut V> { self.get_mut(key) }
    fn remove(&mut self, key: &Q) -> Option<V> { self.remove(key) }
}

impl<'a, K: 'a, V: 'a> EntryMap<'a> for BTreeMap<K, V> where K: Ord {
    type Occupied = btree_map::OccupiedEntry<'a, K, V>;
    type Vacant = btree_map::VacantEntry<'a, K, V>;

    fn entry(&'a mut self, key: K) -> Entry<Self::Occupied, Self::Vacant> {
        match self.entry(key) { // FIXME: all `Entry` enums should be changed to this crate's
            btree_map::Entry::Occupied(e) => Entry::Occupied(e),
            btree_map::Entry::Vacant(e) => Entry::Vacant(e),
        }
    }
}

impl<'a, K: 'a, V: 'a> OccupiedEntry<'a> for btree_map::OccupiedEntry<'a, K, V> where K: Ord {
    type Key = K;
    type Value = V;
    fn get(&self) -> &V { self.get() }
    fn get_mut(&mut self) -> &mut V { self.get_mut() }
    fn insert(&mut self, value: V) -> V { self.insert(value) }
    fn into_mut(self) -> &'a mut V { self.into_mut() }
    fn remove(self) -> V { self.remove() }
}

impl<'a, K: 'a, V: 'a> VacantEntry<'a> for btree_map::VacantEntry<'a, K, V> where K: Ord {
    type Key = K;
    type Value = V;
    fn insert(self, value: V) -> &'a mut V { self.insert(value) }
}

impl<K, V> Collection for HashMap<K, V> where K: Eq + Hash {
    fn clear(&mut self) { self.clear(); }
    fn is_empty(&self) -> bool { self.is_empty() }
    fn len(&self) -> usize { self.len() }
}

impl<K, V> BaseMap for HashMap<K, V> where K: Eq + Hash {
    type Key = K;
    type Value = V;
}

impl<K, V> Map for HashMap<K, V> where K: Eq + Hash {
    fn insert(&mut self, key: K, value: V) -> Option<V> { self.insert(key, value) }
}

impl<K, V, Q: ?Sized> MapLookup<Q> for HashMap<K, V> where K: Eq + Hash + Borrow<Q>, Q: Eq + Hash {
    fn contains_key(&self, key: &Q) -> bool { self.contains_key(key) }
    fn get(&self, key: &Q) -> Option<&V> { self.get(key) }
    fn get_mut(&mut self, key: &Q) -> Option<&mut V> { self.get_mut(key) }
    fn remove(&mut self, key: &Q) -> Option<V> { self.remove(key) }
}

impl<'a, K: 'a, V: 'a> EntryMap<'a> for HashMap<K, V> where K: Eq + Hash {
    type Occupied = hash_map::OccupiedEntry<'a, K, V>;
    type Vacant = hash_map::VacantEntry<'a, K, V>;

    fn entry(&'a mut self, key: K) -> Entry<Self::Occupied, Self::Vacant> {
        match self.entry(key) { // FIXME: all `Entry` enums should be changed to this crate's
            hash_map::Entry::Occupied(e) => Entry::Occupied(e),
            hash_map::Entry::Vacant(e) => Entry::Vacant(e),
        }
    }
}

impl<'a, K: 'a, V: 'a> OccupiedEntry<'a> for hash_map::OccupiedEntry<'a, K, V> where K: Eq + Hash {
    type Key = K;
    type Value = V;
    fn get(&self) -> &V { self.get() }
    fn get_mut(&mut self) -> &mut V { self.get_mut() }
    fn insert(&mut self, value: V) -> V { self.insert(value) }
    fn into_mut(self) -> &'a mut V { self.into_mut() }
    fn remove(self) -> V { self.remove() }
}

impl<'a, K: 'a, V: 'a> VacantEntry<'a> for hash_map::VacantEntry<'a, K, V> where K: Eq + Hash {
    type Key = K;
    type Value = V;
    fn insert(self, value: V) -> &'a mut V { self.insert(value) }
}

impl<T> Collection for BTreeSet<T> where T: Ord {
    fn clear(&mut self) { self.clear(); }
    fn is_empty(&self) -> bool { self.is_empty() }
    fn len(&self) -> usize { self.len() }
}

impl<T> BaseSet for BTreeSet<T> where T: Ord {
    type Item = T;
}

impl<T> Set for BTreeSet<T> where T: Ord {
    fn insert(&mut self, item: T) -> bool { self.insert(item) }
}

impl<T, Q: ?Sized> SetLookup<Q> for BTreeSet<T> where T: Ord + Borrow<Q>, Q: Ord {
    fn contains(&self, item: &Q) -> bool { self.contains(item) }
    fn remove(&mut self, item: &Q) -> bool { self.remove(item) }
}

impl<T> Collection for HashSet<T> where T: Eq + Hash {
    fn clear(&mut self) { self.clear(); }
    fn is_empty(&self) -> bool { self.is_empty() }
    fn len(&self) -> usize { self.len() }
}

impl<T> BaseSet for HashSet<T> where T: Eq + Hash {
    type Item = T;
}

impl<T> Set for HashSet<T> where T: Eq + Hash {
    fn insert(&mut self, item: T) -> bool { self.insert(item) }
}

impl<T, Q: ?Sized> SetLookup<Q> for HashSet<T> where T: Eq + Hash + Borrow<Q>, Q: Eq + Hash {
    fn contains(&self, item: &Q) -> bool { self.contains(item) }
    fn remove(&mut self, item: &Q) -> bool { self.remove(item) }
}
