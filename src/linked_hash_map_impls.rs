extern crate linked_hash_map;

use self::linked_hash_map::LinkedHashMap;
use std::borrow::Borrow;
use std::collections::hash_state::HashState;
use std::hash::Hash;

impl<K, V, S> super::Collection for LinkedHashMap<K, V, S> where K: Eq + Hash, S: HashState {
    fn clear(&mut self) { self.clear(); }
    fn is_empty(&self) -> bool { self.is_empty() }
    fn len(&self) -> usize { self.len() }
}

impl<K, V, S> super::BaseMap for LinkedHashMap<K, V, S> where K: Eq + Hash, S: HashState {
    type Key = K;
    type Value = V;
}

impl<K, V, S, Q: ?Sized> super::MapLookup<Q> for LinkedHashMap<K, V, S>
where
    K: Eq + Hash + Borrow<Q>,
    S: HashState,
    Q: Eq + Hash,
{
    fn contains_key(&self, key: &Q) -> bool { self.contains_key(key) }
    fn get(&self, key: &Q) -> Option<&V> { self.get(key) }
    fn get_mut(&mut self, key: &Q) -> Option<&mut V> { self.get_mut(key) }
    fn remove(&mut self, key: &Q) -> Option<V> { self.remove(key) }
}

impl<K, V, S> super::Map for LinkedHashMap<K, V, S> where K: Eq + Hash, S: HashState {
    fn insert(&mut self, key: K, value: V) -> Option<V> { self.insert(key, value) }
}
