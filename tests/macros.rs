use std::collections::{BTreeMap, BTreeSet, LinkedList};

use collections::{bmap, bset, list, s};

#[test]
fn string_macro_creates_a_string() {
    assert_eq!(s!("text"), "text");
}

#[test]
fn alloc_macros_support_empty_plain_and_owned_forms() {
    let empty_map: BTreeMap<u8, u8> = bmap! {};
    assert!(empty_map.is_empty());

    let map = bmap! { 1 => "one", 2 => "two" };
    assert_eq!(map[&1], "one");

    let owned_map: BTreeMap<String, String> = bmap! { owned: "key" => "value" };
    assert_eq!(owned_map["key"], "value");

    let empty_set: BTreeSet<u8> = bset![];
    assert!(empty_set.is_empty());

    let set = bset![1, 2, 1];
    assert_eq!(set.len(), 2);

    let owned_set = bset! { owned: "one", "two" };
    assert!(owned_set.contains("one"));

    let empty_list: LinkedList<u8> = list! {};
    assert!(empty_list.is_empty());

    let list = list! { "first" => "second" };
    assert_eq!(list.front(), Some(&"first"));

    let owned_list: LinkedList<String> = list! { owned: "first" => "second" };
    assert_eq!(owned_list.front().unwrap(), "first");
}

#[cfg(feature = "std")]
#[test]
fn std_macros_support_empty_plain_and_owned_forms() {
    use std::collections::{HashMap, HashSet};

    use collections::{map, set};

    let empty_map: HashMap<u8, u8> = map! {};
    assert!(empty_map.is_empty());

    let map = map! { 1 => "one", 2 => "two" };
    assert_eq!(map[&1], "one");

    let owned_map: HashMap<String, String> = map! { owned: "key" => "value" };
    assert_eq!(owned_map["key"], "value");

    let empty_set: HashSet<u8> = set![];
    assert!(empty_set.is_empty());

    let set = set![1, 2, 1];
    assert_eq!(set.len(), 2);

    let owned_set = set! { owned: "one", "two" };
    assert!(owned_set.contains("one"));
}
