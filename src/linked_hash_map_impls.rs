extern crate linked_hash_map;

use super::*;

use self::linked_hash_map::LinkedHashMap;
use std::borrow::Borrow;
use std::hash::Hash;

impl<K, V> Len for LinkedHashMap<K, V> where K: Eq + Hash {
    fn is_empty(&self) -> bool { self.is_empty() }
    fn len(&self) -> usize { self.len() }
}

impl<K, V> Clear for LinkedHashMap<K, V> where K: Eq + Hash {
    fn clear(&mut self) { self.clear(); }
}

impl<K, V> map::Base for LinkedHashMap<K, V> where K: Eq + Hash {
    type Key = K;
    type Value = V;
}

impl<K, V, Q: ?Sized> map::Get<Q> for LinkedHashMap<K, V> where K: Eq + Hash + Borrow<Q>, Q: Eq + Hash {
    fn contains_key(&self, key: &Q) -> bool { self.contains_key(key) }
    fn get(&self, key: &Q) -> Option<&V> { self.get(key) }
}

impl<K, V, Q: ?Sized> map::GetMut<Q> for LinkedHashMap<K, V> where K: Eq + Hash + Borrow<Q>, Q: Eq + Hash {
    fn get_mut(&mut self, key: &Q) -> Option<&mut V> { self.get_mut(key) }
}

impl<K, V, Q: ?Sized> map::Remove<Q> for LinkedHashMap<K, V> where K: Eq + Hash + Borrow<Q>, Q: Eq + Hash {
    fn remove(&mut self, key: &Q) -> Option<V> { self.remove(key) }
}

impl<K, V> map::Insert for LinkedHashMap<K, V> where K: Eq + Hash {
    fn insert(&mut self, key: K, value: V) -> Option<V> { self.insert(key, value) }
}
