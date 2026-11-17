//! Firmware with a small heap: the `alloc` collections and a `hashbrown` map
//! are used through the generic traits, and allocation failures are reported
//! instead of aborting.

#![no_std]
#![no_main]

extern crate alloc;

use alloc::collections::{BTreeMap, VecDeque};
use alloc::string::String;
use alloc::vec::Vec;
use core::hash::{BuildHasherDefault, Hasher};
use core::mem::MaybeUninit;

use collections::{
    Collection, CollectionBase, CollectionBytes, CollectionCapacity, CollectionEntry,
    CollectionMap, MapEntry,
};
use cortex_m_rt::entry;
use cortex_m_semihosting::{debug, hprintln};
use embedded_alloc::LlffHeap as Heap;
use panic_semihosting as _;

const HEAP_SIZE: usize = 16 * 1024;

#[global_allocator]
static HEAP: Heap = Heap::empty();

/// FNV-1a with the standard offset basis. Bare-metal targets have no source of
/// entropy, so the hasher is explicit instead of a silently constant default.
#[derive(Clone, Copy)]
struct Fnv1a(u64);

impl Default for Fnv1a {
    fn default() -> Self {
        Self(0xcbf2_9ce4_8422_2325)
    }
}

impl Hasher for Fnv1a {
    fn finish(&self) -> u64 {
        self.0
    }

    fn write(&mut self, bytes: &[u8]) {
        for byte in bytes {
            self.0 = (self.0 ^ u64::from(*byte)).wrapping_mul(0x0000_0100_0000_01b3);
        }
    }
}

type Fnv = BuildHasherDefault<Fnv1a>;

/// Works with any collection of `u32`.
fn squares<C: Collection<Item = u32>>(count: u32) -> C {
    let mut collection = C::with_capacity(count as usize);
    for no in 1..=count {
        collection.push(no * no);
    }
    collection
}

/// Counts words with the generic entry API.
fn count<'a, M: CollectionEntry<Key = &'a str, Value = usize>>(map: &mut M, text: &'a str) {
    for word in text.split_whitespace() {
        *map.entry(word).or_insert(0) += 1;
    }
}

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

#[entry]
fn main() -> ! {
    {
        static mut HEAP_MEM: [MaybeUninit<u8>; HEAP_SIZE] = [MaybeUninit::uninit(); HEAP_SIZE];
        // SAFETY: `main` runs once, and `HEAP_MEM` is used by nothing else.
        unsafe { HEAP.init(&raw mut HEAP_MEM as usize, HEAP_SIZE) }
    }

    let vec: Vec<u32> = squares(4);
    assert_eq!(vec, [1, 4, 9, 16]);

    let deque: VecDeque<u32> = squares(4);
    assert_eq!(deque.item_count(), 4);
    assert_eq!(deque.back(), Some(&16));

    let mut ordered = BTreeMap::new();
    count(&mut ordered, "a b a");
    assert_eq!(ordered.get(&"a"), Some(&2));

    let mut hashed: hashbrown::HashMap<&str, usize, Fnv> = Collection::with_capacity(4);
    count(&mut hashed, "x y x");
    assert_eq!(CollectionMap::get(&hashed, &"x"), Some(&2));
    assert_eq!(hashed.item_count(), 2);

    let mut string = String::new();
    assert_eq!(CollectionCapacity::try_push(&mut string, '€'), Ok(()));
    assert_eq!((string.item_count(), string.byte_len()), (1, 3));

    // Exhausting the heap is reported to the caller instead of aborting.
    let mut big: Vec<u8> = Vec::new();
    assert!(CollectionCapacity::try_reserve(&mut big, 1024 * 1024).is_err());
    assert!(CollectionCapacity::try_reserve(&mut big, usize::MAX).is_err());
    assert!(CollectionCapacity::try_reserve(&mut hashed, 1024 * 1024).is_err());
    assert_eq!(fill(&mut big, 32), 32);

    // Fixed-capacity collections work next to the heap ones.
    let mut fixed = collections::heapless::Vec::<u8, 8>::new();
    assert_eq!(fill(&mut fixed, 32), 8);

    hprintln!("collections: bare-metal heap contract OK");
    debug::exit(debug::EXIT_SUCCESS);

    #[allow(clippy::empty_loop)]
    loop {}
}
