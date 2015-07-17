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

impl<'a, V: 'a> map::EntryMap<'a> for VecMap<V> {
    type OccupiedEntry = vec_map::OccupiedEntry<'a, V>;
    type VacantEntry = vec_map::VacantEntry<'a, V>;

    fn entry(&'a mut self, key: usize) -> map::Entry<Self::OccupiedEntry, Self::VacantEntry> {
        match self.entry(key) {
            vec_map::Entry::Occupied(e) => map::Entry::Occupied(e),
            vec_map::Entry::Vacant(e) => map::Entry::Vacant(e),
        }
    }
}

impl<'a, V: 'a> map::OccupiedEntry<'a> for vec_map::OccupiedEntry<'a, V> {
    type Key = usize;
    type Value = V;
    fn get(&self) -> &V { self.get() }
    fn get_mut(&mut self) -> &mut V { self.get_mut() }
    fn into_mut(self) -> &'a mut V { self.into_mut() }
    fn insert(&mut self, value: V) -> V { self.insert(value) }
    fn remove(self) -> V { self.remove() }
}

impl<'a, V: 'a> map::VacantEntry<'a> for vec_map::VacantEntry<'a, V> {
    type Key = usize;
    type Value = V;
    fn insert(self, value: V) -> &'a mut V { self.insert(value) }
}

#[test]
fn test() {
    let chars = [1, 1, 2, 3, 1, 2];

    let counts: VecMap<_> = map::count(chars.iter().cloned());
    assert_eq!(counts[1], 3);
    assert_eq!(counts[2], 2);
    assert_eq!(counts[3], 1);
}
