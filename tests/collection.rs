#![cfg(feature = "alloc")]

mod common;

use std::alloc::{Global, System};
#[cfg(any(feature = "std", feature = "indexmap", feature = "hashbrown"))]
use std::collections::hash_map::DefaultHasher;
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, LinkedList, VecDeque};
#[cfg(feature = "std")]
use std::collections::{HashMap, HashSet};
#[cfg(any(feature = "std", feature = "indexmap", feature = "hashbrown"))]
use std::hash::BuildHasherDefault;

use collections::{Collection, CollectionBase, CollectionBytes};
use common::TestAlloc;

fn check_collection<C: Collection>(items: &[C::Item])
where
    C::Item: Clone,
{
    assert!(!items.is_empty());

    let mut collection = C::default();
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
fn collection_implementations_follow_the_common_contract() {
    check_collection::<String>(&['a', 'b']);
    check_collection::<String>(&['a', 'é']);
    check_collection::<Vec<u8>>(&[1, 2]);
    check_collection::<VecDeque<u8>>(&[1, 2]);
    check_collection::<LinkedList<u8>>(&[1, 2]);
    check_collection::<BinaryHeap<u8>>(&[1, 2]);
    check_collection::<BTreeSet<u8>>(&[1, 2]);
    check_collection::<BTreeMap<u8, u8>>(&[(1, 10), (2, 20)]);

    #[cfg(feature = "std")]
    {
        check_collection::<HashSet<u8>>(&[1, 2]);
        check_collection::<HashMap<u8, u8>>(&[(1, 10), (2, 20)]);

        type CustomHashSet<T> = HashSet<T, BuildHasherDefault<DefaultHasher>>;
        type CustomHashMap<K, V> = HashMap<K, V, BuildHasherDefault<DefaultHasher>>;
        check_collection::<CustomHashSet<u8>>(&[1, 2]);
        check_collection::<CustomHashMap<u8, u8>>(&[(1, 10), (2, 20)]);
    }

    #[cfg(feature = "hashbrown")]
    {
        use collections::hashbrown::{HashMap, HashSet};

        check_collection::<HashSet<u8>>(&[1, 2]);
        check_collection::<HashMap<u8, u8>>(&[(1, 10), (2, 20)]);

        type CustomHashSet<T> = HashSet<T, BuildHasherDefault<DefaultHasher>>;
        type CustomHashMap<K, V> = HashMap<K, V, BuildHasherDefault<DefaultHasher>>;
        check_collection::<CustomHashSet<u8>>(&[1, 2]);
        check_collection::<CustomHashMap<u8, u8>>(&[(1, 10), (2, 20)]);
    }

    #[cfg(feature = "ascii")]
    {
        use ascii::{AsciiChar, AsciiString};

        let items = [
            AsciiChar::from_ascii(b'a').unwrap(),
            AsciiChar::from_ascii(b'b').unwrap(),
        ];
        check_collection::<AsciiString>(&items);
    }

    #[cfg(feature = "indexmap")]
    {
        use collections::indexmap::{IndexMap, IndexSet};

        type CustomIndexSet<T> = IndexSet<T, BuildHasherDefault<DefaultHasher>>;
        type CustomIndexMap<K, V> = IndexMap<K, V, BuildHasherDefault<DefaultHasher>>;

        check_collection::<CustomIndexSet<u8>>(&[1, 2]);
        check_collection::<CustomIndexMap<u8, u8>>(&[(1, 10), (2, 20)]);

        #[cfg(feature = "std")]
        {
            check_collection::<IndexSet<u8>>(&[1, 2]);
            check_collection::<IndexMap<u8, u8>>(&[(1, 10), (2, 20)]);
        }
    }
}

#[test]
fn collection_edge_cases_match_the_contract() {
    let mut string = String::with_capacity(1);
    CollectionBase::push(&mut string, 'é');
    assert_eq!(string.len(), 2);

    let mut set = BTreeSet::<u8>::with_capacity(8);
    CollectionBase::push(&mut set, 1);
    CollectionBase::push(&mut set, 1);
    assert_eq!(set.len(), 1);

    let mut map = BTreeMap::<u8, u8>::with_capacity(8);
    CollectionBase::push(&mut map, (1, 10));
    CollectionBase::push(&mut map, (1, 20));
    assert_eq!(map.len(), 1);
    assert_eq!(map.get(&1), Some(&20));

    let mut deque = VecDeque::with_capacity(2);
    CollectionBase::push(&mut deque, 1);
    CollectionBase::push(&mut deque, 2);
    assert_eq!(deque.front(), Some(&1));
    assert_eq!(deque.back(), Some(&2));

    let mut list = LinkedList::<u8>::with_capacity(2);
    CollectionBase::push(&mut list, 1);
    CollectionBase::push(&mut list, 2);
    assert_eq!(list.front(), Some(&1));
    assert_eq!(list.back(), Some(&2));
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
fn byte_collections_report_bytes_and_item_byte_lengths() {
    check_bytes(String::new(), &['a', 'é', '€', '🦀'], 10);
    check_bytes(Vec::<u8>::new(), &[1, 2], 2);
    check_bytes(Vec::<u8, Global>::new_in(Global), &[1, 2], 2);
    check_bytes(Vec::<u8, System>::new_in(System), &[1, 2], 2);
    check_bytes(
        Vec::<u8, TestAlloc>::new_in(TestAlloc::default()),
        &[1, 2],
        2,
    );

    #[cfg(feature = "ascii")]
    {
        use ascii::{AsciiChar, AsciiString};

        let items = [
            AsciiChar::from_ascii(b'a').unwrap(),
            AsciiChar::from_ascii(b'b').unwrap(),
        ];
        check_bytes(AsciiString::new(), &items, 2);
    }
}

#[test]
fn string_emptiness_does_not_depend_on_the_char_width() {
    let mut string = String::new();
    assert!(CollectionBase::is_empty(&string));
    assert_eq!(string.item_count(), 0);

    CollectionBase::push(&mut string, 'é');
    assert!(!CollectionBase::is_empty(&string));
    assert_eq!(string.item_count(), 1);
}

fn push_with_item_limit<C: CollectionBase>(
    collection: &mut C,
    item: C::Item,
    maximum: usize,
) -> Result<(), C::Item> {
    if collection.item_count() >= maximum {
        return Err(item);
    }
    CollectionBase::push(collection, item);
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
    CollectionBase::push(collection, item);
    Ok(())
}

#[test]
fn item_and_byte_limits_use_their_named_units() {
    let mut string = String::new();
    assert_eq!(push_with_byte_limit(&mut string, 'a', 2), Ok(()));
    assert_eq!(push_with_byte_limit(&mut string, 'é', 2), Err('é'));
    assert_eq!(push_with_item_limit(&mut string, 'é', 2), Ok(()));
    assert_eq!(push_with_item_limit(&mut string, 'x', 2), Err('x'));
    assert_eq!(string, "aé");
    assert_eq!(
        (string.len(), string.item_count(), string.byte_len()),
        (3, 2, 3)
    );

    let mut bytes = Vec::new();
    assert_eq!(push_with_byte_limit(&mut bytes, 1u8, 1), Ok(()));
    assert_eq!(push_with_byte_limit(&mut bytes, 2u8, 1), Err(2));
    assert_eq!(
        (bytes.len(), bytes.item_count(), bytes.byte_len()),
        (1, 1, 1)
    );

    let mut words = Vec::new();
    assert_eq!(push_with_item_limit(&mut words, 7u32, 1), Ok(()));
    assert_eq!(push_with_item_limit(&mut words, 8u32, 1), Err(8));
    assert_eq!((words.len(), words.item_count()), (1, 1));
}
