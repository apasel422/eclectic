extern crate linked_hash_map;

use self::linked_hash_map::LinkedHashMap;
use std::borrow::Borrow;
use std::collections::hash_state::HashState;
use std::hash::Hash;

impl<K, V, S> super::Collection for LinkedHashMap<K, V, S> where K: Eq + Hash, S: HashState {
    collection_methods!{}
}

impl<K, V, S> super::BaseMap for LinkedHashMap<K, V, S> where K: Eq + Hash, S: HashState {
    type Key = K;
    type Value = V;
}

impl<K, V, S, Q> super::MapLookup<Q> for LinkedHashMap<K, V, S>
where
    K: Eq + Hash + Borrow<Q>,
    S: HashState,
    Q: Eq + Hash,
{
    map_lookup_methods!{Q}
}

impl<K, V, S> super::Map for LinkedHashMap<K, V, S> where K: Eq + Hash, S: HashState {
    map_methods!{}
}
