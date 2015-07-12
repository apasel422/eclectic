extern crate vec_map;

use self::vec_map::VecMap;
use super::Entry;

impl<V> super::Collection for VecMap<V> {
    fn clear(&mut self) { self.clear(); }
    fn is_empty(&self) -> bool { self.is_empty() }
    fn len(&self) -> usize { self.len() }
}

impl<V> super::BaseMap for VecMap<V> {
    type Key = usize;
    type Value = V;
}

impl<V> super::Map for VecMap<V> {
    fn insert(&mut self, key: usize, value: V) -> Option<V> { self.insert(key, value) }
}

impl<V> super::MapLookup<usize> for VecMap<V> {
    fn contains_key(&self, key: &usize) -> bool { self.contains_key(key) }
    fn get(&self, key: &usize) -> Option<&V> { self.get(key) }
    fn get_mut(&mut self, key: &usize) -> Option<&mut V> { self.get_mut(key) }
    fn remove(&mut self, key: &usize) -> Option<V> { self.remove(key) }
}

impl<'a, V: 'a> super::EntryMap<'a> for VecMap<V> {
    type Occupied = vec_map::OccupiedEntry<'a, V>;
    type Vacant = vec_map::VacantEntry<'a, V>;

    fn entry(&'a mut self, key: usize) -> Entry<Self::Occupied, Self::Vacant> {
        match self.entry(key) {
            vec_map::Entry::Occupied(e) => Entry::Occupied(e),
            vec_map::Entry::Vacant(e) => Entry::Vacant(e),
        }
    }
}

impl<'a, V: 'a> super::OccupiedEntry<'a> for vec_map::OccupiedEntry<'a, V> {
    type Key = usize;
    type Value = V;
    fn get(&self) -> &V { self.get() }
    fn get_mut(&mut self) -> &mut V { self.get_mut() }
    fn insert(&mut self, value: V) -> V { self.insert(value) }
    fn into_mut(self) -> &'a mut V { self.into_mut() }
    fn remove(self) -> V { self.remove() }
}

impl<'a, V: 'a> super::VacantEntry<'a> for vec_map::VacantEntry<'a, V> {
    type Key = usize;
    type Value = V;
    fn insert(self, value: V) -> &'a mut V { self.insert(value) }
}
