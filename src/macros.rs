macro_rules! collection_methods {
    () => {
        fn clear(&mut self) { self.clear(); }
        fn is_empty(&self) -> bool { self.is_empty() }
        fn len(&self) -> usize { self.len() }
    }
}

macro_rules! map_lookup_methods {
    ($q:ty, $v:ty) => {
        fn contains_key(&self, key: &$q) -> bool { self.contains_key(key) }
        fn get(&self, key: &$q) -> Option<&$v> { self.get(key) }
        fn get_mut(&mut self, key: &$q) -> Option<&mut $v> { self.get_mut(key) }
        fn remove(&mut self, key: &$q) -> Option<$v> { self.remove(key) }
    }
}

macro_rules! map_methods {
    ($k:ty, $v:ty) => {
        fn insert(&mut self, key: $k, value: $v) -> Option<$v> { self.insert(key, value) }
    }
}

macro_rules! occupied_entry_methods {
    ($v:ty) => {
        fn get(&self) -> &$v { self.get() }
        fn get_mut(&mut self) -> &mut $v { self.get_mut() }
        fn insert(&mut self, value: $v) -> $v { self.insert(value) }
        fn remove(self) -> $v { self.remove() }
    }
}

macro_rules! set_lookup_methods {
    ($q:ty) => {
        fn contains(&self, item: &$q) -> bool { self.contains(item) }
        fn remove(&mut self, item: &$q) -> bool { self.remove(item) }
    }
}

macro_rules! set_methods {
    ($t:ty) => {
        fn insert(&mut self, item: $t) -> bool { self.insert(item) }
    }
}
