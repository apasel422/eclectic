macro_rules! collection_methods {
    () => {
        fn clear(&mut self) { self.clear(); }
        fn is_empty(&self) -> bool { self.is_empty() }
        fn len(&self) -> usize { self.len() }
    }
}

macro_rules! map_lookup_methods {
    ($q:ty) => {
        fn contains_key(&self, key: &$q) -> bool { self.contains_key(key) }
        fn get(&self, key: &$q) -> Option<&Self::Value> { self.get(key) }
        fn get_mut(&mut self, key: &$q) -> Option<&mut Self::Value> { self.get_mut(key) }
        fn remove(&mut self, key: &$q) -> Option<Self::Value> { self.remove(key) }
    }
}

macro_rules! map_methods {
    () => {
        fn insert(&mut self, key: Self::Key, value: Self::Value) -> Option<Self::Value> {
            self.insert(key, value)
        }
    }
}

macro_rules! occupied_entry_methods {
    () => {
        fn get(&self) -> &Self::Value { self.get() }
        fn get_mut(&mut self) -> &mut Self::Value { self.get_mut() }
        fn insert(&mut self, value: Self::Value) -> Self::Value { self.insert(value) }
        fn remove(self) -> Self::Value { self.remove() }
    }
}

macro_rules! set_lookup_methods {
    ($q:ty) => {
        fn contains(&self, item: &$q) -> bool { self.contains(item) }
        fn remove(&mut self, item: &$q) -> bool { self.remove(item) }
    }
}

macro_rules! set_methods {
    () => {
        fn insert(&mut self, item: Self::Item) -> bool { self.insert(item) }
    }
}
