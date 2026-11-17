use std::collections::BTreeMap;
#[cfg(feature = "std")]
use std::collections::HashMap;
#[cfg(any(feature = "std", feature = "indexmap"))]
use std::collections::hash_map::DefaultHasher;
#[cfg(any(feature = "std", feature = "indexmap"))]
use std::hash::BuildHasherDefault;

use collections::{CollectionMap, MapEntry};

fn check_keyed<C: CollectionMap<Key = u8, Value = String>>() {
    let mut map = C::with_capacity(3);
    assert!(map.is_empty());

    assert_eq!(map.insert(1, "one".to_owned()), None);
    assert_eq!(map.insert(2, "two".to_owned()), None);
    assert_eq!(map.insert(3, "three".to_owned()), None);
    assert_eq!(map.insert(1, "updated".to_owned()), Some("one".to_owned()));

    assert!(map.contains_key(&1));
    assert_eq!(map.get(&1).map(String::as_str), Some("updated"));
    assert_eq!(map.get_mut(&1).map(|value| value.as_str()), Some("updated"));

    let pairs: Vec<_> = map.iter().collect();
    assert_eq!(pairs.len(), 3);

    let keys: Vec<_> = map.keys().copied().collect();
    assert_eq!(keys.len(), 3);
    assert!(keys.contains(&1));

    let values: Vec<_> = map.values().map(String::as_str).collect();
    assert_eq!(values.len(), 3);
    assert!(values.contains(&"updated"));

    for (key, value) in map.iter_mut() {
        value.push(char::from(b'0' + *key));
    }
    for value in map.values_mut() {
        value.push('!');
    }

    map.retain(|key, _| *key != 2);
    assert_eq!(map.len(), 2);
    assert_eq!(map.remove(&1), Some("updated1!".to_owned()));
    assert!(!map.contains_key(&1));
    assert_eq!(map.remove(&99), None);
}

fn check_entry_api<M: CollectionMap<Key = &'static str, Value = usize>>() {
    let mut map = M::with_capacity(4);
    for word in ["map", "entry", "map"] {
        *map.entry(word).or_insert(0) += 1;
    }
    assert_eq!(map.get(&"map"), Some(&2));
    assert_eq!(map.get(&"entry"), Some(&1));

    let entry = map.entry("map");
    assert_eq!(*entry.key(), "map");
    entry.and_modify(|count| *count += 1);
    assert_eq!(map.get(&"map"), Some(&3));

    *map.entry("created").or_insert_with(|| 4) += 1;
    *map.entry("defaulted").or_default() += 1;
    assert_eq!(map.get(&"created"), Some(&5));
    assert_eq!(map.get(&"defaulted"), Some(&1));
}

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
struct OrdOnly(u8);

fn check_ord_only_key<M: CollectionMap<Key = OrdOnly, Value = u8>>() {
    let mut map = M::with_capacity(1);
    assert_eq!(map.insert(OrdOnly(1), 10), None);
    assert_eq!(map.get(&OrdOnly(1)), Some(&10));
    *map.entry(OrdOnly(2)).or_insert(20) += 1;
    assert_eq!(map.get(&OrdOnly(2)), Some(&21));
}

#[test]
fn map_families_follow_the_keyed_contract() {
    check_keyed::<BTreeMap<u8, String>>();

    #[cfg(feature = "std")]
    {
        check_keyed::<HashMap<u8, String>>();

        type CustomHashMap<K, V> = HashMap<K, V, BuildHasherDefault<DefaultHasher>>;
        check_keyed::<CustomHashMap<u8, String>>();
    }

    #[cfg(feature = "indexmap")]
    {
        use collections::indexmap::IndexMap;

        type CustomIndexMap<K, V> = IndexMap<K, V, BuildHasherDefault<DefaultHasher>>;
        check_keyed::<CustomIndexMap<u8, String>>();

        #[cfg(feature = "std")]
        check_keyed::<IndexMap<u8, String>>();
    }
}

#[test]
fn native_entry_operations_work_generically() {
    check_entry_api::<BTreeMap<&'static str, usize>>();

    #[cfg(feature = "std")]
    {
        check_entry_api::<HashMap<&'static str, usize>>();

        type CustomHashMap<K, V> = HashMap<K, V, BuildHasherDefault<DefaultHasher>>;
        check_entry_api::<CustomHashMap<&'static str, usize>>();
    }

    #[cfg(feature = "indexmap")]
    {
        use collections::indexmap::IndexMap;

        type CustomIndexMap<K, V> = IndexMap<K, V, BuildHasherDefault<DefaultHasher>>;
        check_entry_api::<CustomIndexMap<&'static str, usize>>();

        #[cfg(feature = "std")]
        check_entry_api::<IndexMap<&'static str, usize>>();
    }
}

#[test]
fn btree_map_accepts_keys_that_are_not_hashable() {
    check_ord_only_key::<BTreeMap<OrdOnly, u8>>();
}

#[cfg(feature = "indexmap")]
#[test]
fn indexmap_removal_preserves_the_order_of_remaining_entries() {
    use collections::indexmap::IndexMap;

    type CustomIndexMap<K, V> = IndexMap<K, V, BuildHasherDefault<DefaultHasher>>;

    let mut map = CustomIndexMap::<u8, u8>::default();
    map.insert(1, 10);
    map.insert(2, 20);
    map.insert(3, 30);

    assert_eq!(CollectionMap::remove(&mut map, &2), Some(20));
    assert_eq!(map.keys().copied().collect::<Vec<_>>(), [1, 3]);
}
