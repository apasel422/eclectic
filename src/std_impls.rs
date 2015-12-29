use super::*;

use std::borrow::Borrow;
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, LinkedList, VecDeque};
use std::collections::{btree_map, hash_map};
use std::hash::Hash;
use std::mem::{replace, swap};

impl<T> Collection for [T] {
    type Item = T;
    fn is_empty(&self) -> bool { self.is_empty() }
    fn len(&self) -> usize { self.len() }
}

impl<T> list::List for [T] {
    fn get(&self, index: usize) -> Option<&T> { self.get(index) }
    fn get_mut(&mut self, index: usize) -> Option<&mut T> { self.get_mut(index) }
    fn swap(&mut self, i: usize, j: usize) { self.swap(i, j); }
}

impl<T: Ord> Collection for BinaryHeap<T> {
    type Item = T;
    fn is_empty(&self) -> bool { self.is_empty() }
    fn len(&self) -> usize { self.len() }
}

impl<T: Ord> collection::Insert for BinaryHeap<T> {
    fn append(&mut self, other: &mut Self) { self.extend(other.drain()); }
}

impl<T: Ord> collection::Remove for BinaryHeap<T> {
    fn clear(&mut self) { self.clear(); }
}

impl<K: Ord, V> Collection for BTreeMap<K, V> {
    type Item = (K, V);
    fn is_empty(&self) -> bool { self.is_empty() }
    fn len(&self) -> usize { self.len() }
}

impl<K: Ord, V> collection::Insert for BTreeMap<K, V> {
    fn append(&mut self, other: &mut Self) { self.extend(replace(other, Self::new())); }
}

impl<K: Ord, V> collection::Remove for BTreeMap<K, V> {
    fn clear(&mut self) { self.clear(); }
}

impl<K: Ord, V> map::Map for BTreeMap<K, V> {
    type Key = K;
    type Value = V;

    fn entry<'a>(&'a mut self, key: Self::Key) -> map::Entry<'a, Self::Key, Self::Value> {
        match self.entry(key) {
            btree_map::Entry::Occupied(e) => map::Entry::Occupied(Box::new(e)),
            btree_map::Entry::Vacant(e) => map::Entry::Vacant(Box::new(e)),
        }
    }
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

impl<'a, K: 'a + Ord, V: 'a> map::OccupiedEntry for btree_map::OccupiedEntry<'a, K, V> {
    type Key = K;
    type Value = V;
    type MutValue = &'a mut V;
    fn get(&self) -> &V { self.get() }
    fn get_mut(&mut self) -> &mut V { self.get_mut() }
    fn into_mut(self: Box<Self>) -> &'a mut V { (*self).into_mut() }
    fn insert(&mut self, value: V) -> V { self.insert(value) }
    fn remove(self: Box<Self>) -> V { (*self).remove() }
}

impl<'a, K: 'a + Ord, V: 'a> map::VacantEntry for btree_map::VacantEntry<'a, K, V> {
    type Key = K;
    type Value = V;
    type MutValue = &'a mut V;
    fn insert(self: Box<Self>, value: V) -> &'a mut V { (*self).insert(value) }
}

impl<T: Ord> Collection for BTreeSet<T> {
    type Item = T;
    fn is_empty(&self) -> bool { self.is_empty() }
    fn len(&self) -> usize { self.len() }
}

impl<T: Ord> collection::Insert for BTreeSet<T> {
    fn append(&mut self, other: &mut Self) { self.extend(replace(other, Self::new())); }
}

impl<T: Ord> collection::Remove for BTreeSet<T> {
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

impl<K: Eq + Hash, V> collection::Insert for HashMap<K, V> {
    fn append(&mut self, other: &mut Self) { self.extend(other.drain()); }
}

impl<K: Eq + Hash, V> collection::Remove for HashMap<K, V> {
    fn clear(&mut self) { self.clear(); }
}

impl<K: Eq + Hash, V> map::Map for HashMap<K, V> {
    type Key = K;
    type Value = V;

    fn entry<'a>(&'a mut self, key: Self::Key) -> map::Entry<'a, Self::Key, Self::Value> {
        match self.entry(key) {
            hash_map::Entry::Occupied(e) => map::Entry::Occupied(Box::new(e)),
            hash_map::Entry::Vacant(e) => map::Entry::Vacant(Box::new(e)),
        }
    }
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

impl<'a, K: 'a + Eq + Hash, V: 'a> map::OccupiedEntry for hash_map::OccupiedEntry<'a, K, V> {
    type Key = K;
    type Value = V;
    type MutValue = &'a mut V;
    fn get(&self) -> &V { self.get() }
    fn get_mut(&mut self) -> &mut V { self.get_mut() }
    fn into_mut(self: Box<Self>) -> &'a mut V { (*self).into_mut() }
    fn insert(&mut self, value: V) -> V { self.insert(value) }
    fn remove(self: Box<Self>) -> V { (*self).remove() }
}

impl<'a, K: 'a + Eq + Hash, V: 'a> map::VacantEntry for hash_map::VacantEntry<'a, K, V> {
    type Key = K;
    type Value = V;
    type MutValue = &'a mut V;
    fn insert(self: Box<Self>, value: V) -> &'a mut V { (*self).insert(value) }
}

impl<T: Eq + Hash> Collection for HashSet<T> {
    type Item = T;
    fn is_empty(&self) -> bool { self.is_empty() }
    fn len(&self) -> usize { self.len() }
}

impl<T: Eq + Hash> collection::Insert for HashSet<T> {
    fn append(&mut self, other: &mut Self) { self.extend(other.drain()); }
}

impl<T: Eq + Hash> collection::Remove for HashSet<T> {
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

impl<T> collection::Insert for LinkedList<T> {
    fn append(&mut self, other: &mut Self) {
        self.append(other);
    }
}

impl<T> collection::Remove for LinkedList<T> {
    fn clear(&mut self) { self.clear(); }
}

impl<T> list::List for LinkedList<T> {
    fn get(&self, index: usize) -> Option<&T> { self.iter().nth(index) }
    fn get_mut(&mut self, index: usize) -> Option<&mut T> { self.iter_mut().nth(index) }

    fn swap(&mut self, i: usize, j: usize) {
        assert!(i < self.len() && j < self.len());

        let (i, j) =
            if i < j {
                (i, j - i - 1)
            } else if i > j {
                (j, i - j - 1)
            } else {
                return;
            };

        let mut it = self.iter_mut().skip(i);
        let i = it.next().unwrap();
        let mut it = it.skip(j);
        let j = it.next().unwrap();
        swap(i, j);
    }
}

impl<T> list::Insert for LinkedList<T> {
    fn insert(&mut self, index: usize, item: T) {
        assert!(index <= self.len());
        let mut it = self.iter_mut();
        for _ in 0..index { it.next(); }
        it.insert_next(item);
    }

    fn push(&mut self, item: T) { self.push_back(item); }
}

impl<T> seq::PushBack for LinkedList<T> {
    fn push_back(&mut self, item: T) { self.push_back(item); }
}

impl<T> seq::Queue for LinkedList<T> {
    fn front(&self) -> Option<&T> { self.front() }
    fn front_mut(&mut self) -> Option<&mut T> { self.front_mut() }
    fn pop_front(&mut self) -> Option<T> { self.pop_front() }
}

impl<T> seq::Stack for LinkedList<T> {
    fn back(&self) -> Option<&T> { self.back() }
    fn back_mut(&mut self) -> Option<&mut T> { self.back_mut() }
    fn pop_back(&mut self) -> Option<T> { self.pop_back() }
}

impl<T> seq::Deque for LinkedList<T> {
    fn push_front(&mut self, item: T) { self.push_front(item); }
}

impl<T> Collection for Vec<T> {
    type Item = T;
    fn is_empty(&self) -> bool { self.is_empty() }
    fn len(&self) -> usize { self.len() }
}

impl<T> collection::Insert for Vec<T> {
    fn append(&mut self, other: &mut Self) { self.append(other); }
}

impl<T> collection::Remove for Vec<T> {
    fn clear(&mut self) { self.clear(); }
}

impl<T> list::List for Vec<T> {
    fn get(&self, index: usize) -> Option<&T> { <[T]>::get(self, index) }
    fn get_mut(&mut self, index: usize) -> Option<&mut T> { <[T]>::get_mut(self, index) }
    fn swap(&mut self, i: usize, j: usize) { <[T]>::swap(self, i, j); }
}

impl<T> list::Insert for Vec<T> {
    fn insert(&mut self, index: usize, item: T) { self.insert(index, item); }
    fn push(&mut self, item: T) { self.push(item); }
}

impl<T> list::Remove for Vec<T> {
    fn pop(&mut self) -> Option<T> { self.pop() }

    fn remove(&mut self, index: usize) -> Option<T> {
        if index < self.len() {
            Some(self.remove(index))
        } else {
            None
        }
    }

    fn split_off(&mut self, index: usize) -> Self { self.split_off(index) }

    fn swap_remove(&mut self, index: usize) -> Option<T> {
        if index < self.len() {
            Some(self.swap_remove(index))
        } else {
            None
        }
    }
}

impl<T> seq::PushBack for Vec<T> {
    fn push_back(&mut self, item: T) { self.push(item); }
}

impl<T> seq::Stack for Vec<T> {
    fn back(&self) -> Option<&T> { self.last() }
    fn back_mut(&mut self) -> Option<&mut T> { self.last_mut() }
    fn pop_back(&mut self) -> Option<T> { self.pop() }
}

impl<T> Collection for VecDeque<T> {
    type Item = T;
    fn is_empty(&self) -> bool { self.is_empty() }
    fn len(&self) -> usize { self.len() }
}

impl<T> collection::Insert for VecDeque<T> {
    fn append(&mut self, other: &mut Self) { self.append(other); }
}

impl<T> collection::Remove for VecDeque<T> {
    fn clear(&mut self) { self.clear(); }
}

impl<T> list::List for VecDeque<T> {
    fn get(&self, index: usize) -> Option<&T> { self.get(index) }
    fn get_mut(&mut self, index: usize) -> Option<&mut T> { self.get_mut(index) }
    fn swap(&mut self, i: usize, j: usize) { self.swap(i, j); }
}

impl<T> list::Insert for VecDeque<T> {
    fn insert(&mut self, index: usize, item: T) { self.insert(index, item); }
    fn push(&mut self, item: T) { self.push_back(item); }
}

impl<T> list::Remove for VecDeque<T> {
    fn pop(&mut self) -> Option<T> { self.pop_back() }
    fn remove(&mut self, index: usize) -> Option<T> { self.remove(index) }
    fn split_off(&mut self, index: usize) -> Self { self.split_off(index) }
    fn swap_remove(&mut self, index: usize) -> Option<T> { self.swap_remove_back(index) }
}

impl<T> seq::PushBack for VecDeque<T> {
    fn push_back(&mut self, item: T) { self.push_back(item); }
}

impl<T> seq::Queue for VecDeque<T> {
    fn front(&self) -> Option<&T> { self.front() }
    fn front_mut(&mut self) -> Option<&mut T> { self.front_mut() }
    fn pop_front(&mut self) -> Option<T> { self.pop_front() }
}

impl<T> seq::Stack for VecDeque<T> {
    fn back(&self) -> Option<&T> { self.back() }
    fn back_mut(&mut self) -> Option<&mut T> { self.back_mut() }
    fn pop_back(&mut self) -> Option<T> { self.pop_back() }
}

impl<T> seq::Deque for VecDeque<T> {
    fn push_front(&mut self, item: T) { self.push_front(item); }
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

#[test]
fn test_linked_list() {
    use list::{Insert, List};

    let mut l: LinkedList<_> = vec![1, 2, 3].into_iter().collect();
    assert_eq!(l.get(1), Some(&2));
    assert_eq!(l.get_mut(2), Some(&mut 3));
    assert_eq!(l.get(3), None);

    l.swap(0, 1);
    assert!(l.iter().eq(&[2, 1, 3]));
    l.swap(0, 0);
    assert!(l.iter().eq(&[2, 1, 3]));
    l.swap(1, 0);
    assert!(l.iter().eq(&[1, 2, 3]));

    l.insert(0, 0);
    assert!(l.iter().eq(&[0, 1, 2, 3]));
    l.insert(4, 4);
    assert!(l.iter().eq(&[0, 1, 2, 3, 4]));
}
