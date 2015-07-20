extern crate trie;

use super::*;

impl<V> Len for trie::Map<V> {
    fn is_empty(&self) -> bool { self.is_empty() }
    fn len(&self) -> usize { self.len() }
}

impl<V> Clear for trie::Map<V> {
    fn clear(&mut self) { self.clear(); }
}

impl<V> map::Base for trie::Map<V> {
    type Key = usize;
    type Value = V;
}

impl<V> map::Get<usize> for trie::Map<V> {
    fn contains_key(&self, key: &usize) -> bool { self.contains_key(key) }
    fn get(&self, key: &usize) -> Option<&V> { self.get(key) }
}

impl<V> map::GetMut<usize> for trie::Map<V> {
    fn get_mut(&mut self, key: &usize) -> Option<&mut V> { self.get_mut(key) }
}

impl<V> map::Remove<usize> for trie::Map<V> {
    fn remove(&mut self, key: &usize) -> Option<V> { self.remove(key) }
}

impl<V> map::Insert for trie::Map<V> {
    fn insert(&mut self, key: usize, value: V) -> Option<V> { self.insert(key, value) }
}

impl<'a, V: 'a> map::EntryMap<'a> for trie::Map<V> {
    type OccupiedEntry = trie::map::OccupiedEntry<'a, V>;
    type VacantEntry = trie::map::VacantEntry<'a, V>;

    fn entry(&'a mut self, key: usize) -> map::Entry<Self::OccupiedEntry, Self::VacantEntry> {
        match self.entry(key) {
            trie::map::Entry::Occupied(e) => map::Entry::Occupied(e),
            trie::map::Entry::Vacant(e) => map::Entry::Vacant(e),
        }
    }
}

impl<'a, V: 'a> map::OccupiedEntry<'a> for trie::map::OccupiedEntry<'a, V> {
    type Key = usize;
    type Value = V;
    fn get(&self) -> &V { self.get() }
    fn get_mut(&mut self) -> &mut V { self.get_mut() }
    fn into_mut(self) -> &'a mut V { self.into_mut() }
    fn insert(&mut self, value: V) -> V { self.insert(value) }
    fn remove(self) -> V { self.remove() }
}

impl<'a, V: 'a> map::VacantEntry<'a> for trie::map::VacantEntry<'a, V> {
    type Key = usize;
    type Value = V;
    fn insert(self, value: V) -> &'a mut V { self.insert(value) }
}

impl Len for trie::Set {
    fn is_empty(&self) -> bool { self.is_empty() }
    fn len(&self) -> usize { self.len() }
}

impl Clear for trie::Set {
    fn clear(&mut self) { self.clear(); }
}

impl set::Base for trie::Set {
    type Item = usize;
}

impl set::Contains<usize> for trie::Set {
    fn contains(&self, item: &usize) -> bool { self.contains(item) }
}

impl set::Remove<usize> for trie::Set {
    fn remove(&mut self, item: &usize) -> bool { self.remove(item) }
}

impl set::Insert for trie::Set {
    fn insert(&mut self, item: usize) -> bool { self.insert(item) }
}

#[cfg(feature = "nightly")]
#[test]
fn test() {
    let chars = [1, 1, 2, 3, 1, 2];

    let counts: trie::Map<_> = map::count(chars.iter().cloned());
    assert_eq!(counts[&1], 3);
    assert_eq!(counts[&2], 2);
    assert_eq!(counts[&3], 1);
}
