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

impl<'a, K: 'a, V: 'a> map::EntryMap<'a> for LinearMap<K, V> where K: Eq {
    type OccupiedEntry = linear_map::OccupiedEntry<'a, K, V>;
    type VacantEntry = linear_map::VacantEntry<'a, K, V>;

    fn entry(&'a mut self, key: K) -> map::Entry<Self::OccupiedEntry, Self::VacantEntry> {
        match self.entry(key) {
            linear_map::Entry::Occupied(e) => map::Entry::Occupied(e),
            linear_map::Entry::Vacant(e) => map::Entry::Vacant(e),
        }
    }
}

impl<'a, K: 'a, V: 'a> map::OccupiedEntry<'a> for linear_map::OccupiedEntry<'a, K, V> where K: Eq {
    type Key = K;
    type Value = V;
    fn get(&self) -> &V { self.get() }
    fn get_mut(&mut self) -> &mut V { self.get_mut() }
    fn into_mut(self) -> &'a mut V { self.into_mut() }
    fn insert(&mut self, value: V) -> V { self.insert(value) }
    fn remove(self) -> V { self.remove() }
}

impl<'a, K: 'a, V: 'a> map::VacantEntry<'a> for linear_map::VacantEntry<'a, K, V> where K: Eq {
    type Key = K;
    type Value = V;
    fn insert(self, value: V) -> &'a mut V { self.insert(value) }
}

#[cfg(feature = "nightly")]
#[test]
fn test() {
    let chars = ['a', 'a', 'b', 'c', 'a', 'b'];

    let counts: LinearMap<_, _> = map::count(chars.iter().cloned());
    assert_eq!(counts[&'a'], 3);
    assert_eq!(counts[&'b'], 2);
    assert_eq!(counts[&'c'], 1);
}
