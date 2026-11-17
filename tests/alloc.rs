#![cfg(feature = "alloc")]

mod common;

use std::alloc::{Global, System};

use collections::{Collection, CollectionAlloc, CollectionCapacity};
use common::TestAlloc;

fn check_allocator<C: CollectionAlloc>(allocator: C::Allocator, items: &[C::Item])
where
    C::Allocator: Clone,
    C::Item: Clone,
{
    assert!(!items.is_empty());

    let mut collection = C::with_capacity_in(items.len(), allocator.clone());
    assert!(collection.is_empty());
    let _: &C::Allocator = collection.allocator();
    collection.push(items[0].clone());
    collection.extend(items[1..].iter().cloned());
    assert_eq!(collection.item_count(), items.len());

    collection.clear();
    assert!(collection.is_empty());

    let collection = C::new_in(allocator);
    assert!(collection.is_empty());
}

#[test]
fn vec_allocators_follow_the_collection_contract() {
    check_allocator::<Vec<u8, Global>>(Global, &[1, 2]);
    check_allocator::<Vec<u8, System>>(System, &[1, 2]);

    let allocator = TestAlloc::default();
    check_allocator::<Vec<u8, TestAlloc>>(allocator.clone(), &[1, 2]);
    assert!(allocator.allocations() > 0);
}

#[cfg(feature = "hashbrown")]
#[test]
fn hashbrown_allocators_follow_the_collection_contract() {
    use collections::hashbrown::{DefaultHashBuilder, HashMap, HashSet};

    type AllocHashSet<T, A> = HashSet<T, DefaultHashBuilder, A>;
    type AllocHashMap<K, V, A> = HashMap<K, V, DefaultHashBuilder, A>;

    check_allocator::<AllocHashSet<u8, Global>>(Global, &[1, 2]);
    check_allocator::<AllocHashMap<u8, u8, Global>>(Global, &[(1, 10), (2, 20)]);
    check_allocator::<AllocHashSet<u8, System>>(System, &[1, 2]);
    check_allocator::<AllocHashMap<u8, u8, System>>(System, &[(1, 10), (2, 20)]);

    let allocator = TestAlloc::default();
    check_allocator::<AllocHashSet<u8, TestAlloc>>(allocator.clone(), &[1, 2]);
    assert!(allocator.allocations() > 0);

    let allocator = TestAlloc::default();
    check_allocator::<AllocHashMap<u8, u8, TestAlloc>>(allocator.clone(), &[(1, 10), (2, 20)]);
    assert!(allocator.allocations() > 0);
}

#[cfg(feature = "hashbrown")]
#[test]
fn one_allocator_type_works_for_vec_and_hashbrown() {
    use std::alloc::Allocator;

    use collections::CollectionBase;
    use collections::hashbrown::{DefaultHashBuilder, HashSet};

    fn check<A: Allocator + Clone>(allocator: A) {
        fn make<C: CollectionAlloc<Item = u8>>(allocator: C::Allocator) -> C {
            let mut collection = C::with_capacity_in(2, allocator);
            collection.push(1);
            collection.push(1);
            collection
        }

        let vec: Vec<u8, A> = make(allocator.clone());
        let set: HashSet<u8, DefaultHashBuilder, A> = make(allocator);
        assert_eq!(vec.item_count(), 2);
        assert_eq!(set.item_count(), 1);
    }

    check(Global);
    check(System);

    let allocator = TestAlloc::default();
    check(allocator.clone());
    assert!(allocator.allocations() >= 2);
}

#[test]
fn capacity_methods_work_with_custom_allocators() {
    let mut vec = Vec::<u8, TestAlloc>::new_in(TestAlloc::default());
    assert_eq!(CollectionCapacity::try_push(&mut vec, 7), Ok(()));
    assert!(CollectionCapacity::try_reserve(&mut vec, usize::MAX).is_err());
    assert_eq!(vec, [7]);

    #[cfg(feature = "hashbrown")]
    {
        use collections::CollectionBase;
        use collections::hashbrown::{DefaultHashBuilder, HashMap, HashSet};

        let mut set = HashSet::<u8, DefaultHashBuilder, _>::new_in(TestAlloc::default());
        assert_eq!(CollectionCapacity::try_push(&mut set, 7), Ok(()));
        assert!(CollectionCapacity::try_reserve(&mut set, usize::MAX).is_err());
        assert_eq!(set.item_count(), 1);

        let mut map = HashMap::<u8, u8, DefaultHashBuilder, _>::new_in(TestAlloc::default());
        assert_eq!(CollectionCapacity::try_push(&mut map, (7, 9)), Ok(()));
        assert!(CollectionCapacity::try_reserve(&mut map, usize::MAX).is_err());
        assert_eq!(map.item_count(), 1);
    }
}

#[test]
fn collection_and_allocator_bounds_share_the_base_methods() {
    fn push<C: Collection + CollectionAlloc>(collection: &mut C, item: C::Item) -> usize {
        collection.push(item);
        collection.item_count()
    }

    let mut vec = Vec::<u8, Global>::new();
    assert_eq!(push(&mut vec, 7), 1);
    assert!(!vec.is_empty());

    #[cfg(feature = "hashbrown")]
    {
        use collections::hashbrown::HashSet;

        let mut set = HashSet::<u8>::new();
        assert_eq!(push(&mut set, 7), 1);
        assert_eq!(push(&mut set, 7), 1);
    }
}
