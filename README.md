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

- two traits, [`Collection`](#collection) and
  [`KeyedCollection`](#keyedcollection);
- their implementations for the collections from `alloc` and `std`, and
  optionally for the types from the [`ascii`] and [`indexmap`] crates;
- a few [convenience macros](#macros) for constructing collections, similar to
  `vec!`.

It supports `no_std` environments (with `alloc`), and all its dependencies are
optional (except `wasm-bindgen`, which is used on `wasm32` targets only).

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
| `fn with_capacity(usize) -> Self` | Creates a new collection with a certain capacity.                  |
| `fn len(&self) -> usize`          | Returns the number of elements.                                    |
| `fn is_empty(&self) -> bool`      | Detects whether the collection is empty (has a default implementation). |
| `fn push(&mut self, Self::Item)`  | Pushes or inserts an element into the collection.                  |
| `fn clear(&mut self)`             | Removes all elements.                                              |

The meaning of `push` depends on the collection: it appends an element to the
end of a sequence, and inserts an element into a set or a map. Sets and maps
keep their usual semantics, so pushing a value (or a key) which is already
present does not increase the length:

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
| `type Key: Eq + Hash`                            | Key type.                                                            |
| `type Value`                                     | Value type.                                                          |
| `type Entry<'a>`                                 | Entry type returned by `entry` (the native entry type of the map).   |
| `fn contains_key(&self, &Key) -> bool`           | Checks whether the key is present.                                   |
| `fn get(&self, &Key) -> Option<&Value>`          | Gets a value by its key.                                             |
| `fn get_mut(&mut self, &Key) -> Option<&mut Value>` | Gets a mutable value by its key.                                  |
| `fn iter(&self)`                                 | Iterates over `(&Key, &Value)` pairs.                                |
| `fn iter_mut(&mut self)`                         | Iterates over `(&Key, &mut Value)` pairs.                            |
| `fn values_mut(&mut self)`                       | Iterates over mutable values.                                        |
| `fn insert(&mut self, Key, Value) -> Option<Value>` | Inserts a value, returning the previous value stored under the key. |
| `fn remove(&mut self, &Key) -> Option<Value>`    | Removes a value by its key and returns it.                           |
| `fn entry(&mut self, Key) -> Self::Entry<'_>`    | Gets the entry for in-place manipulation.                            |
| `fn retain(&mut self, impl FnMut(&Key, &mut Value) -> bool)` | Keeps only the pairs for which the predicate returns `true`. |

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

## Supported collection types

| Type           | Bounds             | `Item`      | `Collection` | `KeyedCollection` | Required feature |
|----------------|--------------------|-------------|:------------:|:-----------------:|------------------|
| `String`       |                    | `char`      |     yes      |                   |                  |
| `Vec<T>`       |                    | `T`         |     yes      |                   |                  |
| `VecDeque<T>`  |                    | `T`         |     yes      |                   |                  |
| `BTreeSet<T>`  | `T: Ord`           | `T`         |     yes      |                   |                  |
| `BTreeMap<K, V>` | `K: Ord + Hash`  | `(K, V)`    |     yes      |        yes        |                  |
| `HashSet<T>`   | `T: Eq + Hash`     | `T`         |     yes      |                   | `std`            |
| `HashMap<K, V>` | `K: Eq + Hash`    | `(K, V)`    |     yes      |        yes        | `std`            |
| `AsciiString`  |                    | `AsciiChar` |     yes      |                   | `ascii`          |
| `IndexSet<T>`  | `T: Eq + Hash`     | `T`         |     yes      |                   | `indexmap`       |
| `IndexMap<K, V>` | `K: Eq + Hash`   | `(K, V)`    |     yes      |        yes        | `indexmap`       |

Implementation details worth knowing:

- `VecDeque::push` appends to the back of the queue (`push_back`).
- `BTreeSet` and `BTreeMap` can't preallocate memory, so `with_capacity`
  ignores its argument and creates an empty collection.
- `IndexMap::remove` preserves the order of the remaining elements
  (`shift_remove`), which takes linear time.
- `entry` returns the native entry type of the map: `btree_map::Entry`,
  `hash_map::Entry` or `indexmap::map::Entry`.

## Macros

The macros create collections in the same manner as `vec!` creates a `Vec`.
Which macros are present depends on the `std` feature:

| Macro                        | Creates                | Available              |
|------------------------------|------------------------|------------------------|
| `s!("literal")`              | `String`               | always                 |
| `map! { key => value, ... }` | `HashMap`              | with `std` (default)   |
| `set![value, ...]`           | `HashSet`              | with `std` (default)   |
| `bmap! { key => value, ... }` | `BTreeMap`            | only without `std`     |
| `bset![value, ...]`          | `BTreeSet`             | only without `std`     |
| `list! { value => value => ... }` | `LinkedList`      | only without `std`     |

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

The macros of the `no_std` configuration (`default-features = false`):

```rust,ignore
use collections::{bmap, bset, list};

let map = bmap! { "key" => 5, "other_key" => 10 };
let set = bset![5, 6, 7];
let list = list! { "item one" => "item two" => "item three" };
```

The macros may also be imported in the old style, with
`#[macro_use] extern crate collections;`.

## Known limitations

The library is young, and the current version has a few rough edges:

- `bmap!`, `bset!` and `list!` exist only when the `std` feature is disabled.
  Cargo unifies features across the dependency tree, so these macros disappear
  if any other crate enables `collections/std`.
- Without `std`, the `s!` macro fails to compile at the place of its use: it
  refers to `alloc::String`, while the type is located at
  `alloc::string::String`.
- The `ascii` and `indexmap` features compile only together with `std`.
- `HashMap`, `HashSet`, `IndexMap` and `IndexSet` are supported only with their
  default hasher.
- `BTreeMap` keys must implement `Hash` in addition to `Ord`, since
  `KeyedCollection` requires `Key: Eq + Hash`.
- `LinkedList` and `BinaryHeap` do not implement `Collection`.
- Inside a generic code nothing can be done with a value returned by `entry`,
  since `KeyedCollection::Entry` has no trait bounds.

## Minimum supported Rust version

The MSRV is **1.75.0**: the traits use `impl Trait` in the return position of
trait methods. The crate is written in Rust edition 2021.

CI checks the library with the MSRV, stable, beta and nightly toolchains on
Linux, macOS and Windows (both x86-64 and ARM), and for the
`wasm32-unknown-unknown` target.

## Project layout

```text
src/
├── lib.rs          crate root: feature gates and re-exports
├── traits.rs       `Collection` and `KeyedCollection` traits
├── linear.rs       implementations for `String`, `Vec` and `VecDeque`
├── btrees.rs       implementations for `BTreeSet` and `BTreeMap`
├── hashed.rs       implementations for `HashSet` and `HashMap` (`std`)
├── asciistr.rs     implementation for `AsciiString` (`ascii`)
├── indexed.rs      implementations for `IndexSet` and `IndexMap` (`indexmap`)
├── macro_std.rs    `s!`, `map!` and `set!` macros (with `std`)
└── macro_alloc.rs  `s!`, `bmap!`, `bset!` and `list!` macros (without `std`)
```

## Development

```console
# Build with the default features, without `std`, and with all features
cargo check
cargo check --no-default-features
cargo check --all-features

# Run the tests (at the moment these are the documentation tests of the macros)
cargo test --all-features

# Lint and format; formatting needs nightly due to the unstable options in `.rustfmt.toml`
cargo +stable clippy --all-features --all-targets -- -D warnings
cargo +nightly fmt --all

# Build the API documentation
cargo doc --all-features --open
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
