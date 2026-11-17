#[cfg(any(feature = "std", feature = "indexmap", feature = "hashbrown"))]
use std::collections::hash_map::DefaultHasher;
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, LinkedList, VecDeque};
#[cfg(feature = "std")]
use std::collections::{HashMap, HashSet};
#[cfg(any(feature = "std", feature = "indexmap", feature = "hashbrown"))]
use std::hash::BuildHasherDefault;

use collections::Collection;

fn check_collection<C: Collection>(items: &[C::Item])
where
    C::Item: Clone,
{
    assert!(!items.is_empty());

    let mut collection = C::with_capacity(items.len());
    assert!(collection.is_empty());
    collection.push(items[0].clone());
    collection.extend(items[1..].iter().cloned());
    assert_eq!(collection.len(), items.len());

    let collected: C = items.iter().cloned().collect();
    assert_eq!(collected.len(), items.len());

    collection.clear();
    assert!(collection.is_empty());
}

#[test]
fn collection_implementations_follow_the_common_contract() {
    check_collection::<String>(&['a', 'b']);
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
    Collection::push(&mut string, 'é');
    assert_eq!(string.len(), 2);

    let mut set = BTreeSet::<u8>::with_capacity(8);
    Collection::push(&mut set, 1);
    Collection::push(&mut set, 1);
    assert_eq!(set.len(), 1);

    let mut map = BTreeMap::<u8, u8>::with_capacity(8);
    Collection::push(&mut map, (1, 10));
    Collection::push(&mut map, (1, 20));
    assert_eq!(map.len(), 1);
    assert_eq!(map.get(&1), Some(&20));

    let mut deque = VecDeque::with_capacity(2);
    Collection::push(&mut deque, 1);
    Collection::push(&mut deque, 2);
    assert_eq!(deque.front(), Some(&1));
    assert_eq!(deque.back(), Some(&2));

    let mut list = LinkedList::<u8>::with_capacity(2);
    Collection::push(&mut list, 1);
    Collection::push(&mut list, 2);
    assert_eq!(list.front(), Some(&1));
    assert_eq!(list.back(), Some(&2));
}
