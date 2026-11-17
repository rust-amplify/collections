#![cfg(feature = "alloc")]

use std::collections::{BTreeMap, BTreeSet, BinaryHeap, LinkedList, VecDeque};

use collections::{bmap, bset, deque, heap, list, s};

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

    let empty_deque: VecDeque<u8> = deque! {};
    assert!(empty_deque.is_empty());

    let deque = deque! { "first" => "second" };
    assert_eq!(deque.front(), Some(&"first"));

    let owned_deque: VecDeque<String> = deque! { owned: "first" => "second" };
    assert_eq!(owned_deque.front().unwrap(), "first");

    let empty_heap: BinaryHeap<u8> = heap![];
    assert!(empty_heap.is_empty());

    let heap = heap![1, 3, 2];
    assert_eq!(heap.peek(), Some(&3));

    let owned_heap: BinaryHeap<String> = heap! { owned: "first", "second" };
    assert_eq!(owned_heap.peek().unwrap(), "second");
}

#[test]
fn alloc_macros_reserve_the_capacity_and_evaluate_items_once() {
    let mut calls = 0u8;
    let mut next = || {
        calls += 1;
        calls
    };

    let deque = deque! { next() => next() => next() };
    assert_eq!(deque, [1, 2, 3]);
    assert!(deque.capacity() >= 3);

    let heap = heap![next(), next(), next(),];
    assert_eq!(heap.into_sorted_vec(), [4, 5, 6]);
    assert_eq!(calls, 6);

    let owned: BinaryHeap<String> = heap! { owned: "a", "b", "c" };
    assert!(owned.capacity() >= 3);
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

#[cfg(feature = "std")]
#[test]
fn std_macros_reserve_the_capacity_and_evaluate_items_once() {
    use collections::{map, set};

    let mut calls = 0u8;
    let mut next = || {
        calls += 1;
        calls
    };

    let map = map! { next() => next(), next() => next() };
    assert_eq!(map.len(), 2);
    assert_eq!(map[&1], 2);
    assert_eq!(map[&3], 4);
    assert!(map.capacity() >= 2);

    let set = set![next(), next(), next(),];
    assert_eq!(set.len(), 3);
    assert!(set.capacity() >= 3);
    assert_eq!(calls, 7);
}
