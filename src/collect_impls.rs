extern crate collect;

use self::collect::compare::Compare;
use self::collect::{TreeMap, TreeSet, TrieMap, TrieSet};
use self::collect::enum_set::{CLike, EnumSet};
use self::collect::proto::linear_map::LinearMap;
use super::{Collection, MutCollection};
use super::{Map, MapLookup, MutMap, MutMapLookup};
use super::{Set, SetLookup, MutSet, MutSetLookup};

impl<K, V> Collection for LinearMap<K, V> where K: Eq { collection_methods!{} }
impl<K, V> MutCollection for LinearMap<K, V> where K: Eq { mut_collection_methods!{} }
impl<K, V> Map for LinearMap<K, V> where K: Eq { type Key = K; type Value = V; }
impl<K, V> MapLookup<K> for LinearMap<K, V> where K: Eq { map_lookup_methods!{K, V} }
impl<K, V> MutMap for LinearMap<K, V> where K: Eq { mut_map_methods!{K, V} }
impl<K, V> MutMapLookup<K> for LinearMap<K, V> where K: Eq { mut_map_lookup_methods!{K, V} }

impl<K, V, C> Collection for TreeMap<K, V, C> where C: Compare<K> { collection_methods!{} }
impl<K, V, C> MutCollection for TreeMap<K, V, C> where C: Compare<K> { mut_collection_methods!{} }
impl<K, V, C> Map for TreeMap<K, V, C> where C: Compare<K> { type Key = K; type Value = V; }
impl<K, V, C, Q: ?Sized> MapLookup<Q> for TreeMap<K, V, C>
    where C: Compare<K> + Compare<Q, K> { map_lookup_methods!{Q, V} }
impl<K, V, C> MutMap for TreeMap<K, V, C> where C: Compare<K> { mut_map_methods!{K, V} }
impl<K, V, C, Q: ?Sized> MutMapLookup<Q> for TreeMap<K, V, C>
    where C: Compare<K> + Compare<Q, K> { mut_map_lookup_methods!{Q, V} }

impl<V> Collection for TrieMap<V> { collection_methods!{} }
impl<V> MutCollection for TrieMap<V> { mut_collection_methods!{} }
impl<V> Map for TrieMap<V> { type Key = usize; type Value = V; }
impl<V> MapLookup<usize> for TrieMap<V> { map_lookup_methods!{usize, V} }
impl<V> MutMap for TrieMap<V> { mut_map_methods!{usize, V} }
impl<V> MutMapLookup<usize> for TrieMap<V> { mut_map_lookup_methods!{usize, V} }

impl<T> Collection for EnumSet<T> where T: CLike { collection_methods!{} }
impl<T> MutCollection for EnumSet<T> where T: CLike { mut_collection_methods!{} }
impl<T> Set for EnumSet<T> where T: CLike { type Item = T; }
impl<T> SetLookup<T> for EnumSet<T> where T: CLike { set_lookup_methods!{T} }
impl<T> MutSet for EnumSet<T> where T: CLike { mut_set_methods!{T} }
impl<T> MutSetLookup<T> for EnumSet<T> where T: CLike { mut_set_lookup_methods!{T} }

impl<T, C> Collection for TreeSet<T, C> where C: Compare<T> { collection_methods!{} }
impl<T, C> MutCollection for TreeSet<T, C> where C: Compare<T> { mut_collection_methods!{} }
impl<T, C> Set for TreeSet<T, C> where C: Compare<T> { type Item = T; }
impl<T, C, Q: ?Sized> SetLookup<Q> for TreeSet<T, C>
    where C: Compare<T> + Compare<Q, T> { set_lookup_methods!{Q} }
impl<T, C> MutSet for TreeSet<T, C> where C: Compare<T> { mut_set_methods!{T} }
impl<T, C, Q: ?Sized> MutSetLookup<Q> for TreeSet<T, C>
    where C: Compare<T> + Compare<Q, T> { mut_set_lookup_methods!{Q} }

impl Collection for TrieSet { collection_methods!{} }
impl MutCollection for TrieSet { mut_collection_methods!{} }
impl Set for TrieSet { type Item = usize; }
impl SetLookup<usize> for TrieSet { set_lookup_methods!{usize} }
impl MutSet for TrieSet { mut_set_methods!{usize} }
impl MutSetLookup<usize> for TrieSet { mut_set_lookup_methods!{usize} }
