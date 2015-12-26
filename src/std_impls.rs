use super::*;

use std::borrow::Borrow;
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, LinkedList, VecDeque};
use std::collections::{btree_map, hash_map};
use std::hash::Hash;
use std::mem::replace;

impl<T> Collection for [T] {
    type Item = T;
    fn is_empty(&self) -> bool { self.is_empty() }
    fn len(&self) -> usize { self.len() }
}

impl<T: Ord> Collection for BinaryHeap<T> {
    type Item = T;
    fn is_empty(&self) -> bool { self.is_empty() }
    fn len(&self) -> usize { self.len() }
}

impl<T: Ord> Remove for BinaryHeap<T> {
    fn clear(&mut self) { self.clear(); }
}

impl<K: Ord, V> Collection for BTreeMap<K, V> {
    type Item = (K, V);
    fn is_empty(&self) -> bool { self.is_empty() }
    fn len(&self) -> usize { self.len() }
}

impl<K: Ord, V> Insert for BTreeMap<K, V> {
    fn append(&mut self, other: &mut Self) { self.extend(replace(other, Self::new())); }
}

impl<K: Ord, V> Remove for BTreeMap<K, V> {
    fn clear(&mut self) { self.clear(); }
}

impl<K: Ord, V> map::Map for BTreeMap<K, V> {
    type Key = K;
    type Value = V;
}

impl<K: Ord + Borrow<Q>, V, Q: ?Sized + Ord> map::Get<Q> for BTreeMap<K, V> {
    fn contains_key(&self, key: &Q) -> bool { self.contains_key(key) }
    fn get(&self, key: &Q) -> Option<&V> { self.get(key) }
    fn get_mut(&mut self, key: &Q) -> Option<&mut V> { self.get_mut(key) }
}

impl<K: Ord + Borrow<Q>, V, Q: ?Sized + Ord> map::Remove<Q> for BTreeMap<K, V> {
    fn remove(&mut self, key: &Q) -> Option<V> { self.remove(key) }
}

impl<K: Ord, V> map::Insert for BTreeMap<K, V> {
    fn insert(&mut self, key: K, value: V) -> Option<V> { self.insert(key, value) }
}

impl<'a, K: 'a + Ord, V: 'a> map::EntryMap<'a> for BTreeMap<K, V> {
    type OccupiedEntry = btree_map::OccupiedEntry<'a, K, V>;
    type VacantEntry = btree_map::VacantEntry<'a, K, V>;

    fn entry(&'a mut self, key: K) -> map::Entry<Self> {
        match self.entry(key) {
            btree_map::Entry::Occupied(e) => map::Entry::Occupied(e),
            btree_map::Entry::Vacant(e) => map::Entry::Vacant(e),
        }
    }
}

impl<'a, K: 'a + Ord, V: 'a> map::OccupiedEntry<'a> for btree_map::OccupiedEntry<'a, K, V> {
    type Map = BTreeMap<K, V>;
    fn get(&self) -> &V { self.get() }
    fn get_mut(&mut self) -> &mut V { self.get_mut() }
    fn into_mut(self) -> &'a mut V { self.into_mut() }
    fn insert(&mut self, value: V) -> V { self.insert(value) }
    fn remove(self) -> V { self.remove() }
}

impl<'a, K: 'a + Ord, V: 'a> map::VacantEntry<'a> for btree_map::VacantEntry<'a, K, V> {
    type Map = BTreeMap<K, V>;
    fn insert(self, value: V) -> &'a mut V { self.insert(value) }
}

impl<T: Ord> Collection for BTreeSet<T> {
    type Item = T;
    fn is_empty(&self) -> bool { self.is_empty() }
    fn len(&self) -> usize { self.len() }
}

impl<T: Ord> Insert for BTreeSet<T> {
    fn append(&mut self, other: &mut Self) { self.extend(replace(other, Self::new())); }
}

impl<T: Ord> Remove for BTreeSet<T> {
    fn clear(&mut self) { self.clear(); }
}

impl<T: Ord> set::Set for BTreeSet<T> {
    fn is_disjoint(&self, other: &Self) -> bool {
        self.is_disjoint(other)
    }

    fn is_subset(&self, other: &Self) -> bool {
        self.is_subset(other)
    }

    fn is_superset(&self, other: &Self) -> bool {
        self.is_superset(other)
    }
}

impl<T: Ord + Borrow<Q>, Q: ?Sized + Ord> set::Get<Q> for BTreeSet<T> {
    fn contains(&self, item: &Q) -> bool { self.contains(item) }
    fn get(&self, item: &Q) -> Option<&T> { self.get(item) }
}

impl<T: Ord + Borrow<Q>, Q: ?Sized + Ord> set::Remove<Q> for BTreeSet<T> {
    fn remove(&mut self, item: &Q) -> bool { self.remove(item) }
    fn take(&mut self, item: &Q) -> Option<T> { self.take(item) }
}

impl<T: Ord> set::Insert for BTreeSet<T> {
    fn insert(&mut self, item: T) -> bool { self.insert(item) }
    fn replace(&mut self, item: T) -> Option<T> { self.replace(item) }
}

impl<K: Eq + Hash, V> Collection for HashMap<K, V> {
    type Item = (K, V);
    fn is_empty(&self) -> bool { self.is_empty() }
    fn len(&self) -> usize { self.len() }
}

impl<K: Eq + Hash, V> Insert for HashMap<K, V> {
    fn append(&mut self, other: &mut Self) { self.extend(other.drain()); }
}

impl<K: Eq + Hash, V> Remove for HashMap<K, V> {
    fn clear(&mut self) { self.clear(); }
}

impl<K: Eq + Hash, V> map::Map for HashMap<K, V> {
    type Key = K;
    type Value = V;
}

impl<K: Eq + Hash + Borrow<Q>, V, Q: ?Sized + Eq + Hash> map::Get<Q> for HashMap<K, V> {
    fn contains_key(&self, key: &Q) -> bool { self.contains_key(key) }
    fn get(&self, key: &Q) -> Option<&V> { self.get(key) }
    fn get_mut(&mut self, key: &Q) -> Option<&mut V> { self.get_mut(key) }
}

impl<K: Eq + Hash + Borrow<Q>, V, Q: ?Sized + Eq + Hash> map::Remove<Q> for HashMap<K, V> {
    fn remove(&mut self, key: &Q) -> Option<V> { self.remove(key) }
}

impl<K: Eq + Hash, V> map::Insert for HashMap<K, V> {
    fn insert(&mut self, key: K, value: V) -> Option<V> { self.insert(key, value) }
}

impl<'a, K: 'a + Eq + Hash, V: 'a> map::EntryMap<'a> for HashMap<K, V> {
    type OccupiedEntry = hash_map::OccupiedEntry<'a, K, V>;
    type VacantEntry = hash_map::VacantEntry<'a, K, V>;

    fn entry(&'a mut self, key: K) -> map::Entry<Self> {
        match self.entry(key) {
            hash_map::Entry::Occupied(e) => map::Entry::Occupied(e),
            hash_map::Entry::Vacant(e) => map::Entry::Vacant(e),
        }
    }
}

impl<'a, K: 'a + Eq + Hash, V: 'a> map::OccupiedEntry<'a> for hash_map::OccupiedEntry<'a, K, V> {
    type Map = HashMap<K, V>;
    fn get(&self) -> &V { self.get() }
    fn get_mut(&mut self) -> &mut V { self.get_mut() }
    fn into_mut(self) -> &'a mut V { self.into_mut() }
    fn insert(&mut self, value: V) -> V { self.insert(value) }
    fn remove(self) -> V { self.remove() }
}

impl<'a, K: 'a + Eq + Hash, V: 'a> map::VacantEntry<'a> for hash_map::VacantEntry<'a, K, V> {
    type Map = HashMap<K, V>;
    fn insert(self, value: V) -> &'a mut V { self.insert(value) }
}

impl<T: Eq + Hash> Collection for HashSet<T> {
    type Item = T;
    fn is_empty(&self) -> bool { self.is_empty() }
    fn len(&self) -> usize { self.len() }
}

impl<T: Eq + Hash> Insert for HashSet<T> {
    fn append(&mut self, other: &mut Self) { self.extend(other.drain()); }
}

impl<T: Eq + Hash> Remove for HashSet<T> {
    fn clear(&mut self) { self.clear(); }
}

impl<T: Eq + Hash> set::Set for HashSet<T> {
    fn is_disjoint(&self, other: &Self) -> bool {
        self.is_disjoint(other)
    }

    fn is_subset(&self, other: &Self) -> bool {
        self.is_subset(other)
    }

    fn is_superset(&self, other: &Self) -> bool {
        self.is_superset(other)
    }
}

impl<T: Eq + Hash + Borrow<Q>, Q: ?Sized + Eq + Hash> set::Get<Q> for HashSet<T> {
    fn contains(&self, item: &Q) -> bool { self.contains(item) }
    fn get(&self, item: &Q) -> Option<&T> { self.get(item) }
}

impl<T: Eq + Hash + Borrow<Q>, Q: ?Sized + Eq + Hash> set::Remove<Q> for HashSet<T> {
    fn remove(&mut self, item: &Q) -> bool { self.remove(item) }
    fn take(&mut self, item: &Q) -> Option<T> { self.take(item) }
}

impl<T: Eq + Hash> set::Insert for HashSet<T> {
    fn insert(&mut self, item: T) -> bool { self.insert(item) }
    fn replace(&mut self, item: T) -> Option<T> { self.replace(item) }
}

impl<T> Collection for LinkedList<T> {
    type Item = T;
    fn is_empty(&self) -> bool { self.is_empty() }
    fn len(&self) -> usize { self.len() }
}

impl<T> Remove for LinkedList<T> {
    fn clear(&mut self) { self.clear(); }
}

impl<T> Collection for Vec<T> {
    type Item = T;
    fn is_empty(&self) -> bool { self.is_empty() }
    fn len(&self) -> usize { self.len() }
}

impl<T> Remove for Vec<T> {
    fn clear(&mut self) { self.clear(); }
}

impl<T> Collection for VecDeque<T> {
    type Item = T;
    fn is_empty(&self) -> bool { self.is_empty() }
    fn len(&self) -> usize { self.len() }
}

impl<T> Remove for VecDeque<T> {
    fn clear(&mut self) { self.clear(); }
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
