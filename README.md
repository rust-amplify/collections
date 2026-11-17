# Traits for collections, implemented for standard collections

![Build](https://github.com/rust-amplify/collections/workflows/Build/badge.svg)
![Tests](https://github.com/rust-amplify/collections/workflows/Tests/badge.svg)

[![crates.io](https://img.shields.io/crates/v/collections)](https://crates.io/crates/collections)
[![Docs](https://docs.rs/collections/badge.svg)](https://docs.rs/collections)
[![License](https://img.shields.io/crates/l/collections)](./LICENSE)

`collections` is a small Rust library providing "the missed collection traits":
a common interface over the collection types of the standard library, so that
generic code can be written once and work with a `Vec`, a `HashSet`, a
`BTreeMap` or any other supported collection.

The crate contains:

- three traits, [`Collection`](#collection),
  [`KeyedCollection`](#keyedcollection) and
  [`MapEntry`](#mapentry);
- their implementations for the collections from `alloc` and `std`, and
  optionally for the types from the [`ascii`] and [`indexmap`] crates;
- a few [convenience macros](#macros) for constructing collections, similar to
  `vec!`.

It supports `no_std` environments (with `alloc`). The `ascii` and `indexmap`
integrations are optional features, and the crate has no wasm-specific
dependency.

## Contents

- [Motivation](#motivation)
- [Installation](#installation)
- [Cargo features](#cargo-features)
- [Traits](#traits)
- [Supported collection types](#supported-collection-types)
- [Macros](#macros)
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
collections = { git = "https://github.com/rust-amplify/collections" }
```

Selecting features:

```toml
[dependencies]
# Everything: `std`, `ascii` and `indexmap`
collections = { git = "https://github.com/rust-amplify/collections", features = ["all"] }
```

```toml
[dependencies]
# `no_std` (the `alloc` crate is still required)
collections = { git = "https://github.com/rust-amplify/collections", default-features = false }
```

## Cargo features

| Feature    | Default | Description                                                                                             |
|------------|:-------:|---------------------------------------------------------------------------------------------------------|
| `std`      |   yes   | Uses the standard library: adds implementations for `HashMap` and `HashSet`, and `map!`/`set!` macros.   |
| `ascii`    |   no    | Adds the [`ascii`] dependency and implements `Collection` for `AsciiString`.                             |
| `indexmap` |   no    | Adds the [`indexmap`] dependency and implements the traits for `IndexMap` and `IndexSet`.                |
| `all`      |   no    | Shortcut for `std` + `ascii` + `indexmap`.                                                               |

When the optional dependencies are enabled, they are re-exported as
`collections::ascii` and `collections::indexmap`; the `alloc` crate is always
re-exported as `collections::alloc`.

## Traits

### `Collection`

`Collection` is implemented by all supported types. It requires
`FromIterator<Self::Item>` and `Extend<Self::Item>`, thus a generic code may
also use `collect()` and `extend()`.

| Item                              | Description                                                        |
|-----------------------------------|--------------------------------------------------------------------|
| `type Item`                       | Type of the elements contained in the collection.                  |
| `fn with_capacity(usize) -> Self` | Creates a new collection, reserving capacity where supported.      |
| `fn len(&self) -> usize`          | Returns the number of items; for `String`, the number of bytes.    |
| `fn is_empty(&self) -> bool`      | Detects whether the collection is empty (has a default implementation). |
| `fn push(&mut self, Self::Item)`  | Pushes or inserts an element into the collection.                  |
| `fn clear(&mut self)`             | Removes all elements.                                              |

The meaning of `push` depends on the collection: it appends to sequences and
linked lists, inserts into sets and maps, pushes into a `BinaryHeap`, and adds a
UTF-8 encoded `char` to `String`. `String::len()` counts bytes, so a `char` may
increase its length by one to four. Duplicate set values do not increase the
length, while pushing a map pair with an existing key replaces its value.
`with_capacity` reserves for sequences, heaps, sets and maps; `String` reserves
bytes. `BTreeSet`, `BTreeMap` and `LinkedList` cannot reserve and ignore the
requested capacity.

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

### `KeyedCollection`

`KeyedCollection` extends `Collection` for key-value maps. It is a
`Collection` with `Item = (Self::Key, Self::Value)`.

| Item                                             | Description                                                          |
|--------------------------------------------------|----------------------------------------------------------------------|
| `type Key`                                       | Key type; each map implementation sets its own bounds.              |
| `type Value`                                     | Value type.                                                          |
| `type Entry<'a>: MapEntry<'a>`                   | Native entry type with generic in-place operations.                  |
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
| `fn entry(&mut self, Key) -> Self::Entry<'_>`    | Gets the entry for in-place manipulation.                            |
| `fn retain(&mut self, impl FnMut(&Key, &mut Value) -> bool)` | Keeps only the pairs for which the predicate returns `true`. |

`MapEntry` provides `key`, `or_insert`, `or_insert_with`, `or_default` and
`and_modify` on the native entry types. This keeps the standard map entry APIs
usable in generic code without replacing the native `Entry` type.

The same generic function can be used with different kinds of maps:

```rust
use std::collections::{BTreeMap, HashMap};

use collections::KeyedCollection;

/// Counts the words which are present in the text more than once.
fn repeated_words<'a, M>(text: &'a str) -> M
where M: KeyedCollection<Key = &'a str, Value = usize> {
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

### `MapEntry`

`MapEntry` is the common interface of the entries returned by
`KeyedCollection::entry`. An entry is a view into a single key of a map, which
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
which are the types returned by `KeyedCollection::entry`:

| Map                 | `KeyedCollection::Entry<'a>`     | Required feature |
|---------------------|----------------------------------|------------------|
| `BTreeMap<K, V>`    | `btree_map::Entry<'a, K, V>`     |                  |
| `HashMap<K, V, S>`  | `hash_map::Entry<'a, K, V>`      | `std`            |
| `IndexMap<K, V, S>` | `indexmap::map::Entry<'a, K, V>` | `indexmap`       |

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

use collections::{KeyedCollection, MapEntry};

/// Counts how many times each word occurs in the text.
fn word_counts<'a, M>(text: &'a str) -> M
where M: KeyedCollection<Key = &'a str, Value = usize> {
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

use collections::{KeyedCollection, MapEntry};

fn update<M>(stock: &mut M)
where M: KeyedCollection<Key = &'static str, Value = u32> {
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

| Type             | Bounds                                  | `Item`      | `Collection` | `KeyedCollection` | Required feature |
|------------------|-----------------------------------------|-------------|:------------:|:-----------------:|------------------|
| `String`         |                                         | `char`      |     yes      |                   |                  |
| `Vec<T>`         |                                         | `T`         |     yes      |                   |                  |
| `VecDeque<T>`    |                                         | `T`         |     yes      |                   |                  |
| `LinkedList<T>`  |                                         | `T`         |     yes      |                   |                  |
| `BinaryHeap<T>`  | `T: Ord`                                | `T`         |     yes      |                   |                  |
| `BTreeSet<T>`    | `T: Ord`                                | `T`         |     yes      |                   |                  |
| `BTreeMap<K, V>` | `K: Ord`                                | `(K, V)`    |     yes      |        yes        |                  |
| `HashSet<T, S>`  | `T: Eq + Hash`, `S: BuildHasher + Default` | `T`      |     yes      |                   | `std`            |
| `HashMap<K, V, S>` | `K: Eq + Hash`, `S: BuildHasher + Default` | `(K, V)` | yes       |        yes        | `std`            |
| `AsciiString`    |                                         | `AsciiChar` |     yes      |                   | `ascii`          |
| `IndexSet<T, S>` | `T: Eq + Hash`, `S: BuildHasher + Default` | `T`      |     yes      |                   | `indexmap`       |
| `IndexMap<K, V, S>` | `K: Eq + Hash`, `S: BuildHasher + Default` | `(K, V)` | yes     |        yes        | `indexmap`       |

Implementation details worth knowing:

- `VecDeque::push` appends to the back of the queue (`push_back`).
- `LinkedList::push` appends to the back; `BinaryHeap::push` maintains the
  standard max-heap ordering.
- `String::len()` counts UTF-8 bytes, although its `Collection::Item` is `char`.
- `BTreeSet`, `BTreeMap` and `LinkedList` can't preallocate memory, so
  `with_capacity` ignores its argument and creates an empty collection.
- Hashed collections accept any hasher implementing `BuildHasher + Default`.
- `IndexMap::remove` preserves the order of the remaining elements
  (`shift_remove`), which takes linear time.
- `entry` returns the native entry type of the map: `btree_map::Entry`,
  `hash_map::Entry` or `indexmap::map::Entry`; generic code can use them through
  `MapEntry`.

## Macros

The macros create collections in the same manner as `vec!` creates a `Vec`.
The `alloc` macros are always present; `map!` and `set!` require `std`.

| Macro                        | Creates                | Available              |
|------------------------------|------------------------|------------------------|
| `s!("literal")`              | `String`               | always                 |
| `map! { key => value, ... }` | `HashMap`              | with `std`             |
| `set![value, ...]`           | `HashSet`              | with `std`             |
| `bmap! { key => value, ... }` | `BTreeMap`            | always                 |
| `bset![value, ...]`          | `BTreeSet`             | always                 |
| `list! { value => value => ... }` | `LinkedList`      | always                 |

Each collection macro has three forms: an empty one, a list of expressions,
and a list prefixed with `owned:`, which calls `to_owned()` on each of the keys
and values:

```rust
use std::collections::{HashMap, HashSet};

use collections::{map, s, set};

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
```

The `alloc` macros also work when `collections` is used without its `std`
feature (`default-features = false`):

```rust
use collections::{bmap, bset, list, s};

let text = s!("a string");
let map = bmap! { "key" => 5, "other_key" => 10 };
let set = bset![5, 6, 7];
let list = list! { "item one" => "item two" => "item three" };
assert_eq!(text, "a string");
assert_eq!(map.len(), 2);
assert_eq!(set.len(), 3);
assert_eq!(list.len(), 3);
```

The macros may also be imported in the old style, with
`#[macro_use] extern crate collections;`.

## Known limitations

- `String::len()` reports UTF-8 bytes, not the number of `char` items; this
  matches the standard `String` API.
- `KeyedCollection` uses the map's concrete key type for lookups, so it does not
  provide generic borrowed-key lookup such as `&str` for a `String` key.
- With `indexmap` and without `std`, the dependency has no default hasher type;
  specify the hasher parameter explicitly. It must implement `BuildHasher + Default`.
- The traits intentionally expose only common collection operations; APIs such
  as indexed sequence access, set membership and heap priority operations stay
  on the native collection types.

## Minimum supported Rust version

The MSRV is **1.85.0**, required by Rust edition 2024. The traits also use
`impl Trait` in the return position of trait methods.

CI checks the library with the MSRV, stable, beta and nightly toolchains on
Linux, macOS and Windows (both x86-64 and ARM), and for the
`wasm32-unknown-unknown` target.

## Project layout

```text
src/
├── lib.rs          crate root, feature gates and re-exports
├── traits.rs       `Collection`, `KeyedCollection` and `MapEntry`
├── linear.rs       `String`, `Vec`, `VecDeque` and `LinkedList`
├── heap.rs         `BinaryHeap`
├── btrees.rs       `BTreeSet` and `BTreeMap`
├── hashed.rs       `HashSet` and `HashMap` (`std`)
├── asciistr.rs     `AsciiString` (`ascii`)
├── indexed.rs      `IndexSet` and `IndexMap` (`indexmap`)
└── macros.rs       collection construction macros

tests/
├── collection.rs   `Collection` implementations and behavior
├── keyed.rs        map APIs, entry methods and key bounds
├── macros.rs       macro forms
└── no_std_consumer/ a `#![no_std]` downstream macro consumer
```

## Development

```console
# Build representative feature combinations
cargo check --locked
cargo check --locked --no-default-features
cargo check --locked --no-default-features --features ascii,indexmap
cargo check --locked --all-features

# Run tests with and without `std`
cargo test --locked
cargo test --locked --no-default-features
cargo test --locked --all-features
cargo check --manifest-path tests/no_std_consumer/Cargo.toml

# Lint, format and build docs (formatting uses nightly options from `.rustfmt.toml`)
cargo clippy --locked --all-features --all-targets -- -D warnings
cargo clippy --locked --no-default-features --all-targets -- -D warnings
cargo +nightly fmt --all -- --check
RUSTDOCFLAGS="--cfg docsrs -D warnings" cargo +nightly doc --locked --all-features --no-deps

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
[`indexmap`]: https://crates.io/crates/indexmap
