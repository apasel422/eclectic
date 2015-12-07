use super::*;

use std::borrow::Borrow;
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, LinkedList, VecDeque};
use std::collections::{btree_map, hash_map};
use std::hash::Hash;

impl<T> Len for [T] {
    fn is_empty(&self) -> bool { self.is_empty() }
    fn len(&self) -> usize { self.len() }
}

impl<T> Len for BinaryHeap<T> where T: Ord {
    fn is_empty(&self) -> bool { self.is_empty() }
    fn len(&self) -> usize { self.len() }
}

impl<T> Clear for BinaryHeap<T> where T: Ord {
    fn clear(&mut self) { self.clear(); }
}

impl<T> New for BinaryHeap<T> where T: Ord {
    fn with_capacity_hint(capacity: usize) -> Self {
        Self::with_capacity(capacity)
    }
}

impl<K, V> Len for BTreeMap<K, V> where K: Ord {
    fn is_empty(&self) -> bool { self.is_empty() }
    fn len(&self) -> usize { self.len() }
}

impl<K, V> Clear for BTreeMap<K, V> where K: Ord {
    fn clear(&mut self) { self.clear(); }
}

impl<K, V> New for BTreeMap<K, V> where K: Ord {}

impl<K, V> map::Base for BTreeMap<K, V> where K: Ord {
    type Key = K;
    type Value = V;
}

impl<K, V, Q: ?Sized> map::Get<Q> for BTreeMap<K, V> where K: Ord + Borrow<Q>, Q: Ord {
    fn contains_key(&self, key: &Q) -> bool { self.contains_key(key) }
    fn get(&self, key: &Q) -> Option<&V> { self.get(key) }
}

impl<K, V, Q: ?Sized> map::GetMut<Q> for BTreeMap<K, V> where K: Ord + Borrow<Q>, Q: Ord {
    fn get_mut(&mut self, key: &Q) -> Option<&mut V> { self.get_mut(key) }
}

impl<K, V, Q: ?Sized> map::Remove<Q> for BTreeMap<K, V> where K: Ord + Borrow<Q>, Q: Ord {
    fn remove(&mut self, key: &Q) -> Option<V> { self.remove(key) }
}

impl<K, V> map::Insert for BTreeMap<K, V> where K: Ord {
    fn insert(&mut self, key: K, value: V) -> Option<V> { self.insert(key, value) }
}

impl<'a, K: 'a, V: 'a> map::EntryMap<'a> for BTreeMap<K, V> where K: Ord {
    type OccupiedEntry = btree_map::OccupiedEntry<'a, K, V>;
    type VacantEntry = btree_map::VacantEntry<'a, K, V>;

    fn entry(&'a mut self, key: K) -> map::Entry<Self::OccupiedEntry, Self::VacantEntry> {
        match self.entry(key) {
            btree_map::Entry::Occupied(e) => map::Entry::Occupied(e),
            btree_map::Entry::Vacant(e) => map::Entry::Vacant(e),
        }
    }
}

impl<'a, K: 'a, V: 'a> map::OccupiedEntry<'a> for btree_map::OccupiedEntry<'a, K, V> where K: Ord {
    type Key = K;
    type Value = V;
    fn get(&self) -> &V { self.get() }
    fn get_mut(&mut self) -> &mut V { self.get_mut() }
    fn into_mut(self) -> &'a mut V { self.into_mut() }
    fn insert(&mut self, value: V) -> V { self.insert(value) }
    fn remove(self) -> V { self.remove() }
}

impl<'a, K: 'a, V: 'a> map::VacantEntry<'a> for btree_map::VacantEntry<'a, K, V> where K: Ord {
    type Key = K;
    type Value = V;
    fn insert(self, value: V) -> &'a mut V { self.insert(value) }
}

impl<T> Len for BTreeSet<T> where T: Ord {
    fn is_empty(&self) -> bool { self.is_empty() }
    fn len(&self) -> usize { self.len() }
}

impl<T> Clear for BTreeSet<T> where T: Ord {
    fn clear(&mut self) { self.clear(); }
}

impl<T> New for BTreeSet<T> where T: Ord {}

impl<T> set::Base for BTreeSet<T> where T: Ord {
    type Item = T;
}

impl<T, Q: ?Sized> set::Contains<Q> for BTreeSet<T> where T: Ord + Borrow<Q>, Q: Ord {
    fn contains(&self, item: &Q) -> bool { self.contains(item) }
}

impl<T, Q: ?Sized> set::Remove<Q> for BTreeSet<T> where T: Ord + Borrow<Q>, Q: Ord {
    fn remove(&mut self, item: &Q) -> bool { self.remove(item) }
}

impl<T> set::Insert for BTreeSet<T> where T: Ord {
    fn insert(&mut self, item: T) -> bool { self.insert(item) }
}

impl<K, V> Len for HashMap<K, V> where K: Eq + Hash {
    fn is_empty(&self) -> bool { self.is_empty() }
    fn len(&self) -> usize { self.len() }
}

impl<K, V> Clear for HashMap<K, V> where K: Eq + Hash {
    fn clear(&mut self) { self.clear(); }
}

impl<K, V> New for HashMap<K, V> where K: Eq + Hash {
    fn with_capacity_hint(capacity: usize) -> Self {
        Self::with_capacity(capacity)
    }
}

impl<K, V> map::Base for HashMap<K, V> where K: Eq + Hash {
    type Key = K;
    type Value = V;
}

impl<K, V, Q: ?Sized> map::Get<Q> for HashMap<K, V> where K: Eq + Hash + Borrow<Q>, Q: Eq + Hash {
    fn contains_key(&self, key: &Q) -> bool { self.contains_key(key) }
    fn get(&self, key: &Q) -> Option<&V> { self.get(key) }
}

impl<K, V, Q: ?Sized> map::GetMut<Q> for HashMap<K, V> where K: Eq + Hash + Borrow<Q>, Q: Eq + Hash {
    fn get_mut(&mut self, key: &Q) -> Option<&mut V> { self.get_mut(key) }
}

impl<K, V, Q: ?Sized> map::Remove<Q> for HashMap<K, V> where K: Eq + Hash + Borrow<Q>, Q: Eq + Hash {
    fn remove(&mut self, key: &Q) -> Option<V> { self.remove(key) }
}

impl<K, V> map::Insert for HashMap<K, V> where K: Eq + Hash {
    fn insert(&mut self, key: K, value: V) -> Option<V> { self.insert(key, value) }
}

impl<'a, K: 'a, V: 'a> map::EntryMap<'a> for HashMap<K, V> where K: Eq + Hash {
    type OccupiedEntry = hash_map::OccupiedEntry<'a, K, V>;
    type VacantEntry = hash_map::VacantEntry<'a, K, V>;

    fn entry(&'a mut self, key: K) -> map::Entry<Self::OccupiedEntry, Self::VacantEntry> {
        match self.entry(key) {
            hash_map::Entry::Occupied(e) => map::Entry::Occupied(e),
            hash_map::Entry::Vacant(e) => map::Entry::Vacant(e),
        }
    }
}

impl<'a, K: 'a, V: 'a> map::OccupiedEntry<'a> for hash_map::OccupiedEntry<'a, K, V>
where
    K: Eq + Hash,
{
    type Key = K;
    type Value = V;
    fn get(&self) -> &V { self.get() }
    fn get_mut(&mut self) -> &mut V { self.get_mut() }
    fn into_mut(self) -> &'a mut V { self.into_mut() }
    fn insert(&mut self, value: V) -> V { self.insert(value) }
    fn remove(self) -> V { self.remove() }
}

impl<'a, K: 'a, V: 'a> map::VacantEntry<'a> for hash_map::VacantEntry<'a, K, V>
where
    K: Eq + Hash,
{
    type Key = K;
    type Value = V;
    fn insert(self, value: V) -> &'a mut V { self.insert(value) }
}

impl<T> Len for HashSet<T> where T: Eq + Hash {
    fn is_empty(&self) -> bool { self.is_empty() }
    fn len(&self) -> usize { self.len() }
}

impl<T> Clear for HashSet<T> where T: Eq + Hash {
    fn clear(&mut self) { self.clear(); }
}

impl<T> New for HashSet<T> where T: Eq + Hash {
    fn with_capacity_hint(capacity: usize) -> Self {
        Self::with_capacity(capacity)
    }
}

impl<T> set::Base for HashSet<T> where T: Eq + Hash {
    type Item = T;
}

impl<T, Q: ?Sized> set::Contains<Q> for HashSet<T> where T: Eq + Hash + Borrow<Q>, Q: Eq + Hash {
    fn contains(&self, item: &Q) -> bool { self.contains(item) }
}

impl<T, Q: ?Sized> set::Remove<Q> for HashSet<T> where T: Eq + Hash + Borrow<Q>, Q: Eq + Hash {
    fn remove(&mut self, item: &Q) -> bool { self.remove(item) }
}

impl<T> set::Insert for HashSet<T> where T: Eq + Hash {
    fn insert(&mut self, item: T) -> bool { self.insert(item) }
}

impl<T> Len for LinkedList<T> {
    fn is_empty(&self) -> bool { self.is_empty() }
    fn len(&self) -> usize { self.len() }
}

impl<T> Clear for LinkedList<T> {
    fn clear(&mut self) { self.clear(); }
}

impl<T> New for LinkedList<T> {}

impl Len for String {
    fn is_empty(&self) -> bool { self.is_empty() }
    fn len(&self) -> usize { self.len() }
}

impl Clear for String {
    fn clear(&mut self) { self.clear(); }
}

impl New for String {
    fn with_capacity_hint(capacity: usize) -> Self {
        Self::with_capacity(capacity)
    }
}

impl Len for str {
    fn is_empty(&self) -> bool { self.is_empty() }
    fn len(&self) -> usize { self.len() }
}

impl<T> Len for Vec<T> {
    fn is_empty(&self) -> bool { self.is_empty() }
    fn len(&self) -> usize { self.len() }
}

impl<T> Clear for Vec<T> {
    fn clear(&mut self) { self.clear(); }
}

impl<T> New for Vec<T> {
    fn with_capacity_hint(capacity: usize) -> Self {
        Self::with_capacity(capacity)
    }
}

impl<T> Len for VecDeque<T> {
    fn is_empty(&self) -> bool { self.is_empty() }
    fn len(&self) -> usize { self.len() }
}

impl<T> Clear for VecDeque<T> {
    fn clear(&mut self) { self.clear(); }
}

impl<T> New for VecDeque<T> {
    fn with_capacity_hint(capacity: usize) -> Self {
        Self::with_capacity(capacity)
    }
}

#[test]
fn test() {
    let chars = ['a', 'a', 'b', 'c', 'a', 'b'];

    let counts: BTreeMap<_, _> = map::count(chars.iter().cloned());
    assert_eq!(counts[&'a'], 3);
    assert_eq!(counts[&'b'], 2);
    assert_eq!(counts[&'c'], 1);

    let counts: HashMap<_, _> = map::count(chars.iter().cloned());
    assert_eq!(counts[&'a'], 3);
    assert_eq!(counts[&'b'], 2);
    assert_eq!(counts[&'c'], 1);
}
