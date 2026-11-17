#![cfg(feature = "heapless")]

use std::fmt::Debug;
use std::panic::{UnwindSafe, catch_unwind};

use collections::heapless::binary_heap::{Max, Min};
use collections::heapless::index_map::FnvIndexMap;
use collections::heapless::index_set::FnvIndexSet;
use collections::heapless::{BinaryHeap, Deque, LinearMap, String as FixedString, Vec as FixedVec};
use collections::{
    Collection, CollectionBase, CollectionBytes, CollectionCapacity, CollectionEntry,
    CollectionMap, MapEntry,
};

/// Checks an empty collection which has room for exactly `items.len()` items.
fn check_fixed<C: CollectionCapacity>(mut collection: C, items: &[C::Item], extra: C::Item)
where
    C::Item: Clone + Debug + PartialEq,
{
    let capacity = collection.capacity();
    assert_eq!(capacity, items.len());
    assert!(collection.is_empty());
    assert_eq!(collection.item_count(), 0);

    for (no, item) in items.iter().enumerate() {
        assert_eq!(collection.try_push(item.clone()), Ok(()));
        assert_eq!(collection.item_count(), no + 1);
    }
    assert!(!collection.is_empty());
    assert_eq!(collection.capacity(), capacity);

    assert!(collection.try_reserve(0).is_ok());
    assert!(collection.try_reserve(1).is_err());
    assert!(collection.try_reserve(usize::MAX).is_err());
    assert_eq!(collection.try_push(extra.clone()), Err(extra));
    assert_eq!(collection.item_count(), items.len());

    collection.clear();
    assert!(collection.is_empty());
    assert_eq!(collection.capacity(), capacity);

    collection.reserve(capacity);
    assert!(collection.try_reserve(capacity).is_ok());
    assert!(collection.try_reserve(capacity + 1).is_err());
    collection.push(items[0].clone());
    assert_eq!(collection.item_count(), 1);
    assert!(collection.try_reserve(capacity).is_err());
}

#[test]
fn fixed_capacity_collections_follow_the_base_and_capacity_contracts() {
    check_fixed(FixedVec::<u8, 4>::new(), &[1, 2, 3, 4], 5);
    check_fixed(FixedVec::<u8, 4, u8>::new(), &[1, 2, 3, 4], 5);
    check_fixed(FixedString::<4>::new(), &['a', 'b', 'c', 'd'], 'e');
    check_fixed(FixedString::<4, u8>::new(), &['a', 'b', 'c', 'd'], 'e');
    check_fixed(Deque::<u8, 4>::new(), &[1, 2, 3, 4], 5);
    check_fixed(BinaryHeap::<u8, Max, 4>::new(), &[1, 2, 3, 4], 5);
    check_fixed(BinaryHeap::<u8, Min, 4>::new(), &[1, 2, 3, 4], 5);
    check_fixed(FnvIndexSet::<u8, 4>::new(), &[1, 2, 3, 4], 5);
    check_fixed(
        FnvIndexMap::<u8, u8, 4>::new(),
        &[(1, 10), (2, 20), (3, 30), (4, 40)],
        (5, 50),
    );
    check_fixed(
        LinearMap::<u8, u8, 4>::new(),
        &[(1, 10), (2, 20), (3, 30), (4, 40)],
        (5, 50),
    );
}

fn check_collection<C: Collection + CollectionCapacity>(items: &[C::Item], capacity: usize)
where
    C::Item: Clone,
{
    assert!(!items.is_empty());
    assert!(C::default().is_empty());

    let mut collection = C::with_capacity(usize::MAX);
    assert_eq!(collection.capacity(), capacity);
    assert!(collection.is_empty());
    collection.push(items[0].clone());
    collection.extend(items[1..].iter().cloned());
    assert_eq!(collection.item_count(), items.len());

    let collected: C = items.iter().cloned().collect();
    assert_eq!(collected.item_count(), items.len());

    collection.clear();
    assert!(collection.is_empty());
}

#[test]
fn fixed_capacity_constructors_ignore_the_requested_capacity() {
    check_collection::<FixedVec<u8, 4>>(&[1, 2], 4);
    check_collection::<FixedVec<u8, 4, u8>>(&[1, 2], 4);
    check_collection::<FnvIndexSet<u8, 4>>(&[1, 2], 4);
    check_collection::<FnvIndexMap<u8, u8, 4>>(&[(1, 10), (2, 20)], 4);
}

fn check_map<M: CollectionMap<Key = u8, Value = u32>>(mut map: M) {
    assert!(map.is_empty());

    assert_eq!(map.insert(1, 10), None);
    assert_eq!(map.insert(2, 20), None);
    assert_eq!(map.insert(3, 30), None);
    assert_eq!(map.insert(1, 11), Some(10));
    assert_eq!(map.item_count(), 3);

    assert!(map.contains_key(&1));
    assert!(!map.contains_key(&9));
    assert_eq!(map.get(&1), Some(&11));
    assert_eq!(map.get(&9), None);
    *map.get_mut(&2).unwrap() += 1;
    assert_eq!(map.get(&2), Some(&21));

    let mut pairs: Vec<_> = map.iter().map(|(key, value)| (*key, *value)).collect();
    pairs.sort_unstable();
    assert_eq!(pairs, [(1, 11), (2, 21), (3, 30)]);

    let mut keys: Vec<_> = map.keys().copied().collect();
    keys.sort_unstable();
    assert_eq!(keys, [1, 2, 3]);

    let mut values: Vec<_> = map.values().copied().collect();
    values.sort_unstable();
    assert_eq!(values, [11, 21, 30]);

    for (key, value) in map.iter_mut() {
        *value += u32::from(*key);
    }
    for value in map.values_mut() {
        *value *= 2;
    }
    assert_eq!(map.get(&3), Some(&66));

    map.retain(|key, _| *key != 2);
    assert_eq!(map.item_count(), 2);
    assert_eq!(map.remove(&1), Some(24));
    assert_eq!(map.remove(&1), None);
    assert!(!map.contains_key(&1));
}

#[test]
fn fixed_capacity_maps_follow_the_keyed_contract() {
    check_map(FnvIndexMap::<u8, u32, 4>::new());
    check_map(LinearMap::<u8, u32, 4>::new());
}

fn check_entry_api<M: CollectionEntry<Key = &'static str, Value = usize>>(mut map: M) {
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
    assert_eq!(map.item_count(), 4);

    // Occupied entries of a full map stay usable.
    *map.entry("map").or_insert(0) += 1;
    *map.entry("entry").or_insert_with(|| 0) += 1;
    *map.entry("created").or_default() += 1;
    assert_eq!(map.get(&"map"), Some(&4));
    assert_eq!(map.get(&"entry"), Some(&2));
    assert_eq!(map.get(&"created"), Some(&6));
}

#[test]
fn fixed_capacity_entry_operations_work_generically() {
    check_entry_api(FnvIndexMap::<&'static str, usize, 4>::new());
}

fn check_bytes<C: CollectionBytes>(mut collection: C, items: &[C::Item], total: usize)
where
    C::Item: Clone,
{
    assert_eq!(collection.byte_len(), 0);

    let mut expected = 0;
    for item in items {
        expected += C::item_byte_len(item);
        collection.push(item.clone());
        assert_eq!(collection.byte_len(), expected);
    }
    assert_eq!(collection.byte_len(), total);
    assert_eq!(collection.item_count(), items.len());

    collection.clear();
    assert_eq!(collection.byte_len(), 0);
}

#[test]
fn fixed_capacity_byte_collections_report_bytes() {
    check_bytes(FixedString::<10>::new(), &['a', 'é', '€', '🦀'], 10);
    check_bytes(FixedString::<10, u8>::new(), &['a', 'é', '€', '🦀'], 10);
    check_bytes(FixedVec::<u8, 2>::new(), &[1, 2], 2);
    check_bytes(FixedVec::<u8, 2, u8>::new(), &[1, 2], 2);
}

fn push_with_item_limit<C: CollectionBase>(
    collection: &mut C,
    item: C::Item,
    maximum: usize,
) -> Result<(), C::Item> {
    if collection.item_count() >= maximum {
        return Err(item);
    }
    collection.push(item);
    Ok(())
}

fn push_with_byte_limit<C: CollectionBytes>(
    collection: &mut C,
    item: C::Item,
    maximum: usize,
) -> Result<(), C::Item> {
    let Some(new_len) = collection.byte_len().checked_add(C::item_byte_len(&item)) else {
        return Err(item);
    };
    if new_len > maximum {
        return Err(item);
    }
    collection.push(item);
    Ok(())
}

#[test]
fn fixed_string_counts_chars_and_bytes_separately() {
    let mut string = FixedString::<8>::new();
    assert!(CollectionBase::is_empty(&string));
    assert_eq!(string.item_count(), 0);

    assert_eq!(push_with_byte_limit(&mut string, 'a', 2), Ok(()));
    assert_eq!(push_with_byte_limit(&mut string, 'é', 2), Err('é'));
    assert_eq!(push_with_item_limit(&mut string, 'é', 2), Ok(()));
    assert_eq!(push_with_item_limit(&mut string, 'x', 2), Err('x'));
    assert_eq!(string.as_str(), "aé");
    assert_eq!(
        (string.len(), string.item_count(), string.byte_len()),
        (3, 2, 3)
    );

    let mut string = FixedString::<4>::new();
    CollectionBase::push(&mut string, '🦀');
    assert!(!CollectionBase::is_empty(&string));
    assert_eq!((string.item_count(), string.byte_len()), (1, 4));
    assert_eq!(CollectionCapacity::capacity(&string), 4);
}

#[test]
fn try_push_returns_the_item_when_there_is_no_room() {
    let mut vec = FixedVec::<u8, 2>::new();
    assert_eq!(vec.try_push(1), Ok(()));
    assert_eq!(vec.try_push(2), Ok(()));
    assert_eq!(vec.try_push(3), Err(3));
    assert_eq!(vec.as_slice(), [1, 2]);

    let mut string = FixedString::<3>::new();
    assert_eq!(string.try_push('a'), Ok(()));
    assert_eq!(string.try_push('é'), Ok(()));
    assert_eq!(string.try_push('é'), Err('é'));
    assert_eq!(string.as_str(), "aé");

    let mut string = FixedString::<3>::new();
    assert_eq!(string.try_push('a'), Ok(()));
    assert_eq!(string.try_push('a'), Ok(()));
    assert!(CollectionCapacity::try_reserve(&mut string, 1).is_ok());
    assert_eq!(string.try_push('é'), Err('é'));
    assert_eq!(string.try_push('🦀'), Err('🦀'));
    assert_eq!(string.as_str(), "aa");

    let mut deque = Deque::<u8, 1>::new();
    assert_eq!(deque.try_push(1), Ok(()));
    assert_eq!(deque.try_push(2), Err(2));
    assert_eq!(deque.front(), Some(&1));

    let mut heap = BinaryHeap::<u8, Max, 1>::new();
    assert_eq!(heap.try_push(1), Ok(()));
    assert_eq!(heap.try_push(2), Err(2));
    assert_eq!(heap.peek(), Some(&1));
}

#[test]
fn try_push_of_a_present_value_or_key_needs_no_room() {
    let mut set = FnvIndexSet::<u8, 2>::new();
    assert_eq!(set.try_push(1), Ok(()));
    assert_eq!(set.try_push(2), Ok(()));
    assert_eq!(set.try_push(1), Ok(()));
    assert_eq!(set.try_push(3), Err(3));
    assert_eq!(set.item_count(), 2);

    let mut map = FnvIndexMap::<u8, u8, 2>::new();
    assert_eq!(map.try_push((1, 10)), Ok(()));
    assert_eq!(map.try_push((2, 20)), Ok(()));
    assert_eq!(map.try_push((1, 11)), Ok(()));
    assert_eq!(map.try_push((3, 30)), Err((3, 30)));
    assert_eq!(CollectionMap::get(&map, &1), Some(&11));
    assert_eq!(CollectionMap::insert(&mut map, 2, 21), Some(20));
    assert_eq!(map.item_count(), 2);

    let mut map = LinearMap::<u8, u8, 1>::new();
    assert_eq!(map.try_push((1, 10)), Ok(()));
    assert_eq!(map.try_push((1, 11)), Ok(()));
    assert_eq!(map.try_push((2, 20)), Err((2, 20)));
    assert_eq!(CollectionMap::get(&map, &1), Some(&11));
    assert_eq!(CollectionMap::insert(&mut map, 1, 12), Some(11));
    assert_eq!(map.item_count(), 1);
}

fn assert_full_panic(operation: impl FnOnce() + UnwindSafe) {
    let payload = catch_unwind(operation).expect_err("the operation must panic");
    let message = payload.downcast_ref::<&str>().copied().unwrap_or_default();
    assert_eq!(message, "fixed-capacity collection is full");
}

fn push_twice<C: CollectionBase>(mut collection: C, first: C::Item, second: C::Item) {
    collection.push(first);
    collection.push(second);
}

#[test]
fn infallible_insertion_into_a_full_collection_panics() {
    assert_full_panic(|| push_twice(FixedVec::<u8, 1>::new(), 1, 2));
    assert_full_panic(|| push_twice(FixedString::<1>::new(), 'a', 'b'));
    assert_full_panic(|| push_twice(FixedString::<1>::new(), 'a', 'é'));
    assert_full_panic(|| push_twice(Deque::<u8, 1>::new(), 1, 2));
    assert_full_panic(|| push_twice(BinaryHeap::<u8, Max, 1>::new(), 1, 2));
    assert_full_panic(|| {
        let mut set = FnvIndexSet::<u8, 2>::new();
        set.extend([1, 2]);
        CollectionBase::push(&mut set, 3);
    });
    assert_full_panic(|| {
        let mut map = FnvIndexMap::<u8, u8, 2>::new();
        map.extend([(1, 10), (2, 20)]);
        CollectionBase::push(&mut map, (3, 30));
    });
    assert_full_panic(|| push_twice(LinearMap::<u8, u8, 1>::new(), (1, 10), (2, 20)));

    assert_full_panic(|| {
        let mut map = FnvIndexMap::<u8, u8, 2>::new();
        map.extend([(1, 10), (2, 20)]);
        CollectionMap::insert(&mut map, 3, 30);
    });
    assert_full_panic(|| {
        let mut map = LinearMap::<u8, u8, 1>::new();
        CollectionMap::insert(&mut map, 1, 10);
        CollectionMap::insert(&mut map, 2, 20);
    });
}

#[test]
fn vacant_entries_of_a_full_map_panic_on_insertion() {
    fn full() -> FnvIndexMap<u8, u8, 2> {
        let mut map = FnvIndexMap::new();
        map.extend([(1, 10), (2, 20)]);
        map
    }

    assert_full_panic(|| {
        MapEntry::or_insert(CollectionEntry::entry(&mut full(), 3), 30);
    });
    assert_full_panic(|| {
        MapEntry::or_insert_with(CollectionEntry::entry(&mut full(), 3), || 30);
    });
    assert_full_panic(|| {
        MapEntry::or_default(CollectionEntry::entry(&mut full(), 3));
    });
}

#[test]
fn reserving_more_than_the_remaining_room_panics() {
    assert_full_panic(|| CollectionCapacity::reserve(&mut FixedVec::<u8, 1>::new(), 2));
    assert_full_panic(|| CollectionCapacity::reserve(&mut FixedString::<1>::new(), 2));
    assert_full_panic(|| CollectionCapacity::reserve(&mut Deque::<u8, 1>::new(), 2));
    assert_full_panic(|| CollectionCapacity::reserve(&mut BinaryHeap::<u8, Max, 1>::new(), 2));
    assert_full_panic(|| CollectionCapacity::reserve(&mut FnvIndexSet::<u8, 2>::new(), 3));
    assert_full_panic(|| CollectionCapacity::reserve(&mut FnvIndexMap::<u8, u8, 2>::new(), 3));
    assert_full_panic(|| CollectionCapacity::reserve(&mut LinearMap::<u8, u8, 1>::new(), 2));
    assert_full_panic(|| {
        CollectionCapacity::reserve(&mut FixedVec::<u8, 1>::new(), usize::MAX);
    });
}

#[test]
fn one_generic_function_fills_heap_and_fixed_capacity_collections() {
    fn fill<C: CollectionCapacity<Item = u8>>(collection: &mut C, limit: u8) -> u8 {
        let mut stored = 0;
        for item in 0..limit {
            if collection.try_push(item).is_err() {
                break;
            }
            stored += 1;
        }
        stored
    }

    assert_eq!(fill(&mut FixedVec::<u8, 4>::new(), 10), 4);
    assert_eq!(fill(&mut Deque::<u8, 4>::new(), 10), 4);
    assert_eq!(fill(&mut BinaryHeap::<u8, Min, 4>::new(), 10), 4);
    assert_eq!(fill(&mut FnvIndexSet::<u8, 4>::new(), 10), 4);

    #[cfg(feature = "alloc")]
    {
        assert_eq!(fill(&mut Vec::new(), 10), 10);
        assert_eq!(fill(&mut std::collections::VecDeque::new(), 10), 10);
    }
}
