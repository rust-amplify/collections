#![cfg(feature = "alloc")]

use std::collections::{BinaryHeap, VecDeque};

use collections::{CollectionBase, CollectionBytes, CollectionCapacity};

fn check_capacity<C: CollectionCapacity>(mut collection: C, item: C::Item) {
    collection.reserve(4);
    assert!(collection.capacity() >= 4);
    assert!(collection.try_reserve(8).is_ok());
    assert!(collection.capacity() >= 8);
    assert!(collection.is_empty());

    assert!(collection.try_push(item).is_ok());
    assert_eq!(collection.item_count(), 1);

    assert!(collection.try_reserve(usize::MAX).is_err());
    assert_eq!(collection.item_count(), 1);
}

#[test]
fn capacity_implementations_follow_the_common_contract() {
    check_capacity(String::new(), 'a');
    check_capacity(Vec::<u8>::new(), 1);
    check_capacity(VecDeque::<u8>::new(), 1);
    check_capacity(BinaryHeap::<u8>::new(), 1);

    #[cfg(feature = "std")]
    {
        use std::collections::{HashMap, HashSet};

        check_capacity(HashSet::<u8>::new(), 1);
        check_capacity(HashMap::<u8, u8>::new(), (1, 1));
    }

    #[cfg(feature = "hashbrown")]
    {
        use collections::hashbrown::{HashMap, HashSet};

        check_capacity(HashSet::<u8>::new(), 1);
        check_capacity(HashMap::<u8, u8>::new(), (1, 1));
    }

    #[cfg(feature = "indexmap")]
    {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::BuildHasherDefault;

        use collections::indexmap::{IndexMap, IndexSet};

        type CustomIndexSet<T> = IndexSet<T, BuildHasherDefault<DefaultHasher>>;
        type CustomIndexMap<K, V> = IndexMap<K, V, BuildHasherDefault<DefaultHasher>>;

        check_capacity(CustomIndexSet::<u8>::default(), 1);
        check_capacity(CustomIndexMap::<u8, u8>::default(), (1, 1));
    }
}

#[cfg(any(feature = "std", feature = "hashbrown", feature = "indexmap"))]
#[test]
fn try_push_does_not_duplicate_existing_set_values_and_map_keys() {
    fn check<C: CollectionCapacity>(mut collection: C, first: C::Item, second: C::Item) {
        assert!(collection.try_push(first).is_ok());
        assert!(collection.try_push(second).is_ok());
        assert_eq!(collection.item_count(), 1);
    }

    #[cfg(feature = "std")]
    {
        use std::collections::{HashMap, HashSet};

        check(HashSet::<u8>::new(), 1, 1);
        check(HashMap::<u8, u8>::new(), (1, 1), (1, 2));
    }

    #[cfg(feature = "hashbrown")]
    {
        use collections::hashbrown::{HashMap, HashSet};

        check(HashSet::<u8>::new(), 1, 1);
        check(HashMap::<u8, u8>::new(), (1, 1), (1, 2));
    }

    #[cfg(feature = "indexmap")]
    {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::BuildHasherDefault;

        use collections::indexmap::{IndexMap, IndexSet};

        type CustomIndexSet<T> = IndexSet<T, BuildHasherDefault<DefaultHasher>>;
        type CustomIndexMap<K, V> = IndexMap<K, V, BuildHasherDefault<DefaultHasher>>;

        check(CustomIndexSet::<u8>::default(), 1, 1);
        check(CustomIndexMap::<u8, u8>::default(), (1, 1), (1, 2));
    }
}

#[test]
fn string_try_push_reserves_the_char_utf8_width() {
    let mut string = String::new();
    for (item, width) in [('a', 1), ('é', 2), ('€', 3), ('🦀', 4)] {
        let before = string.byte_len();
        assert_eq!(CollectionCapacity::try_push(&mut string, item), Ok(()));
        assert_eq!(string.byte_len(), before + width);
        assert!(string.capacity() >= string.byte_len());
    }
    assert_eq!(string, "aé€🦀");
    assert_eq!(string.item_count(), 4);
}

#[test]
fn string_capacity_counts_bytes() {
    let mut string = String::new();
    CollectionCapacity::reserve(&mut string, 4);
    let capacity = CollectionCapacity::capacity(&string);
    assert!(capacity >= 4);

    CollectionBase::push(&mut string, '🦀');
    assert_eq!(string.item_count(), 1);
    assert_eq!(string.byte_len(), 4);
    assert_eq!(CollectionCapacity::capacity(&string), capacity);
}
