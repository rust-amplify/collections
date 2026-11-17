//! Firmware without a heap: neither `alloc` nor a global allocator is linked,
//! and fixed-capacity collections are used through the generic traits.

#![no_std]
#![no_main]

use collections::heapless::binary_heap::Max;
use collections::heapless::index_map::FnvIndexMap;
use collections::heapless::index_set::FnvIndexSet;
use collections::heapless::{BinaryHeap, Deque, LinearMap, String, Vec};
use collections::{
    CollectionBase, CollectionBytes, CollectionCapacity, CollectionEntry, CollectionMap, MapEntry,
};
use cortex_m_rt::entry;
use cortex_m_semihosting::{debug, hprintln};
use panic_semihosting as _;

/// Pushes items until the collection reports that it is full.
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

/// Counts words with the generic entry API.
fn count<'a, M: CollectionEntry<Key = &'a str, Value = usize>>(map: &mut M, text: &'a str) {
    for word in text.split_whitespace() {
        *map.entry(word).or_insert(0) += 1;
    }
}

/// Stores squares using the map operations only.
fn squares<M: CollectionMap<Key = u8, Value = u16>>(map: &mut M, count: u8) {
    for no in 1..=count {
        map.insert(no, u16::from(no) * u16::from(no));
    }
}

#[entry]
fn main() -> ! {
    let mut vec = Vec::<u8, 8>::new();
    assert_eq!(fill(&mut vec, 32), 8);
    assert_eq!(vec.item_count(), 8);
    assert_eq!(vec.byte_len(), 8);
    assert!(vec.try_reserve(1).is_err());

    let mut deque = Deque::<u8, 4>::new();
    assert_eq!(fill(&mut deque, 32), 4);
    assert_eq!(deque.front(), Some(&0));

    let mut heap = BinaryHeap::<u8, Max, 4>::new();
    assert_eq!(fill(&mut heap, 32), 4);
    assert_eq!(heap.peek(), Some(&3));

    let mut set = FnvIndexSet::<u8, 4>::new();
    assert_eq!(fill(&mut set, 32), 4);
    assert_eq!(set.try_push(0), Ok(()));
    assert_eq!(set.item_count(), 4);

    let mut string = String::<4>::new();
    assert_eq!(string.try_push('a'), Ok(()));
    assert_eq!(string.try_push('€'), Ok(()));
    assert_eq!(string.try_push('b'), Err('b'));
    assert_eq!((string.item_count(), string.byte_len()), (2, 4));

    let mut words = FnvIndexMap::<&str, usize, 8>::new();
    count(&mut words, "x y x");
    assert_eq!(CollectionMap::get(&words, &"x"), Some(&2));
    assert_eq!(words.item_count(), 2);

    let mut linear = LinearMap::<u8, u16, 4>::new();
    squares(&mut linear, 4);
    assert_eq!(CollectionMap::get(&linear, &4), Some(&16));
    assert_eq!(linear.try_push((5, 25)), Err((5, 25)));
    assert_eq!(linear.try_push((4, 0)), Ok(()));

    hprintln!("collections: no-heap contract OK");
    debug::exit(debug::EXIT_SUCCESS);

    #[allow(clippy::empty_loop)]
    loop {}
}
