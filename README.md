# Traits for collections, implemented for standard and fixed-capacity collections

![Build](https://github.com/rust-amplify/collections/workflows/Build/badge.svg)
![Tests](https://github.com/rust-amplify/collections/workflows/Tests/badge.svg)

[![crates.io](https://img.shields.io/crates/v/collections-traits)](https://crates.io/crates/collections-traits)
[![Docs](https://docs.rs/collections-traits/badge.svg)](https://docs.rs/collections-traits)
[![License](https://img.shields.io/crates/l/collections-traits)](./LICENSE)

`collections` is a small Rust library providing "the missed collection traits":
a common interface over the collection types of the standard library, so that
generic code can be written once and work with a `Vec`, a `HashSet`, a
`BTreeMap` or any other supported collection, including allocator-aware and
fixed-capacity ones.

The package is named `collections-traits`, and its library is imported as
`collections`.

The crate contains:

- traits for base operations ([`CollectionBase`](#collectionbase)), byte
  buffers ([`CollectionBytes`](#collectionbytes)), constructors
  ([`Collection`](#collection), [`CollectionAlloc`](#collectionalloc), and
  [`CollectionCapacity`](#collectioncapacity)), sets
  ([`CollectionSet`](#collectionset)), maps ([`CollectionMap`](#collectionmap)
  and [`CollectionEntry`](#collectionentry)), and [`MapEntry`](#mapentry);
- their implementations for the collections from `alloc` and `std`, and
  optionally for the types from the [`ascii`], [`hashbrown`], [`heapless`] and
  [`indexmap`] crates;
- a few [convenience macros](#macros) for constructing collections, similar to
  `vec!`.

The traits themselves need neither `std`, `alloc` nor a global allocator, so
the crate can be used in firmware without a heap, where the fixed-capacity
collections of [`heapless`] are available through the same traits. All
integrations are optional features, the crate has no wasm-specific dependency,
and its own code contains no `unsafe`.

## Contents

- [Motivation](#motivation)
- [Installation](#installation)
- [Cargo features](#cargo-features)
- [Traits](#traits)
- [Supported collection types](#supported-collection-types)
- [Macros](#macros)
- [Platform support](#platform-support)
- [Security considerations](#security-considerations)
- [Known limitations](#known-limitations)
- [Minimum supported Rust version](#minimum-supported-rust-version)
- [Project layout](#project-layout)
- [Development](#development)
- [Contributing](#contributing)
- [License](#license)

## Motivation

Rust standard collections share a lot of methods (`len`, `clear`,
`with_capacity`, `insert`, ...), but these are inherent methods of each type:
the standard library has no trait which would say "this type is a collection".
Because of that, it is not possible to write a function or a data type which is
generic over the kind of collection it works with.

This crate fills that gap:

```rust
use std::collections::{BTreeSet, HashSet, VecDeque};

use collections::Collection;

/// Works with any collection of `u32`.
fn squares<C: Collection<Item = u32>>(count: u32) -> C {
    let mut collection = C::with_capacity(count as usize);
    for no in 1..=count {
        collection.push(no * no);
    }
    collection
}

let vec: Vec<u32> = squares(3);
let deque: VecDeque<u32> = squares(3);
let ordered: BTreeSet<u32> = squares(3);
let hashed: HashSet<u32> = squares(3);

assert_eq!(vec, [1, 4, 9]);
assert_eq!(deque, [1, 4, 9]);
assert_eq!(ordered.len(), 3);
assert!(hashed.contains(&9));
```

The code was originally a part of the [`amplify`] crate and was moved into a
separate library.

## Installation

The crate is at an early stage (version `0.1.0`), and at the moment of writing
it is not published on crates.io, so it has to be added as a git dependency:

```toml
[dependencies]
collections = { package = "collections-traits", git = "https://github.com/rust-amplify/collections" }
```

Selecting features:

```toml
[dependencies]
# Everything: `std`, `ascii`, `hashbrown`, `heapless` and `indexmap`
collections = { package = "collections-traits", git = "https://github.com/rust-amplify/collections", features = ["all"] }
```

```toml
[dependencies]
# `no_std` with a heap: the collections of the `alloc` crate
collections = { package = "collections-traits", git = "https://github.com/rust-amplify/collections", default-features = false, features = ["alloc"] }
```

```toml
[dependencies]
# `no_std` without a heap: the traits and fixed-capacity collections only
collections = { package = "collections-traits", git = "https://github.com/rust-amplify/collections", default-features = false, features = ["heapless"] }
```

With `default-features = false` and no other feature the crate provides the
traits only. It does not link the `alloc` crate then, so it needs no global
allocator; add the `alloc` feature to get the implementations for `Vec`,
`String`, `BTreeMap` and the other `alloc` collections back.

## Cargo features

| Feature     | Default | Implies | Description                                                                                          |
|-------------|:-------:|---------|------------------------------------------------------------------------------------------------------|
| `std`       |   yes   | `alloc` | Uses the standard library: adds implementations for `HashMap` and `HashSet`, and `map!`/`set!` macros. |
| `alloc`     |   yes   |         | Implements the traits for the collections of the `alloc` crate and adds the `alloc` macros.           |
| `ascii`     |   no    | `alloc` | Adds the [`ascii`] dependency and implements the traits for `AsciiString`.                            |
| `indexmap`  |   no    | `alloc` | Adds the [`indexmap`] dependency and implements the traits for `IndexMap` and `IndexSet`.             |
| `hashbrown` |   no    | `alloc` | Adds [`hashbrown`] support for allocator-aware hash maps and sets using `core::alloc::Allocator`.     |
| `heapless`  |   no    |         | Adds the [`heapless`] dependency and implements the traits for its fixed-capacity collections.       |
| `all`       |   no    | all of them | Shortcut for `std` + `ascii` + `hashbrown` + `heapless` + `indexmap`.                            |

The `alloc` feature is on by default only because `std` implies it. Without
`alloc` the crate does not link the `alloc` crate, so the traits and the
`heapless` implementations can be used in programs which have no global
allocator.

When optional dependencies are enabled, they are re-exported as
`collections::ascii`, `collections::hashbrown`, `collections::heapless` and
`collections::indexmap`. The `alloc` crate is re-exported as
`collections::alloc` with the `alloc` feature.

## Traits

### `CollectionBase`

`CollectionBase` is the constructor-free root, implemented by every supported
collection. `item_count()` counts values of the associated `Item` type: it
counts `char`s for strings and elements or entries for other collections.
Counting a string's characters is O(n); it is O(1) for other types.
`is_empty()` is O(1) for every implementation. `push` appends to sequences and
linked lists, inserts into sets and maps, pushes into a `BinaryHeap`, and adds a
UTF-8 encoded `char` to a string. Duplicate set values do not increase the
item count, while pushing a map pair with an existing key replaces its value.

`push` is infallible. A growable collection panics or aborts when the memory
cannot be allocated, and a fixed-capacity collection panics with the message
`fixed-capacity collection is full` when it has no room left. Use
[`CollectionCapacity::try_push`](#collectioncapacity) to get the item back
instead.

| Item                              | Description                                                  |
|-----------------------------------|--------------------------------------------------------------|
| `type Item`                       | Type of the values contained in the collection.              |
| `fn item_count(&self) -> usize`   | Number of `Item`s; O(n) for strings and O(1) otherwise.       |
| `fn is_empty(&self) -> bool`      | O(1) check for an empty collection.                          |
| `fn push(&mut self, Item)`        | Appends or inserts an element into the collection.           |
| `fn clear(&mut self)`             | Removes all elements.                                        |

There is deliberately no `len` method in the traits. Calling `len()` on a
concrete value uses that type's inherent method, while generic code names its
unit explicitly with `item_count()` or `byte_len()`:

| Value                      | Inherent `len()` | `item_count()` | `byte_len()`    |
|----------------------------|:----------------:|:--------------:|:---------------:|
| `String` holding `"aé"`    |        3         |       2        |        3        |
| `Vec<u8>` holding `[1, 2]`  |        2         |       2        |        2        |
| `Vec<u32>` holding `[1, 2]` |        2         |       2        | not implemented |

`item_count()` has to walk a string to count its `char`s. Code which only
needs a size limit for a string or a byte buffer should use `byte_len()`,
which is O(1).

```rust
use std::collections::BTreeSet;

use collections::{CollectionBase, CollectionBytes};

/// Pushes an item unless the collection already holds `max` items.
fn push_max_items<C: CollectionBase>(collection: &mut C, item: C::Item, max: usize) -> bool {
    if collection.item_count() >= max {
        return false;
    }
    collection.push(item);
    true
}

/// Pushes an item unless the collection would exceed `max` bytes.
fn push_max_bytes<C: CollectionBytes>(collection: &mut C, item: C::Item, max: usize) -> bool {
    if collection.byte_len() + C::item_byte_len(&item) > max {
        return false;
    }
    collection.push(item);
    true
}

let mut text = String::new();
assert!(push_max_bytes(&mut text, 'a', 2));
assert!(!push_max_bytes(&mut text, 'é', 2)); // would be 3 bytes
assert!(push_max_items(&mut text, 'é', 2)); // but it is the second `char`
assert_eq!((text.len(), text.item_count(), text.byte_len()), (3, 2, 3));

let mut set = BTreeSet::new();
assert!(push_max_items(&mut set, 1, 1));
assert!(!push_max_items(&mut set, 2, 1));
```

### `CollectionBytes`

`CollectionBytes` is implemented only for contiguous byte buffers: `String`,
`Vec<u8, A>` for any allocator, `AsciiString`, `heapless::String<N>` and
`heapless::Vec<u8, N>`. `byte_len()` is O(1), and `item_byte_len` reports the
increase in bytes from pushing one item. For a `char`, that is its UTF-8
encoded length; for an ASCII or byte item, it is one. General collections such
as `Vec<u32>` or `VecDeque<u8>` do not implement this trait.

| Item                                   | Description                                      |
|----------------------------------------|--------------------------------------------------|
| `fn byte_len(&self) -> usize`          | Number of bytes stored; O(1).                    |
| `fn item_byte_len(&Item) -> usize`     | Number of bytes added by pushing one item.       |

### `Collection`

`Collection` adds default construction and iterator support to
`CollectionBase`. It requires `Default`, `FromIterator<Item>` and `Extend<Item>`,
so generic code may use `collect()` and `extend()` as well as `with_capacity`.

| Item                              | Description                                                   |
|-----------------------------------|---------------------------------------------------------------|
| `fn with_capacity(usize) -> Self` | Creates a collection, reserving capacity where supported.     |

`with_capacity` reserves items for sequences, heaps, sets and maps, or bytes
for strings. `BTreeSet`, `BTreeMap` and `LinkedList` cannot reserve and ignore
the requested capacity, as do the fixed-capacity collections, whose capacity
is a part of the type. An empty collection is created with `C::default()`.

The capacity must not be taken from untrusted input: see
[`CollectionCapacity`](#collectioncapacity).

```rust
use std::collections::BTreeSet;

use collections::Collection;

fn collect_twice<C: Collection>(items: impl IntoIterator<Item = C::Item> + Clone) -> C {
    let mut collection = items.clone().into_iter().collect::<C>();
    collection.extend(items);
    collection
}

let string: String = collect_twice(['a', 'b']);
assert_eq!(string, "abab");

let vec: Vec<u8> = collect_twice([1, 2]);
assert_eq!(vec, [1, 2, 1, 2]);

let set: BTreeSet<u8> = collect_twice([1, 2]);
assert_eq!(set.len(), 2);
```

### `CollectionAlloc`

`CollectionAlloc` is the constructor interface for allocator-aware collections.
It extends `CollectionBase` and `Extend<Item>`, but not `FromIterator`, because
custom-allocator types are not always supported by `collect()`. It is
implemented for `Vec<T, A>` and, with the `hashbrown` feature, hashbrown maps
and sets using `core::alloc::Allocator`.

| Item                                              | Description                                      |
|---------------------------------------------------|--------------------------------------------------|
| `type Allocator`                                  | Allocator used by the collection.                |
| `fn with_capacity_in(usize, Allocator) -> Self`  | Creates a collection with capacity and allocator.|
| `fn new_in(Allocator) -> Self`                   | Creates an empty collection with the allocator.  |
| `fn allocator(&self) -> &Allocator`              | Returns the allocator used by the collection.    |

The same generic code can construct vectors with different allocators:

```rust
use std::alloc::{Global, System};

use collections::CollectionAlloc;

fn values<C: CollectionAlloc<Item = u8, Allocator = Global>>() -> C {
    let mut values = C::with_capacity_in(2, Global);
    values.push(1);
    values.extend([2]);
    values
}

let global: Vec<u8, Global> = values();
assert_eq!(global, [1, 2]);

let mut system = Vec::<u8, System>::new_in(System);
system.push(3);
assert_eq!(system, [3]);
```

### `CollectionCapacity`

`CollectionCapacity` adds capacity management and fallible insertion to the
collections which have a capacity. `BTreeSet`, `BTreeMap` and `LinkedList`
have none, and `AsciiString` has no fallible reservation, so they do not
implement the trait.

| Item                                           | Description                                              |
|------------------------------------------------|----------------------------------------------------------|
| `type ReserveError`                            | Error returned by a failed reservation.                  |
| `fn capacity(&self) -> usize`                  | Current capacity in native storage units.                |
| `fn reserve(&mut self, additional: usize)`     | Panicking reservation for additional storage units.      |
| `fn try_reserve(&mut self, additional: usize)` | Fallible reservation.                                    |
| `fn try_push(&mut self, Item)`                 | Fallible insertion; returns the item if there is no room. |

Capacities are counted in the native storage units of a collection, as its
inherent methods do: bytes for strings and items for everything else. A string
with a capacity of `n` may thus hold fewer than `n` `char`s, and `try_push`
reserves the UTF-8 width of the `char` it is given.

`ReserveError` is `alloc::collections::TryReserveError` for the collections of
`alloc` and `std`, `hashbrown::TryReserveError`, `indexmap::TryReserveError`
and `heapless::CapacityError`.

Fixed-capacity collections never allocate. Their `capacity()` is constant,
`try_reserve` only checks that the requested room is left, `reserve` panics if
it is not, and `try_push` uses the fallible insertion of the collection.
Pushing a set value or a map key which is already present needs no room and
succeeds on a full collection.

The infallible methods `with_capacity`, `with_capacity_in`, `reserve` and
`push` panic or abort the process when the memory cannot be allocated. Do not
pass them a size which comes from untrusted input; create an empty collection
and use `try_reserve` and `try_push` instead:

```rust
use std::collections::VecDeque;

use collections::{Collection, CollectionCapacity};

/// Reads at most `declared` items, where `declared` comes from the input.
fn read<C>(declared: usize, items: impl IntoIterator<Item = C::Item>) -> Option<C>
where C: Collection + CollectionCapacity {
    let mut collection = C::default();
    collection.try_reserve(declared).ok()?;
    for item in items.into_iter().take(declared) {
        collection.try_push(item).ok()?;
    }
    Some(collection)
}

let vec: Option<Vec<u8>> = read(3, [1, 2, 3, 4]);
assert_eq!(vec, Some(vec![1, 2, 3]));

let text: Option<String> = read(2, ['a', 'é']);
assert_eq!(text.as_deref(), Some("aé"));

// A hostile length is rejected instead of aborting the process
let deque: Option<VecDeque<u64>> = read(usize::MAX, [1]);
assert_eq!(deque, None);
```

The same code fills a fixed-capacity collection until it is full. With the
`heapless` feature:

```rust,ignore
use collections::CollectionCapacity;
use collections::heapless::Vec as FixedVec;

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

assert_eq!(fill(&mut FixedVec::<u8, 4>::new(), 10), 4); // no heap is used
assert_eq!(fill(&mut Vec::new(), 10), 10);
```

### `CollectionSet`

`CollectionSet` extends `CollectionBase` with common set operations. Items are
unique according to the collection's equality or ordering rules.

| Item                                      | Description                                               |
|-------------------------------------------|-----------------------------------------------------------|
| `fn contains(&self, &Item) -> bool`       | Checks whether an equal item is present.                  |
| `fn get(&self, &Item) -> Option<&Item>`   | Gets a reference to the stored item equal to the argument.|
| `fn iter(&self)`                          | Iterates over the set's items.                            |
| `fn insert(&mut self, Item) -> bool`      | Inserts an item, returning whether it was new.            |
| `fn replace(&mut self, Item) -> Option<Item>` | Inserts or replaces an item, returning the previous equal item. |
| `fn remove(&mut self, &Item) -> bool`     | Removes an equal item, returning whether it was present.  |
| `fn take(&mut self, &Item) -> Option<Item>` | Removes and returns the stored item, if present.        |
| `fn retain(&mut self, impl FnMut(&Item) -> bool)` | Keeps only items accepted by the predicate.       |

The same generic function works with ordered, hashed and insertion-ordered
sets:

```rust
use std::collections::{BTreeSet, HashSet};

use collections::{Collection, CollectionSet};

fn unique<C: CollectionSet<Item = u8> + Collection>(items: impl IntoIterator<Item = u8>) -> C {
    let mut set = C::with_capacity(16);
    for item in items {
        set.insert(item);
    }
    set
}

let ordered: BTreeSet<u8> = unique([3, 1, 2, 1]);
let hashed: HashSet<u8> = unique([3, 1, 2, 1]);
assert_eq!(ordered.len(), 3);
assert!(hashed.contains(&2));
```

### `CollectionMap`

`CollectionMap` extends `CollectionBase` for key-value maps. It has
`Item = (Self::Key, Self::Value)` and does not require constructors or an entry
API, so maps without them, like `heapless::LinearMap`, implement it too.
`insert` panics if a fixed-capacity map is full and the key is not present.

| Item                                             | Description                                                          |
|--------------------------------------------------|----------------------------------------------------------------------|
| `type Key`                                       | Key type; each map implementation sets its own bounds.              |
| `type Value`                                     | Value type.                                                          |
| `fn contains_key(&self, &Key) -> bool`           | Checks whether the key is present.                                   |
| `fn get(&self, &Key) -> Option<&Value>`          | Gets a value by its key.                                             |
| `fn get_mut(&mut self, &Key) -> Option<&mut Value>` | Gets a mutable value by its key.                                  |
| `fn iter(&self)`                                 | Iterates over `(&Key, &Value)` pairs.                                |
| `fn keys(&self)`                                 | Iterates over keys.                                                  |
| `fn values(&self)`                               | Iterates over values.                                                |
| `fn iter_mut(&mut self)`                         | Iterates over `(&Key, &mut Value)` pairs.                            |
| `fn values_mut(&mut self)`                       | Iterates over mutable values.                                        |
| `fn insert(&mut self, Key, Value) -> Option<Value>` | Inserts a value, returning the previous value stored under the key. |
| `fn remove(&mut self, &Key) -> Option<Value>`    | Removes a value by its key and returns it.                           |
| `fn retain(&mut self, impl FnMut(&Key, &mut Value) -> bool)` | Keeps only the pairs for which the predicate returns `true`. |

The same generic function can be used with different kinds of maps:

```rust
use std::collections::{BTreeMap, HashMap};

use collections::{Collection, CollectionMap};

/// Counts the words which are present in the text more than once.
fn repeated_words<'a, M>(text: &'a str) -> M
where M: CollectionMap<Key = &'a str, Value = usize> + Collection {
    let mut counts = M::with_capacity(16);
    for word in text.split_whitespace() {
        match counts.get_mut(&word) {
            Some(count) => *count += 1,
            None => {
                counts.insert(word, 1);
            }
        }
    }
    counts.retain(|_, count| *count > 1);
    counts
}

let text = "the quick brown fox jumps over the lazy dog and the other fox";

let hashed: HashMap<&str, usize> = repeated_words(text);
assert_eq!(hashed.len(), 2);
assert_eq!(hashed["the"], 3);

let ordered: BTreeMap<&str, usize> = repeated_words(text);
assert_eq!(ordered.into_iter().collect::<Vec<_>>(), [("fox", 2), ("the", 3)]);
```

### `CollectionEntry`

`CollectionEntry` is an optional map capability for collections with an
in-place entry API. It is separate from `CollectionMap`, so maps without native
entries can still implement the map operations. The methods of `MapEntry`
which insert a value panic if the entry is vacant and a fixed-capacity map is
full.

| Item                                      | Description                                    |
|-------------------------------------------|------------------------------------------------|
| `type Entry<'a>: MapEntry<'a>`            | Native entry type with generic operations.     |
| `fn entry(&mut self, Key) -> Entry<'_>`   | Gets the entry for in-place manipulation.      |

### `MapEntry`

`MapEntry` is the common interface of the entries returned by
`CollectionEntry::entry`. An entry is a view into a single key of a map, which
is either occupied or vacant; it allows reading, inserting or updating the
value with a single lookup. Each standard map has its own entry type, but there
is no trait uniting them. `MapEntry` is that trait, so the entry API can be used
in generic code.

| Item                                                               | Description                                                                   |
|--------------------------------------------------------------------|-------------------------------------------------------------------------------|
| `type Key`                                                         | Key type stored in the map.                                                   |
| `type Value`                                                       | Value type stored in the map.                                                 |
| `fn key(&self) -> &Key`                                            | Returns the key of the entry.                                                 |
| `fn or_insert(self, Value) -> &'a mut Value`                       | Inserts the value if the entry is vacant; returns the value kept in the map.  |
| `fn or_insert_with(self, impl FnOnce() -> Value) -> &'a mut Value` | The same, but the value is computed only if the entry is vacant.              |
| `fn or_default(self) -> &'a mut Value`                             | Inserts `Value::default()` if the entry is vacant; requires `Value: Default`. |
| `fn and_modify(self, impl FnOnce(&mut Value)) -> Self`             | Changes the value if the entry is occupied; returns the entry for chaining.   |

The trait is declared as `MapEntry<'a>`, where `'a` is the lifetime of the
mutable borrow of the map taken by `entry`. The `or_*` methods consume the entry
and return a reference which lives as long as that borrow.

`MapEntry` is implemented for the native entry types of the supported maps,
which are the types returned by `CollectionEntry::entry`:

| Map                               | `CollectionEntry::Entry<'a>`               | Required feature |
|-----------------------------------|--------------------------------------------|------------------|
| `BTreeMap<K, V>`                  | `btree_map::Entry<'a, K, V>`               | `alloc`          |
| `HashMap<K, V, S>`                | `hash_map::Entry<'a, K, V>`                | `std`            |
| `hashbrown::HashMap<K, V, S, A>`  | `hash_map::Entry<'a, K, V, S, A>`          | `hashbrown`      |
| `IndexMap<K, V, S>`               | `indexmap::map::Entry<'a, K, V>`           | `indexmap`       |
| `heapless::IndexMap<K, V, S, N>`  | `heapless::index_map::Entry<'a, K, V, N>`  | `heapless`       |

Generic code calls the methods through the bound
`Entry<'a>: MapEntry<'a, Key = Self::Key, Value = Self::Value>` and has to
import `MapEntry` for that. With a concrete map the import is not needed: the
entry is the native type, so its inherent methods are called, and it can still
be matched as `Occupied` or `Vacant`.

The function below counts words with one lookup per word, while
`repeated_words` from the previous section needs two for each new word
(`get_mut`, then `insert`):

```rust
use std::collections::{BTreeMap, HashMap};

use collections::{Collection, CollectionEntry, CollectionMap, MapEntry};

/// Counts how many times each word occurs in the text.
fn word_counts<'a, M>(text: &'a str) -> M
where M: CollectionMap<Key = &'a str, Value = usize> + CollectionEntry + Collection {
    let mut counts = M::with_capacity(16);
    for word in text.split_whitespace() {
        *counts.entry(word).or_insert(0) += 1;
    }
    counts
}

let text = "the quick brown fox jumps over the lazy dog and the other fox";

let hashed: HashMap<&str, usize> = word_counts(text);
assert_eq!(hashed["the"], 3);
assert_eq!(hashed["dog"], 1);

let ordered: BTreeMap<&str, usize> = word_counts(text);
assert_eq!(ordered["fox"], 2);
```

All the methods in one place:

```rust
use std::collections::BTreeMap;

use collections::{CollectionEntry, CollectionMap, MapEntry};

fn update<M>(stock: &mut M)
where M: CollectionMap<Key = &'static str, Value = u32> + CollectionEntry {
    // `key` returns the key the entry was requested for
    assert_eq!(*stock.entry("apples").key(), "apples");

    // `or_insert` and `or_insert_with` insert a value only into a vacant entry
    assert_eq!(*stock.entry("apples").or_insert(10), 10);
    assert_eq!(*stock.entry("apples").or_insert(99), 10);
    assert_eq!(*stock.entry("apples").or_insert_with(|| unreachable!("occupied")), 10);

    // `or_default` inserts `u32::default()`
    assert_eq!(*stock.entry("pears").or_default(), 0);

    // `and_modify` changes only an occupied entry and can be chained with `or_*`
    stock.entry("apples").and_modify(|units| *units += 5).or_insert(1);
    stock.entry("plums").and_modify(|units| *units += 5).or_insert(1);
}

let mut stock = BTreeMap::new();
update(&mut stock);
assert_eq!(stock, BTreeMap::from([("apples", 15), ("pears", 0), ("plums", 1)]));
```

## Supported collection types

| Type                              | Feature     | `Item`      | Base | Bytes    | Collection  | Alloc | Capacity | Set | Map | Entry |
|-----------------------------------|-------------|-------------|:----:|:--------:|:-----------:|:-----:|:--------:|:---:|:---:|:-----:|
| `String`                          | `alloc`     | `char`      | yes  | yes      | yes         |       | yes      |     |     |       |
| `Vec<T, A>`                       | `alloc`     | `T`         | yes  | `T = u8` | default `A` | yes   | yes      |     |     |       |
| `VecDeque<T>`                     | `alloc`     | `T`         | yes  |          | yes         |       | yes      |     |     |       |
| `LinkedList<T>`                   | `alloc`     | `T`         | yes  |          | yes         |       |          |     |     |       |
| `BinaryHeap<T>`                   | `alloc`     | `T`         | yes  |          | yes         |       | yes      |     |     |       |
| `BTreeSet<T>`                     | `alloc`     | `T`         | yes  |          | yes         |       |          | yes |     |       |
| `BTreeMap<K, V>`                  | `alloc`     | `(K, V)`    | yes  |          | yes         |       |          |     | yes | yes   |
| `HashSet<T, S>`                   | `std`       | `T`         | yes  |          | yes         |       | yes      | yes |     |       |
| `HashMap<K, V, S>`                | `std`       | `(K, V)`    | yes  |          | yes         |       | yes      |     | yes | yes   |
| `hashbrown::HashSet<T, S, A>`     | `hashbrown` | `T`         | yes  |          | default `A` | yes   | yes      | yes |     |       |
| `hashbrown::HashMap<K, V, S, A>`  | `hashbrown` | `(K, V)`    | yes  |          | default `A` | yes   | yes      |     | yes | yes   |
| `IndexSet<T, S>`                  | `indexmap`  | `T`         | yes  |          | yes         |       | yes      | yes |     |       |
| `IndexMap<K, V, S>`               | `indexmap`  | `(K, V)`    | yes  |          | yes         |       | yes      |     | yes | yes   |
| `AsciiString`                     | `ascii`     | `AsciiChar` | yes  | yes      | yes         |       |          |     |     |       |
| `heapless::Vec<T, N>`             | `heapless`  | `T`         | yes  | `T = u8` | yes         |       | yes      |     |     |       |
| `heapless::String<N>`             | `heapless`  | `char`      | yes  | yes      |             |       | yes      |     |     |       |
| `heapless::Deque<T, N>`           | `heapless`  | `T`         | yes  |          |             |       | yes      |     |     |       |
| `heapless::BinaryHeap<T, K, N>`   | `heapless`  | `T`         | yes  |          |             |       | yes      |     |     |       |
| `heapless::IndexSet<T, S, N>`     | `heapless`  | `T`         | yes  |          | yes         |       | yes      |     |     |       |
| `heapless::IndexMap<K, V, S, N>`  | `heapless`  | `(K, V)`    | yes  |          | yes         |       | yes      |     | yes | yes   |
| `heapless::LinearMap<K, V, N>`    | `heapless`  | `(K, V)`    | yes  |          |             |       | yes      |     | yes |       |

The columns are `CollectionBase`, `CollectionBytes`, `Collection`,
`CollectionAlloc`, `CollectionCapacity`, `CollectionSet`, `CollectionMap` and
`CollectionEntry`. "default `A`" means that the trait is implemented for the
default (global) allocator only, while the other traits of that row are
implemented for every allocator.

Bounds of the implementations:

- `BinaryHeap<T>` and `BTreeSet<T>` need `T: Ord`, and `BTreeMap<K, V>` needs
  `K: Ord`.
- Hashed and indexed sets and maps need `T: Eq + Hash` or `K: Eq + Hash`, and
  `S: BuildHasher`. `Collection` and `CollectionAlloc` create the hasher and
  thus also need `S: Default`.
- `Vec<T, A>` and the hashbrown collections accept any
  `A: core::alloc::Allocator`.
- `heapless::BinaryHeap<T, K, N>` needs `T: Ord` and `K: Kind`, and
  `heapless::LinearMap<K, V, N>` needs `K: Eq`. `heapless::Vec` and
  `heapless::String` are supported with any length type.

Implementation details worth knowing:

- `VecDeque::push` and `heapless::Deque::push` append to the back of the queue
  (`push_back`).
- `LinkedList::push` appends to the back; `BinaryHeap::push` maintains the
  heap ordering.
- `item_count()` of `String` and `heapless::String` counts UTF-8 `char`s in
  O(n); `byte_len()` and the inherent `len()` count UTF-8 bytes in O(1).
- `BTreeSet`, `BTreeMap` and `LinkedList` can't preallocate memory, and the
  capacity of the `heapless` collections is fixed, so `with_capacity` ignores
  its argument and creates an empty collection.
- `CollectionAlloc` supports `Vec<T, A>` and hashbrown collections with
  `core::alloc::Allocator`; enabling `hashbrown` also enables allocator-api2's
  `nightly` compatibility feature for the whole dependency graph. Crates that
  directly use allocator-api2 may be affected because polyfill-only APIs are
  unavailable in this mode.
- `IndexMap::remove` preserves the order of the remaining elements
  (`shift_remove`), which takes linear time. `heapless::IndexMap::remove`
  moves the last element into the freed position instead (`swap_remove`).
- `CollectionEntry` is separate from `CollectionMap`; `BTreeMap`, standard
  maps, hashbrown maps with any allocator, `IndexMap` and `heapless::IndexMap`
  implement both. The entry methods operate on each map's native entry type
  through `MapEntry`.
- Infallible insertion into a full `heapless` collection (`push`,
  `CollectionMap::insert`, the inserting methods of `MapEntry`) panics with
  `fixed-capacity collection is full`; `CollectionCapacity::try_push` returns
  the item instead. `extend()` and `collect()` are the implementations of
  `heapless`, which panic as well when the capacity is exceeded.
- `heapless::String`, `Deque`, `BinaryHeap` and `LinearMap` do not implement
  `Collection`, because `heapless` provides no `Extend` or `FromIterator` for
  them. `heapless::IndexSet` is not a `CollectionSet`, because it has no `get`,
  `replace` and `take`.

## Macros

The macros create collections in the same manner as `vec!` creates a `Vec`.
They require the `alloc` feature; `map!` and `set!` require `std`.

| Macro                              | Creates      | Available    |
|------------------------------------|--------------|--------------|
| `s!("literal")`                    | `String`     | with `alloc` |
| `map! { key => value, ... }`       | `HashMap`    | with `std`   |
| `set![value, ...]`                 | `HashSet`    | with `std`   |
| `bmap! { key => value, ... }`      | `BTreeMap`   | with `alloc` |
| `bset![value, ...]`                | `BTreeSet`   | with `alloc` |
| `list! { value => value => ... }`  | `LinkedList` | with `alloc` |
| `deque! { value => value => ... }` | `VecDeque`   | with `alloc` |
| `heap![value, ...]`                | `BinaryHeap` | with `alloc` |

`map!`, `set!`, `deque!` and `heap!` reserve the capacity for the listed items
before inserting them, so the collection is allocated once.

Each collection macro has three forms: an empty one, a list of expressions,
and a list prefixed with `owned:`, which calls `to_owned()` on each of the keys
and values:

```rust
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};

use collections::{deque, heap, map, s, set};

let map = map! {
    s!("key") => 5,
    s!("other_key") => 10,
};
assert_eq!(map["key"], 5);

// `owned:` converts `&str` keys and values into `String`s
let owned: HashMap<String, String> = map! { owned: "key" => "value" };
assert_eq!(owned["key"], "value");

// Repeated values are stored once
assert_eq!(set![1, 2, 3, 1], set![1, 2, 3]);

let empty: HashSet<u8> = set![];
assert!(empty.is_empty());

let deque: VecDeque<_> = deque! { "first" => "second" };
assert_eq!(deque.front(), Some(&"first"));

let heap: BinaryHeap<_> = heap![1, 3, 2];
assert_eq!(heap.peek(), Some(&3));
```

The `alloc` macros also work when `collections` is used without its `std`
feature (`default-features = false, features = ["alloc"]`):

```rust
use collections::{bmap, bset, deque, heap, list, s};

let text = s!("a string");
let map = bmap! { "key" => 5, "other_key" => 10 };
let set = bset![5, 6, 7];
let list = list! { "item one" => "item two" => "item three" };
let deque = deque! { "item one" => "item two" => "item three" };
let heap = heap![3, 1, 2];
assert_eq!(text, "a string");
assert_eq!(map.len(), 2);
assert_eq!(set.len(), 3);
assert_eq!(list.len(), 3);
assert_eq!(deque.len(), 3);
assert_eq!(heap.peek(), Some(&3));
```

The macros may also be imported in the old style, with
`#[macro_use] extern crate collections;`.

## Platform support

The crate has no platform-specific code, so what is available on a target is
defined by the features, that is by the `alloc` crate and the dependencies:

| Target                                     | Traits | `heapless` | `alloc` | `indexmap` | `hashbrown` | `ascii` | `std` | Checked in CI by                |
|--------------------------------------------|:------:|:----------:|:-------:|:----------:|:-----------:|:-------:|:-----:|---------------------------------|
| Linux, macOS, Windows (x86-64 and ARM64)    | yes    | yes        | yes     | yes        | yes         | yes     | yes   | running the tests               |
| `wasm32-unknown-unknown`                    | yes    | yes        | yes     | yes        | yes         | yes     | yes   | running the contract under Node |
| `thumbv7m-none-eabi`, `thumbv7em-none-eabi` | yes    | yes        | yes     | yes        | yes         | yes     |       | linking and running firmware    |
| `thumbv6m-none-eabi`                        | yes    | yes        | yes     | yes        | yes         | no      |       | type checking                   |
| `riscv32imc-unknown-none-elf`               | yes    | yes        | yes     | yes        | yes         | no      |       | type checking                   |

- Without the `alloc` feature the crate needs no heap and no global allocator.
  The fixture in `tests/embedded` is firmware for a Cortex-M3: its `noheap`
  binary uses the `heapless` collections through the traits and is linked
  without an allocator, and its `heap` binary uses the `alloc` collections and
  a hashbrown map on a heap of 16 KiB. CI links both and runs them under
  QEMU.
- With the `alloc` feature a `no_std` program has to provide a global
  allocator. A small heap can be exhausted, so use `try_reserve` and
  `try_push` there to handle the failure instead of aborting.
- The `ascii` crate needs `alloc::sync::Arc`, which does not exist on the
  targets without atomic compare-and-swap, like `thumbv6m-none-eabi` and
  `riscv32imc-unknown-none-elf`. The other features build there.
- The fixture in `tests/wasm_consumer` is a WebAssembly module without imports.
  CI executes its checks of the `std`, hashbrown, indexmap and `heapless`
  implementations with Node, and verifies that a failing check is detected.

## Security considerations

The crate adds no logic of its own to the collections: every trait method
forwards to a method of the collection. Its code contains no `unsafe`, which
is enforced with `#![forbid(unsafe_code)]`, so memory safety rests on the
standard library and on the optional dependencies. What remains is how generic
code uses the collections with data it does not control.

### Sizes from untrusted input

`Collection::with_capacity`, `CollectionAlloc::with_capacity_in` and
`CollectionCapacity::reserve` forward to the infallible methods of the
collection. With a size which the collection cannot allocate they panic
(`capacity overflow`) or abort the process, so a length field read from a
file or from the network must not be passed to them: one message can then
terminate the program.

- Create the collection with `C::default()` and use `try_reserve`, which
  returns an error instead.
- Check the size against a limit of the application first. A successful
  `try_reserve` still claims the memory it was asked for.
- Use `try_push` instead of `push` where an allocation failure has to be
  survived, for example on a small heap of a microcontroller. `push`,
  `extend()` and `collect()` are infallible and abort when memory runs out.
- A full fixed-capacity collection makes `push` panic. In firmware a panic
  usually halts or resets the device, so use `try_push` for data which depends
  on input.
- `item_count()` walks the whole string. Calling it for every `char` appended
  to a `String` or a `heapless::String` takes quadratic time; limit strings by
  `byte_len()`, which is O(1).

### Hash flooding

A hash map or a hash set whose keys are chosen by an attacker degrades to
linear time per operation if the attacker can predict the hashes. The default
hashers make that hard with a seed which differs between processes, but only
where the platform gives them something to derive the seed from:

| Platform                 | `std` `RandomState`              | hashbrown's default hasher                          |
|--------------------------|----------------------------------|-----------------------------------------------------|
| Linux, macOS, Windows    | random seed in every process     | seed differs between processes                      |
| `wasm32-unknown-unknown` | **constant seed**                | **constant seed**                                   |
| bare metal (`no_std`)    | not available                    | **predictable seed**: derived from memory addresses |

- On `wasm32-unknown-unknown` a module has no imports, hence no access to the
  randomness of its host: the same key has the same hash in every run. This
  was observed for both hashers by running one module in separate Node
  processes, while the hashes of the native build differed in every process.
- The default hasher of hashbrown (`foldhash`) takes its seed from the
  addresses of the stack and of its own code and data. An operating system
  randomises them; firmware is loaded at the same addresses on every start.
  On the targets without atomic compare-and-swap the seed is a constant. This
  is how `foldhash 0.2.0` is written; it was not measured on a device. On any
  platform, its documentation does not claim resistance to hash flooding by an
  interactive attacker.
- The `Fnv` hashers of `heapless::IndexMap` and `IndexSet` are not seeded at
  all.

For keys which come from untrusted input on these platforms:

- use `BTreeMap` and `BTreeSet`, which do not hash and work through the same
  traits, or
- pass the hasher explicitly, seeded from a random number generator of the
  platform. All hashed collections are supported with any `S: BuildHasher`;
  only the constructors of `Collection` and `CollectionAlloc` need
  `S: Default`.

### Dependencies and the build

- All dependencies are optional, and the crate without features has none.
- Enabling `hashbrown` turns on the `nightly` feature of `allocator-api2`,
  which replaces its `Allocator` trait with the one from `core` for the whole
  build. Other crates in the dependency graph which use `allocator-api2` see
  that change as well.
- CI runs `cargo deny check` on every push and pull request and once a week.
  It fails on an advisory of the [RustSec] database, on a yanked or
  unmaintained dependency, on a licence outside the allow-list, on two
  versions of one crate, and on a source other than crates.io. When this
  section was written, it reported nothing for the versions in `Cargo.lock`.
- CI runs the tests under [Miri] to detect undefined behaviour in the code of
  the dependencies which the tests execute.
- The workflows have read-only access to the repository, and the actions they
  use are pinned to commit hashes.

See [SECURITY.md](./SECURITY.md) for what is considered a vulnerability of this
crate and how to report it.

## Known limitations

- `item_count()` is O(n) for `String` and `heapless::String`, since it counts
  `char`s. Use `byte_len()` where a byte length is enough.
- Capacities and `item_count()` use different units for strings: `reserve(n)`
  reserves `n` bytes, which may hold fewer than `n` `char`s.
- `CollectionMap` uses the map's concrete key type for lookups, so it does not
  provide generic borrowed-key lookup such as `&str` for a `String` key.
- `CollectionAlloc` is implemented for `Vec` and the hashbrown collections
  only: the allocator-aware constructors of the other standard collections
  are not stable yet.
- `BTreeSet`, `BTreeMap`, `LinkedList` and `AsciiString` do not implement
  `CollectionCapacity`, so there is no fallible insertion for them.
- With `indexmap` and without `std`, the dependency has no default hasher type;
  specify the hasher parameter explicitly. It must implement `BuildHasher + Default`.
- The traits intentionally expose only common collection operations; APIs such
  as indexed sequence access, set algebra and heap priority operations stay on
  the native collection types.

## Minimum supported Rust version

The MSRV is **1.100.0**, which stabilizes allocator support for `Vec<T, A>`;
Rust edition 2024 alone requires Rust 1.85.0.

CI checks the library with the MSRV, stable, beta and nightly toolchains, and
on the targets listed in [Platform support](#platform-support).

## Project layout

```text
src/
├── lib.rs          crate root, feature gates and re-exports
├── traits.rs       the collection traits
├── linear.rs       `String`, `Vec`, `VecDeque` and `LinkedList` (`alloc`)
├── heap.rs         `BinaryHeap` (`alloc`)
├── btrees.rs       `BTreeSet` and `BTreeMap` (`alloc`)
├── hashed.rs       `HashSet` and `HashMap` (`std`)
├── hashbrowns.rs   `hashbrown::HashSet` and `hashbrown::HashMap` (`hashbrown`)
├── asciistr.rs     `AsciiString` (`ascii`)
├── indexed.rs      `IndexSet` and `IndexMap` (`indexmap`)
├── fixed.rs        fixed-capacity collections (`heapless`)
└── macros.rs       collection construction macros (`alloc`)

tests/
├── alloc.rs        allocator-aware collection behavior
├── capacity.rs     capacity control and fallible insertion
├── collection.rs   `Collection` implementations, item and byte lengths
├── fixed.rs        fixed-capacity collections
├── keyed.rs        map APIs, entry methods and key bounds
├── set.rs          set APIs and set ordering behavior
├── macros.rs       macro forms
├── common/         the allocator shared by the tests
├── no_std_consumer/ a `#![no_std]` downstream macro consumer
├── embedded/       Cortex-M firmware linked with and without a heap
└── wasm_consumer/  WebAssembly module with contract checks run by Node
```

## Development

```console
# Build representative feature combinations
cargo check --locked
cargo check --locked --no-default-features
cargo check --locked --no-default-features --features alloc
cargo check --locked --no-default-features --features heapless
cargo check --locked --all-features

# Build every feature combination, as CI does (needs `cargo-hack`)
cargo hack check --locked --all-targets --feature-powerset --exclude-features default,all

# Run tests with and without `std` and `alloc`
cargo test --locked
cargo test --locked --no-default-features
cargo test --locked --no-default-features --features alloc
cargo test --locked --no-default-features --features heapless
cargo test --locked --all-features
cargo check --manifest-path tests/no_std_consumer/Cargo.toml

# Bare metal: check a target, then link the firmware without and with a heap
cargo check --locked --target thumbv6m-none-eabi --no-default-features --features heapless
cd tests/embedded
cargo build --locked --release --target thumbv7em-none-eabi --bin noheap
cargo build --locked --release --target thumbv7em-none-eabi --bin heap --features heap
# Run the firmware (needs `qemu-system-arm` and the `thumbv7m-none-eabi` target)
cargo run --locked --release --bin noheap
cargo run --locked --release --bin heap --features heap
cd ../..

# WebAssembly: build the fixture and execute its checks with Node
cd tests/wasm_consumer
cargo build --release --target wasm32-unknown-unknown
node run.mjs target/wasm32-unknown-unknown/release/collections_wasm_consumer.wasm
cd ../..

# Lint, format and build docs (formatting uses nightly options from `.rustfmt.toml`)
cargo clippy --locked --all-features --all-targets -- -D warnings
cargo clippy --locked --no-default-features --all-targets -- -D warnings
cargo +nightly fmt --all -- --check
RUSTDOCFLAGS="--cfg docsrs -D warnings" cargo +nightly doc --locked --all-features --no-deps

# Audit the dependencies (needs `cargo-deny`) and run the tests under Miri
cargo deny --locked check
cargo +nightly miri test --locked --all-features
```

## Contributing

Contributions are welcome. Please read the
[contributing guidelines](./CONTRIBUTING.md) and the
[code of conduct](./CODE_OF_CONDUCT.md) first; all commits must be signed off
according to the [DCO](./DCO) (`git commit -s`).

Security vulnerabilities should be reported privately, as described in
[SECURITY.md](./SECURITY.md). The list of maintainers and their keys is kept in
[MAINTAINERS.md](./MAINTAINERS.md).

## License

The library is distributed under the [Apache 2.0 license](./LICENSE).

[`amplify`]: https://github.com/rust-amplify/rust-amplify
[`ascii`]: https://crates.io/crates/ascii
[`hashbrown`]: https://crates.io/crates/hashbrown
[`heapless`]: https://crates.io/crates/heapless
[`indexmap`]: https://crates.io/crates/indexmap
[Miri]: https://github.com/rust-lang/miri
[RustSec]: https://rustsec.org
