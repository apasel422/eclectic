use super::*;

use std::borrow::Borrow;
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, LinkedList, VecDeque};
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

impl<K, V> Len for BTreeMap<K, V> where K: Ord {
    fn is_empty(&self) -> bool { self.is_empty() }
    fn len(&self) -> usize { self.len() }
}

impl<K, V> Clear for BTreeMap<K, V> where K: Ord {
    fn clear(&mut self) { self.clear(); }
}

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

impl<T> Len for BTreeSet<T> where T: Ord {
    fn is_empty(&self) -> bool { self.is_empty() }
    fn len(&self) -> usize { self.len() }
}

impl<T> Clear for BTreeSet<T> where T: Ord {
    fn clear(&mut self) { self.clear(); }
}

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

impl<T> Len for HashSet<T> where T: Eq + Hash {
    fn is_empty(&self) -> bool { self.is_empty() }
    fn len(&self) -> usize { self.len() }
}

impl<T> Clear for HashSet<T> where T: Eq + Hash {
    fn clear(&mut self) { self.clear(); }
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

impl Len for String {
    fn is_empty(&self) -> bool { self.is_empty() }
    fn len(&self) -> usize { self.len() }
}

impl Clear for String {
    fn clear(&mut self) { self.clear(); }
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

impl<T> Len for VecDeque<T> {
    fn is_empty(&self) -> bool { self.is_empty() }
    fn len(&self) -> usize { self.len() }
}

impl<T> Clear for VecDeque<T> {
    fn clear(&mut self) { self.clear(); }
}
