extern crate compare;
extern crate collect;

use self::compare::Compare;
use self::collect::{TreeMap, TreeSet};
use self::collect::enum_set::{CLike, EnumSet};
use self::collect::proto::linear_map::LinearMap;
use super::{Collection, Map, MapLookup, Set, SetLookup};

impl<K, V> Collection for LinearMap<K, V> where K: Eq { collection_methods!{} }
impl<K, V> Map for LinearMap<K, V> where K: Eq { type Key = K; type Value = V; map_methods!{K, V} }
impl<K, V> MapLookup<K> for LinearMap<K, V> where K: Eq { type MapValue = V; map_lookup_methods!{K, V} }

impl<K, V, C> Collection for TreeMap<K, V, C> where C: Compare<K> { collection_methods!{} }
impl<K, V, C> Map for TreeMap<K, V, C> where C: Compare<K> { type Key = K; type Value = V; map_methods!{K, V} }
impl<K, V, C, Q: ?Sized> MapLookup<Q> for TreeMap<K, V, C>
    where C: Compare<K> + Compare<Q, K> { type MapValue = V; map_lookup_methods!{Q, V} }

impl<T> Collection for EnumSet<T> where T: CLike { collection_methods!{} }
impl<T> Set for EnumSet<T> where T: CLike { type Item = T; set_methods!{T} }
impl<T> SetLookup<T> for EnumSet<T> where T: CLike { set_lookup_methods!{T} }

impl<T, C> Collection for TreeSet<T, C> where C: Compare<T> { collection_methods!{} }
impl<T, C> Set for TreeSet<T, C> where C: Compare<T> { type Item = T; set_methods!{T} }
impl<T, C, Q: ?Sized> SetLookup<Q> for TreeSet<T, C>
    where C: Compare<T> + Compare<Q, T> { set_lookup_methods!{Q} }
