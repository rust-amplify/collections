#![cfg(feature = "alloc")]

#[cfg(feature = "hashbrown")]
mod common;

use std::collections::BTreeSet;
#[cfg(feature = "std")]
use std::collections::HashSet;
#[cfg(any(feature = "std", feature = "indexmap", feature = "hashbrown"))]
use std::collections::hash_map::DefaultHasher;
#[cfg(any(feature = "std", feature = "indexmap", feature = "hashbrown"))]
use std::hash::BuildHasherDefault;

use collections::CollectionSet;

fn check_set<C: CollectionSet<Item = u8>>(mut set: C) {
    assert!(set.is_empty());

    assert!(set.insert(1));
    assert!(set.insert(2));
    assert!(set.insert(3));
    assert!(!set.insert(1));
    assert_eq!(set.item_count(), 3);

    assert!(set.contains(&1));
    assert_eq!(set.get(&2), Some(&2));
    assert_eq!(set.get(&99), None);

    let mut items: Vec<_> = set.iter().copied().collect();
    items.sort_unstable();
    assert_eq!(items, [1, 2, 3]);

    assert_eq!(set.replace(2), Some(2));
    assert!(set.insert(4));
    assert_eq!(set.replace(4), Some(4));
    assert_eq!(set.replace(99), None);
    assert!(set.contains(&99));
    assert_eq!(set.item_count(), 5);

    set.retain(|item| item % 2 == 0);
    assert_eq!(set.item_count(), 2);
    assert!(set.contains(&2));
    assert!(set.contains(&4));
    assert!(set.remove(&2));
    assert!(!set.remove(&2));
    assert_eq!(set.take(&4), Some(4));
    assert_eq!(set.take(&4), None);

    set.clear();
    assert!(set.is_empty());
}

#[test]
fn set_families_follow_the_set_contract() {
    check_set(BTreeSet::<u8>::new());

    #[cfg(feature = "std")]
    {
        check_set(HashSet::<u8>::new());

        type CustomHashSet<T> = HashSet<T, BuildHasherDefault<DefaultHasher>>;
        check_set(CustomHashSet::<u8>::default());
    }

    #[cfg(feature = "hashbrown")]
    {
        use collections::hashbrown::HashSet;

        check_set(HashSet::<u8>::new());

        type CustomHashSet<T> = HashSet<T, BuildHasherDefault<DefaultHasher>>;
        check_set(CustomHashSet::<u8>::default());

        let allocator = common::TestAlloc::default();
        check_set(HashSet::<u8, _, _>::new_in(allocator.clone()));
        assert!(allocator.allocations() > 0);
    }

    #[cfg(feature = "indexmap")]
    {
        use collections::indexmap::IndexSet;

        type CustomIndexSet<T> = IndexSet<T, BuildHasherDefault<DefaultHasher>>;
        check_set(CustomIndexSet::<u8>::default());

        #[cfg(feature = "std")]
        check_set(IndexSet::<u8>::new());
    }
}

#[cfg(feature = "indexmap")]
#[test]
fn indexset_removal_preserves_the_order_of_remaining_items() {
    use collections::indexmap::IndexSet;

    type CustomIndexSet<T> = IndexSet<T, BuildHasherDefault<DefaultHasher>>;

    let mut set = CustomIndexSet::<u8>::default();
    set.insert(1);
    set.insert(2);
    set.insert(3);

    assert!(CollectionSet::remove(&mut set, &2));
    assert_eq!(set.iter().copied().collect::<Vec<_>>(), [1, 3]);
    assert_eq!(CollectionSet::take(&mut set, &1), Some(1));
    assert_eq!(set.iter().copied().collect::<Vec<_>>(), [3]);
}
