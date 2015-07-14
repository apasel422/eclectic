extern crate vec_map;

use super::*;

use self::vec_map::VecMap;

impl<V> Len for VecMap<V> {
    fn is_empty(&self) -> bool { self.is_empty() }
    fn len(&self) -> usize { self.len() }
}

impl<V> Clear for VecMap<V> {
    fn clear(&mut self) { self.clear(); }
}

impl<V> map::Base for VecMap<V> {
    type Key = usize;
    type Value = V;
}

impl<V> map::Get<usize> for VecMap<V> {
    fn contains_key(&self, key: &usize) -> bool { self.contains_key(key) }
    fn get(&self, key: &usize) -> Option<&V> { self.get(key) }
}

impl<V> map::GetMut<usize> for VecMap<V> {
    fn get_mut(&mut self, key: &usize) -> Option<&mut V> { self.get_mut(key) }
}

impl<V> map::Remove<usize> for VecMap<V> {
    fn remove(&mut self, key: &usize) -> Option<V> { self.remove(key) }
}

impl<V> map::Insert for VecMap<V> {
    fn insert(&mut self, key: usize, value: V) -> Option<V> { self.insert(key, value) }
}
