extern crate linear_map;

use super::*;

use self::linear_map::LinearMap;
use std::borrow::Borrow;

impl<K, V> Len for LinearMap<K, V> where K: Eq {
    fn is_empty(&self) -> bool { self.is_empty() }
    fn len(&self) -> usize { self.len() }
}

impl<K, V> Clear for LinearMap<K, V> where K: Eq {
    fn clear(&mut self) { self.clear(); }
}

impl<K, V> map::Base for LinearMap<K, V> where K: Eq {
    type Key = K;
    type Value = V;
}

impl<K, V, Q: ?Sized> map::Get<Q> for LinearMap<K, V> where K: Eq + Borrow<Q>, Q: Eq {
    fn contains_key(&self, key: &Q) -> bool { self.contains_key(key) }
    fn get(&self, key: &Q) -> Option<&V> { self.get(key) }
}

impl<K, V, Q: ?Sized> map::GetMut<Q> for LinearMap<K, V> where K: Eq + Borrow<Q>, Q: Eq {
    fn get_mut(&mut self, key: &Q) -> Option<&mut V> { self.get_mut(key) }
}

impl<K, V, Q: ?Sized> map::Remove<Q> for LinearMap<K, V> where K: Eq + Borrow<Q>, Q: Eq {
    fn remove(&mut self, key: &Q) -> Option<V> { self.remove(key) }
}

impl<K, V> map::Insert for LinearMap<K, V> where K: Eq {
    fn insert(&mut self, key: K, value: V) -> Option<V> { self.insert(key, value) }
}
