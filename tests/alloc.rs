use std::alloc::{Global, System};

use collections::CollectionAlloc;

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
    assert_eq!(collection.len(), items.len());

    collection.clear();
    assert!(collection.is_empty());

    let collection = C::new_in(allocator);
    assert!(collection.is_empty());
}

#[test]
fn vec_allocators_follow_the_collection_contract() {
    check_allocator::<Vec<u8, Global>>(Global, &[1, 2]);
    check_allocator::<Vec<u8, System>>(System, &[1, 2]);
}

#[cfg(feature = "hashbrown")]
#[test]
fn hashbrown_allocators_follow_the_collection_contract() {
    use collections::allocator_api2::alloc::Global as HashbrownGlobal;
    use collections::hashbrown::{DefaultHashBuilder, HashMap, HashSet};

    type AllocHashSet<T, A> = HashSet<T, DefaultHashBuilder, A>;
    type AllocHashMap<K, V, A> = HashMap<K, V, DefaultHashBuilder, A>;

    check_allocator::<AllocHashSet<u8, HashbrownGlobal>>(HashbrownGlobal, &[1, 2]);
    check_allocator::<AllocHashMap<u8, u8, HashbrownGlobal>>(HashbrownGlobal, &[(1, 10), (2, 20)]);
}
