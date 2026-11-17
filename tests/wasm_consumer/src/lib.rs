//! Contract checks which are executed on `wasm32-unknown-unknown` by
//! `run.mjs`. A failed check panics, which traps the WebAssembly module.

use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, LinkedList, VecDeque};
use std::sync::atomic::{AtomicU32, Ordering};

use collections::heapless::index_map::FnvIndexMap;
use collections::heapless::{LinearMap, String as FixedString, Vec as FixedVec};
use collections::{
    Collection, CollectionBase, CollectionBytes, CollectionCapacity, CollectionEntry,
    CollectionMap, CollectionSet, MapEntry, hashbrown, indexmap,
};

/// Number of the checks which have passed since the module was instantiated.
static PASSED: AtomicU32 = AtomicU32::new(0);

#[track_caller]
fn check(condition: bool) {
    assert!(condition);
    PASSED.fetch_add(1, Ordering::Relaxed);
}

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

/// Inserts a repeated value and returns the number of the stored ones.
fn unique<S: CollectionSet<Item = u8>>(set: &mut S) -> usize {
    set.insert(1);
    set.insert(1);
    set.insert(2);
    set.item_count()
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

/// Runs the contract checks and returns their number.
#[unsafe(no_mangle)]
pub extern "C" fn run() -> u32 {
    // Constructors, `push` and `item_count` of every family
    check(squares::<Vec<u32>>(3) == [1, 4, 9]);
    check(squares::<VecDeque<u32>>(3).item_count() == 3);
    check(squares::<LinkedList<u32>>(3).item_count() == 3);
    check(squares::<BinaryHeap<u32>>(3).peek() == Some(&9));
    check(squares::<BTreeSet<u32>>(3).item_count() == 3);
    check(squares::<HashSet<u32>>(3).contains(&9));
    check(squares::<hashbrown::HashSet<u32>>(3).contains(&9));
    check(squares::<indexmap::IndexSet<u32>>(3).contains(&9));
    check(squares::<FixedVec<u32, 4>>(3).as_slice() == [1, 4, 9]);

    // Sets
    check(unique(&mut BTreeSet::new()) == 2);
    check(unique(&mut HashSet::new()) == 2);
    check(unique(&mut hashbrown::HashSet::new()) == 2);
    check(unique(&mut indexmap::IndexSet::new()) == 2);

    // Maps and their entries
    let mut map = BTreeMap::new();
    count(&mut map, "a b a");
    check(CollectionMap::get(&map, &"a") == Some(&2));

    let mut map = HashMap::new();
    count(&mut map, "a b a");
    check(CollectionMap::get(&map, &"a") == Some(&2));

    let mut map = hashbrown::HashMap::new();
    count(&mut map, "a b a");
    check(CollectionMap::get(&map, &"a") == Some(&2));

    let mut map = indexmap::IndexMap::new();
    count(&mut map, "a b a");
    check(CollectionMap::get(&map, &"a") == Some(&2));

    let mut map = FnvIndexMap::<&str, usize, 4>::new();
    count(&mut map, "a b a");
    check(CollectionMap::get(&map, &"a") == Some(&2));

    let mut map = LinearMap::<u8, u8, 1>::new();
    check(CollectionMap::insert(&mut map, 1, 10).is_none());
    check(map.try_push((1, 11)) == Ok(()));
    check(map.try_push((2, 20)) == Err((2, 20)));

    // Capacity and fallible insertion
    check(fill(&mut FixedVec::<u8, 8>::new(), 32) == 8);
    check(fill(&mut Vec::new(), 32) == 32);
    check(fill(&mut VecDeque::new(), 32) == 32);
    check(fill(&mut HashSet::new(), 32) == 32);
    check(fill(&mut hashbrown::HashSet::new(), 32) == 32);

    let mut big = Vec::<u8>::new();
    check(CollectionCapacity::try_reserve(&mut big, usize::MAX).is_err());
    let mut big = hashbrown::HashMap::<u64, u64>::new();
    check(CollectionCapacity::try_reserve(&mut big, usize::MAX).is_err());

    // Items and bytes
    let mut string = String::new();
    check(CollectionCapacity::try_push(&mut string, 'a') == Ok(()));
    check(CollectionCapacity::try_push(&mut string, 'é') == Ok(()));
    check((string.len(), string.item_count(), string.byte_len()) == (3, 2, 3));

    let mut string = FixedString::<3>::new();
    check(string.try_push('a') == Ok(()));
    check(string.try_push('é') == Ok(()));
    check(string.try_push('é') == Err('é'));
    check((string.item_count(), string.byte_len()) == (2, 3));

    PASSED.load(Ordering::Relaxed)
}

/// Negative control of the harness: its check fails, so the module must trap.
#[unsafe(no_mangle)]
pub extern "C" fn run_failing() -> u32 {
    check(squares::<Vec<u32>>(3) == [1, 4, 10]);
    PASSED.load(Ordering::Relaxed)
}

/// Returns the number of the checks passed so far, also after a trap.
#[unsafe(no_mangle)]
pub extern "C" fn passed() -> u32 {
    PASSED.load(Ordering::Relaxed)
}
