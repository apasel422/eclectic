use std::borrow::Borrow;
use std::collections::{BTreeMap, HashMap, VecMap};
use std::collections::{BitSet, BTreeSet, HashSet};
use std::hash::Hash;
use super::{Collection, MutCollection};
use super::{Map, MapLookup, MutMap, MutMapLookup};
use super::{Set, SetLookup, MutSet, MutSetLookup};

impl<K, V> Collection for BTreeMap<K, V> where K: Ord { collection_methods!{} }
impl<K, V> MutCollection for BTreeMap<K, V> where K: Ord { mut_collection_methods!{} }
impl<K, V> Map for BTreeMap<K, V> where K: Ord { type Key = K; type Value = V; }
impl<K, V, Q: ?Sized> MapLookup<Q> for BTreeMap<K, V>
    where K: Ord + Borrow<Q>, Q: Ord { map_lookup_methods!{Q, V} }
impl<K, V> MutMap for BTreeMap<K, V> where K: Ord { mut_map_methods!{K, V} }
impl<K, V, Q: ?Sized> MutMapLookup<Q> for BTreeMap<K, V>
    where K: Ord + Borrow<Q>, Q: Ord { mut_map_lookup_methods!{Q, V} }

impl<K, V> Collection for HashMap<K, V> where K: Eq + Hash { collection_methods!{} }
impl<K, V> MutCollection for HashMap<K, V> where K: Eq + Hash { mut_collection_methods!{} }
impl<K, V> Map for HashMap<K, V> where K: Eq + Hash { type Key = K; type Value = V; }
impl<K, V, Q: ?Sized> MapLookup<Q> for HashMap<K, V>
    where K: Eq + Hash + Borrow<Q>, Q: Eq + Hash { map_lookup_methods!{Q, V} }
impl<K, V> MutMap for HashMap<K, V> where K: Eq + Hash { mut_map_methods!{K, V} }
impl<K, V, Q: ?Sized> MutMapLookup<Q> for HashMap<K, V>
    where K: Eq + Hash + Borrow<Q>, Q: Eq + Hash { mut_map_lookup_methods!{Q, V} }

impl<V> Collection for VecMap<V> { collection_methods!{} }
impl<V> MutCollection for VecMap<V> { mut_collection_methods!{} }
impl<V> Map for VecMap<V> { type Key = usize; type Value = V; }
impl<V> MapLookup<usize> for VecMap<V> { map_lookup_methods!{usize, V} }
impl<V> MutMap for VecMap<V> { mut_map_methods!{usize, V} }
impl<V> MutMapLookup<usize> for VecMap<V> { mut_map_lookup_methods!{usize, V} }

impl Collection for BitSet { collection_methods!{} }
impl MutCollection for BitSet { mut_collection_methods!{} }
impl Set for BitSet { type Item = usize; }
impl SetLookup<usize> for BitSet { set_lookup_methods!{usize} }
impl MutSet for BitSet { mut_set_methods!{usize} }
impl MutSetLookup<usize> for BitSet { mut_set_lookup_methods!{usize} }

impl<T> Collection for BTreeSet<T> where T: Ord { collection_methods!{} }
impl<T> MutCollection for BTreeSet<T> where T: Ord { mut_collection_methods!{} }
impl<T> Set for BTreeSet<T> where T: Ord { type Item = T; }
impl<T, Q: ?Sized> SetLookup<Q> for BTreeSet<T>
    where T: Ord + Borrow<Q>, Q: Ord { set_lookup_methods!{Q} }
impl<T> MutSet for BTreeSet<T> where T: Ord { mut_set_methods!{T} }
impl<T, Q: ?Sized> MutSetLookup<Q> for BTreeSet<T>
    where T: Ord + Borrow<Q>, Q: Ord { mut_set_lookup_methods!{Q} }

impl<T> Collection for HashSet<T> where T: Eq + Hash { collection_methods!{} }
impl<T> MutCollection for HashSet<T> where T: Eq + Hash { mut_collection_methods!{} }
impl<T> Set for HashSet<T> where T: Eq + Hash { type Item = T; }
impl<T, Q: ?Sized> SetLookup<Q> for HashSet<T>
    where T: Eq + Hash + Borrow<Q>, Q: Eq + Hash { set_lookup_methods!{Q} }
impl<T> MutSet for HashSet<T> where T: Eq + Hash { mut_set_methods!{T} }
impl<T, Q: ?Sized> MutSetLookup<Q> for HashSet<T>
    where T: Eq + Hash + Borrow<Q>, Q: Eq + Hash { mut_set_lookup_methods!{Q} }
