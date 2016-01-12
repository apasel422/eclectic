use std::borrow::Borrow;
use std::collections::*;
use std::hash::Hash;
use std::mem;
use std::ops::Range;
use super::*;

impl<T> Mut for [T] {}

impl<T> Collection for [T] {
    type Item = T;

    fn len(&self) -> usize {
        self.len()
    }

    fn capacity(&self) -> usize {
        self.len()
    }

    fn extend_object(&mut self, _items: &mut Iterator<Item = T>) where Self: Own {
        unimplemented!()
    }

    fn drain<'a>(&'a mut self) -> Box<Iterator<Item = T> + 'a> where Self: Own {
        unimplemented!()
    }

    fn reserve(&mut self, _additional: usize) where Self: Own {
        unimplemented!()
    }

    fn shrink_to_fit(&mut self) where Self: Own {
        unimplemented!()
    }
}

impl<T> Iter for [T] {
    fn iter<'a>(&'a self) -> Box<Iterator<Item = &'a T> + 'a> {
        Box::new(self.iter())
    }

    fn iter_mut<'a>(&'a mut self) -> Box<Iterator<Item = &'a mut T> + 'a> {
        Box::new(self.iter_mut())
    }
}

impl<T> DrainRange<Range<usize>> for [T] {
    fn drain_range<'a>(&'a mut self, _range: Range<usize>)
        -> Box<Iterator<Item = T> + 'a> where Self: Own
    {
        unimplemented!()
    }
}

impl<T> List for [T] {
    fn get(&self, index: usize) -> Option<&T> {
        self.get(index)
    }

    fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        self.get_mut(index)
    }

    fn swap(&mut self, i: usize, j: usize) {
        self.swap(i, j);
    }

    fn insert(&mut self, _index: usize, _item: T) where Self: Own {
        unimplemented!()
    }

    fn remove(&mut self, _index: usize) -> Option<T> where Self: Own {
        unimplemented!()
    }

    fn swap_remove(&mut self, _index: usize) -> Option<T> where Self: Own {
        unimplemented!()
    }
}

impl<K: Ord, V> Mut for BTreeMap<K, V> {}

impl<K: Ord, V> Own for BTreeMap<K, V> {}

impl<K: Ord, V> Collection for BTreeMap<K, V> {
    type Item = (K, V);

    fn len(&self) -> usize {
        self.len()
    }

    fn capacity(&self) -> usize {
        self.len()
    }

    fn append(&mut self, other: &mut Self) {
        self.extend(mem::replace(other, Self::new()));
    }

    fn extend_object(&mut self, items: &mut Iterator<Item = (K, V)>) {
        self.extend(items);
    }

    fn clear(&mut self) {
        self.clear();
    }

    fn drain<'a>(&'a mut self) -> Box<Iterator<Item = (K, V)> + 'a> {
        Box::new(mem::replace(self, Self::new()).into_iter())
    }

    fn reserve(&mut self, _additional: usize) {}

    fn shrink_to_fit(&mut self) {}
}

impl<K: Ord, V> map::Base for BTreeMap<K, V> {
    type Key = K;
    type Value = V;

    fn iter<'a>(&'a self) -> Box<Iterator<Item = (&'a K, &'a V)> + 'a> {
        Box::new(self.iter())
    }

    fn iter_mut<'a>(&'a mut self) -> Box<Iterator<Item = (&'a K, &'a mut V)> + 'a> {
        Box::new(self.iter_mut())
    }

    fn insert(&mut self, key: K, value: V) -> Option<V> {
        self.insert(key, value)
    }

    fn entry<'a>(&'a mut self, key: K) -> map::Entry<'a, K, V> {
        match self.entry(key) {
            btree_map::Entry::Occupied(e) => map::Entry::Occupied(Box::new(e)),
            btree_map::Entry::Vacant(e) => map::Entry::Vacant(Box::new(e)),
        }
    }
}

impl<'a, K: 'a + Ord, V: 'a> map::OccupiedEntry for btree_map::OccupiedEntry<'a, K, V> {
    type Key = K;
    type Value = V;
    type MutValue = &'a mut V;

    fn get(&self) -> &V {
        self.get()
    }

    fn get_mut(&mut self) -> &mut V {
        self.get_mut()
    }

    fn into_mut(self: Box<Self>) -> &'a mut V {
        (*self).into_mut()
    }

    fn remove(self: Box<Self>) -> V {
        (*self).remove()
    }
}

impl<'a, K: 'a + Ord, V: 'a> map::VacantEntry for btree_map::VacantEntry<'a, K, V> {
    type Key = K;
    type Value = V;
    type MutValue = &'a mut V;

    fn insert(self: Box<Self>, value: V) -> &'a mut V {
        (*self).insert(value)
    }
}

impl<T: Ord> Own for BTreeSet<T> {}

impl<T: Ord> Collection for BTreeSet<T> {
    type Item = T;

    fn len(&self) -> usize {
        self.len()
    }

    fn capacity(&self) -> usize {
        self.len()
    }

    fn append(&mut self, other: &mut Self) {
        self.extend(mem::replace(other, Self::new()));
    }

    fn extend_object(&mut self, items: &mut Iterator<Item = T>) {
        self.extend(items);
    }

    fn clear(&mut self) {
        self.clear();
    }

    fn drain<'a>(&'a mut self) -> Box<Iterator<Item = T> + 'a> {
        Box::new(mem::replace(self, Self::new()).into_iter())
    }

    fn reserve(&mut self, _additional: usize) {}

    fn shrink_to_fit(&mut self) {}
}

impl<T: Ord> Iter for BTreeSet<T> {
    fn iter<'a>(&'a self) -> Box<Iterator<Item = &'a T> + 'a> {
        Box::new(self.iter())
    }

    fn iter_mut<'a>(&'a mut self) -> Box<Iterator<Item = &'a mut T> + 'a> where Self: Mut {
        unimplemented!()
    }
}

impl<T: Ord> set::Base for BTreeSet<T> {
    fn is_disjoint(&self, other: &Self) -> bool {
        self.is_disjoint(other)
    }

    fn is_subset(&self, other: &Self) -> bool {
        self.is_subset(other)
    }

    fn insert(&mut self, item: T) -> bool {
        self.insert(item)
    }

    fn replace(&mut self, item: T) -> Option<T> {
        self.replace(item)
    }
}

impl<T: Ord + Borrow<Q>, Q: ?Sized + Ord> Set<Q> for BTreeSet<T> {
    fn get(&self, item: &Q) -> Option<&T> {
        self.get(item)
    }

    fn take(&mut self, item: &Q) -> Option<T> {
        self.take(item)
    }
}

impl<T: Ord> Own for BinaryHeap<T> {}

impl<T: Ord> Collection for BinaryHeap<T> {
    type Item = T;

    fn len(&self) -> usize {
        self.len()
    }

    fn capacity(&self) -> usize {
        self.capacity()
    }

    fn append(&mut self, other: &mut Self) {
        self.extend(other.drain());
    }

    fn extend_object(&mut self, items: &mut Iterator<Item = T>) {
        self.extend(items);
    }

    fn clear(&mut self) {
        self.clear();
    }

    fn drain<'a>(&'a mut self) -> Box<Iterator<Item = T> + 'a> {
        Box::new(self.drain())
    }

    fn reserve(&mut self, additional: usize) {
        self.reserve(additional);
    }

    fn shrink_to_fit(&mut self) {
        self.shrink_to_fit();
    }
}

impl<T: Ord> Iter for BinaryHeap<T> {
    fn iter<'a>(&'a self) -> Box<Iterator<Item = &'a T> + 'a> {
        Box::new(self.iter())
    }

    fn iter_mut<'a>(&'a mut self) -> Box<Iterator<Item = &'a mut T> + 'a> where Self: Mut {
        unimplemented!()
    }
}

impl<T: Ord> Queue for BinaryHeap<T> {
    fn push(&mut self, item: T) {
        self.push(item);
    }

    fn front(&self) -> Option<&T> {
        self.peek()
    }

    fn pop_front(&mut self) -> Option<T> {
        self.pop()
    }
}

impl<K: Eq + Hash, V> Mut for HashMap<K, V> {}

impl<K: Eq + Hash, V> Own for HashMap<K, V> {}

impl<K: Eq + Hash, V> Collection for HashMap<K, V> {
    type Item = (K, V);

    fn len(&self) -> usize {
        self.len()
    }

    fn capacity(&self) -> usize {
        self.capacity()
    }

    fn append(&mut self, other: &mut Self) {
        self.extend(other.drain());
    }

    fn extend_object(&mut self, items: &mut Iterator<Item = (K, V)>) {
        self.extend(items);
    }

    fn clear(&mut self) {
        self.clear();
    }

    fn drain<'a>(&'a mut self) -> Box<Iterator<Item = (K, V)> + 'a> {
        Box::new(self.drain())
    }

    fn reserve(&mut self, additional: usize) {
        self.reserve(additional);
    }

    fn shrink_to_fit(&mut self) {
        self.shrink_to_fit();
    }
}

impl<K: Eq + Hash, V> map::Base for HashMap<K, V> {
    type Key = K;
    type Value = V;

    fn iter<'a>(&'a self) -> Box<Iterator<Item = (&'a K, &'a V)> + 'a> {
        Box::new(self.iter())
    }

    fn iter_mut<'a>(&'a mut self) -> Box<Iterator<Item = (&'a K, &'a mut V)> + 'a> {
        Box::new(self.iter_mut())
    }

    fn insert(&mut self, key: K, value: V) -> Option<V> {
        self.insert(key, value)
    }

    fn entry<'a>(&'a mut self, key: K) -> map::Entry<'a, K, V> {
        match self.entry(key) {
            hash_map::Entry::Occupied(e) => map::Entry::Occupied(Box::new(e)),
            hash_map::Entry::Vacant(e) => map::Entry::Vacant(Box::new(e)),
        }
    }
}

impl<'a, K: 'a + Eq + Hash, V: 'a> map::OccupiedEntry for hash_map::OccupiedEntry<'a, K, V> {
    type Key = K;
    type Value = V;
    type MutValue = &'a mut V;

    fn get(&self) -> &V {
        self.get()
    }

    fn get_mut(&mut self) -> &mut V {
        self.get_mut()
    }

    fn into_mut(self: Box<Self>) -> &'a mut V {
        (*self).into_mut()
    }

    fn remove(self: Box<Self>) -> V {
        (*self).remove()
    }
}

impl<'a, K: 'a + Eq + Hash, V: 'a> map::VacantEntry for hash_map::VacantEntry<'a, K, V> {
    type Key = K;
    type Value = V;
    type MutValue = &'a mut V;

    fn insert(self: Box<Self>, value: V) -> &'a mut V {
        (*self).insert(value)
    }
}


impl<T: Eq + Hash> Own for HashSet<T> {}

impl<T: Eq + Hash> Collection for HashSet<T> {
    type Item = T;

    fn len(&self) -> usize {
        self.len()
    }

    fn capacity(&self) -> usize {
        self.capacity()
    }

    fn append(&mut self, other: &mut Self) {
        self.extend(other.drain());
    }

    fn extend_object(&mut self, items: &mut Iterator<Item = T>) {
        self.extend(items);
    }

    fn clear(&mut self) {
        self.clear();
    }

    fn drain<'a>(&'a mut self) -> Box<Iterator<Item = T> + 'a> {
        Box::new(self.drain())
    }

    fn reserve(&mut self, _additional: usize) {}

    fn shrink_to_fit(&mut self) {}
}

impl<T: Eq + Hash> Iter for HashSet<T> {
    fn iter<'a>(&'a self) -> Box<Iterator<Item = &'a T> + 'a> {
        Box::new(self.iter())
    }

    fn iter_mut<'a>(&'a mut self) -> Box<Iterator<Item = &'a mut T> + 'a> where Self: Mut {
        unimplemented!()
    }
}

impl<T: Eq + Hash> set::Base for HashSet<T> {
    fn is_disjoint(&self, other: &Self) -> bool {
        self.is_disjoint(other)
    }

    fn is_subset(&self, other: &Self) -> bool {
        self.is_subset(other)
    }

    fn insert(&mut self, item: T) -> bool {
        self.insert(item)
    }

    fn replace(&mut self, item: T) -> Option<T> {
        self.replace(item)
    }
}

impl<T: Eq + Hash + Borrow<Q>, Q: ?Sized + Eq + Hash> Set<Q> for HashSet<T> {
    fn get(&self, item: &Q) -> Option<&T> {
        self.get(item)
    }

    fn take(&mut self, item: &Q) -> Option<T> {
        self.take(item)
    }
}

impl<T> Mut for LinkedList<T> {}

impl<T> Own for LinkedList<T> {}

impl<T> Collection for LinkedList<T> {
    type Item = T;

    fn len(&self) -> usize {
        self.len()
    }

    fn capacity(&self) -> usize {
        self.len()
    }

    fn append(&mut self, other: &mut Self) {
        self.append(other);
    }

    fn extend_object(&mut self, items: &mut Iterator<Item = T>) {
        self.extend(items);
    }

    fn clear(&mut self) {
        self.clear();
    }

    fn drain<'a>(&'a mut self) -> Box<Iterator<Item = T> + 'a> {
        Box::new(mem::replace(self, Self::new()).into_iter())
    }

    fn reserve(&mut self, _additional: usize) {}

    fn shrink_to_fit(&mut self) {}
}

impl<T> Iter for LinkedList<T> {
    fn iter<'a>(&'a self) -> Box<Iterator<Item = &'a T> + 'a> {
        Box::new(self.iter())
    }

    fn iter_mut<'a>(&'a mut self) -> Box<Iterator<Item = &'a mut T> + 'a> {
        Box::new(self.iter_mut())
    }
}

impl<T> Queue for LinkedList<T> {
    fn push(&mut self, item: T) {
        self.push_back(item);
    }

    fn front(&self) -> Option<&T> {
        self.front()
    }

    fn pop_front(&mut self) -> Option<T> {
        self.pop_front()
    }
}

impl<T> Deque for LinkedList<T> {
    fn back(&self) -> Option<&T> {
        self.back()
    }

    fn pop_back(&mut self) -> Option<T> {
        self.pop_back()
    }
}

impl<T> FifoQueue for LinkedList<T> {
    fn front_mut(&mut self) -> Option<&mut T> {
        self.front_mut()
    }
}

impl<T> FifoDeque for LinkedList<T> {
    fn push_front(&mut self, item: T) {
        self.push_front(item);
    }

    fn back_mut(&mut self) -> Option<&mut T> {
        self.back_mut()
    }
}

impl<T> Mut for Vec<T> {}

impl<T> Own for Vec<T> {}

impl<T> Collection for Vec<T> {
    type Item = T;

    fn len(&self) -> usize {
        self.len()
    }

    fn capacity(&self) -> usize {
        self.capacity()
    }

    fn append(&mut self, other: &mut Self) {
        self.append(other);
    }

    fn extend_object(&mut self, items: &mut Iterator<Item = T>) {
        self.extend(items);
    }

    fn clear(&mut self) {
        self.clear();
    }

    fn drain<'a>(&'a mut self) -> Box<Iterator<Item = T> + 'a> {
        Box::new(self.drain(..))
    }

    fn reserve(&mut self, additional: usize) {
        self.reserve(additional);
    }

    fn shrink_to_fit(&mut self) {
        self.shrink_to_fit();
    }
}

impl<T> Iter for Vec<T> {
    fn iter<'a>(&'a self) -> Box<Iterator<Item = &'a T> + 'a> {
        Box::new((**self).iter())
    }

    fn iter_mut<'a>(&'a mut self) -> Box<Iterator<Item = &'a mut T> + 'a> {
        Box::new((**self).iter_mut())
    }
}

impl<T> DrainRange<Range<usize>> for Vec<T> {
    fn drain_range<'a>(&'a mut self, range: Range<usize>) -> Box<Iterator<Item = T> + 'a> {
        Box::new(self.drain(range))
    }
}

impl<T> List for Vec<T> {
    fn get(&self, index: usize) -> Option<&T> {
        (**self).get(index)
    }

    fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        (**self).get_mut(index)
    }

    fn swap(&mut self, i: usize, j: usize) {
        (**self).swap(i, j);
    }

    fn push(&mut self, item: T) {
        self.push(item);
    }

    fn insert(&mut self, index: usize, item: T) {
        self.insert(index, item);
    }

    fn pop(&mut self) -> Option<T> {
        self.pop()
    }

    fn remove(&mut self, index: usize) -> Option<T> {
        if index < self.len() {
            Some(self.remove(index))
        } else {
            None
        }
    }

    fn swap_remove(&mut self, index: usize) -> Option<T> {
        if index < self.len() {
            Some(self.swap_remove(index))
        } else {
            None
        }
    }

    fn truncate(&mut self, len: usize) {
        self.truncate(len);
    }

    fn split_off(&mut self, index: usize) -> Self {
        self.split_off(index)
    }
}

impl<T> Mut for VecDeque<T> {}

impl<T> Own for VecDeque<T> {}

impl<T> Collection for VecDeque<T> {
    type Item = T;

    fn len(&self) -> usize {
        self.len()
    }

    fn capacity(&self) -> usize {
        self.capacity()
    }

    fn clear(&mut self) {
        self.clear();
    }

    fn append(&mut self, other: &mut Self) {
        self.append(other);
    }

    fn extend_object(&mut self, items: &mut Iterator<Item = T>) {
        self.extend(items);
    }

    fn drain<'a>(&'a mut self) -> Box<Iterator<Item = T> + 'a> {
        Box::new(self.drain(..))
    }

    fn reserve(&mut self, additional: usize) {
        self.reserve(additional);
    }

    fn shrink_to_fit(&mut self) {
        self.shrink_to_fit();
    }
}

impl<T> Iter for VecDeque<T> {
    fn iter<'a>(&'a self) -> Box<Iterator<Item = &'a T> + 'a> {
        Box::new(self.iter())
    }

    fn iter_mut<'a>(&'a mut self) -> Box<Iterator<Item = &'a mut T> + 'a> {
        Box::new(self.iter_mut())
    }
}

impl<T> DrainRange<Range<usize>> for VecDeque<T> {
    fn drain_range<'a>(&'a mut self, range: Range<usize>) -> Box<Iterator<Item = T> + 'a> {
        Box::new(self.drain(range))
    }
}

impl<T> List for VecDeque<T> {
    fn get(&self, index: usize) -> Option<&T> {
        self.get(index)
    }

    fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        self.get_mut(index)
    }

    fn swap(&mut self, i: usize, j: usize) {
        self.swap(i, j);
    }

    fn push(&mut self, item: T) {
        self.push_back(item);
    }

    fn insert(&mut self, index: usize, item: T) {
        self.insert(index, item);
    }

    fn pop(&mut self) -> Option<T> {
        self.pop_back()
    }

    fn remove(&mut self, index: usize) -> Option<T> {
        self.remove(index)
    }

    fn swap_remove(&mut self, index: usize) -> Option<T> {
        self.swap_remove_back(index)
    }

    fn truncate(&mut self, len: usize) {
        self.truncate(len);
    }

    fn split_off(&mut self, index: usize) -> Self {
        self.split_off(index)
    }
}

impl<T> Queue for VecDeque<T> {
    fn push(&mut self, item: T) {
        self.push_back(item);
    }

    fn front(&self) -> Option<&T> {
        self.front()
    }

    fn pop_front(&mut self) -> Option<T> {
        self.pop_front()
    }
}

impl<T> Deque for VecDeque<T> {
    fn back(&self) -> Option<&T> {
        self.back()
    }

    fn pop_back(&mut self) -> Option<T> {
        self.pop_back()
    }
}

impl<T> FifoQueue for VecDeque<T> {
    fn front_mut(&mut self) -> Option<&mut T> {
        self.front_mut()
    }
}

impl<T> FifoDeque for VecDeque<T> {
    fn push_front(&mut self, item: T) {
        self.push_front(item);
    }

    fn back_mut(&mut self) -> Option<&mut T> {
        self.back_mut()
    }
}
