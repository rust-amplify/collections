Change Log
==========

## _Unreleased_

### Breaking changes

- The package is renamed to `collections-traits`; the library is still imported
  as `collections`. Depend on it with
  `collections = { package = "collections-traits", ... }`.
- `alloc` is now a Cargo feature, implied by `std`, `ascii`, `indexmap` and
  `hashbrown`. `default-features = false` no longer includes the `alloc`
  collections and macros, and the crate then needs neither the `alloc` crate
  nor a global allocator. Add `features = ["alloc"]` to keep the previous
  behaviour.
- The traits are layered on the new constructor-free root `CollectionBase`,
  which owns `Item`, `item_count`, `is_empty`, `push` and `clear`.
  `Collection`, `CollectionAlloc`, `CollectionSet` and `CollectionMap` extend
  it and no longer declare these items themselves; `CollectionSet` and
  `CollectionMap` no longer require `Collection`.
- No trait declares `len()` any more. `CollectionBase::item_count()` returns the
  number of `Item`s, which is the number of `char`s for strings (counted in
  O(n)), and `CollectionBytes::byte_len()` returns the number of bytes.
- `Collection` requires `Default`.
- `entry` and `Entry<'a>` moved from `CollectionMap` to the new
  `CollectionEntry` trait.
- hashbrown collections are bound on `core::alloc::Allocator` instead of the
  `allocator-api2` trait, the `Clone` bound on their allocator is dropped, and
  `collections::allocator_api2` is no longer re-exported. Enabling the
  `hashbrown` feature turns on the `nightly` feature of `allocator-api2` for the
  whole build, which makes its `Allocator` the one from `core`; crates using
  the polyfill-only APIs of `allocator-api2` may stop compiling.
- Renamed `KeyedCollection` to `CollectionMap`.
- The crate now requires Rust 1.100.0 or later for stable allocator support.

### Added

- `CollectionBase`, the root trait implemented by every supported collection.
- `CollectionBytes` with `byte_len()` and `item_byte_len()` for `String`,
  `Vec<u8, A>`, `AsciiString`, `heapless::String<N>` and
  `heapless::Vec<u8, N>`.
- `CollectionCapacity` with `capacity`, `reserve`, `try_reserve` and the
  fallible `try_push`, which returns the item instead of panicking or aborting
  when there is no room for it. Strings reserve the UTF-8 width of the `char`.
- `CollectionEntry` for maps with an in-place entry API.
- The `heapless` feature, which implements the traits for the fixed-capacity
  `heapless::Vec`, `String`, `Deque`, `BinaryHeap`, `IndexSet`, `IndexMap` and
  `LinearMap`. Infallible insertion into a full collection panics with
  `fixed-capacity collection is full`; `try_push` returns the item.
- hashbrown sets and maps with any allocator implement `CollectionBase`,
  `CollectionSet`, `CollectionMap`, `CollectionEntry` and `CollectionCapacity`;
  `std::alloc::Global`, `System` and custom allocators can be used with them.
- `MapEntry` for generic map entry operations, plus `keys()` and `values()` on
  `CollectionMap`.
- `CollectionSet` for common set operations on `BTreeSet`, `HashSet` and
  `IndexSet`.
- `CollectionAlloc` for allocator-aware `Vec`, `hashbrown::HashMap` and
  `hashbrown::HashSet` collections.
- `deque!` and `heap!` macros for constructing `VecDeque` and `BinaryHeap`.
- Feature-gated `hashbrown` implementations for `HashMap` and `HashSet`.
- `Collection` implementations for `LinkedList` and `BinaryHeap`.
- Generic contract tests for supported collection types and macros.
- A Cortex-M firmware fixture (`tests/embedded`) which is linked with and
  without a heap, and a WebAssembly fixture (`tests/wasm_consumer`) whose
  contract checks are executed with Node.

### Changed

- `map!`, `set!`, `deque!` and `heap!` reserve the capacity for the listed
  items before inserting them.
- Hash-based and index-based collection implementations now accept any hasher
  that implements `BuildHasher`; only the constructors require `Default`.
- `BTreeMap` requires only `Ord` for its keys.
- Collection macros now use a single module and are available consistently
  across feature combinations.
- CI checks every feature combination with `cargo hack`, type-checks the
  `thumbv6m-none-eabi`, `thumbv7em-none-eabi`, `riscv32imc-unknown-none-elf`
  and `wasm32-unknown-unknown` targets, runs the firmware fixture under QEMU
  and the WebAssembly fixture under Node, and runs the tests with `alloc` only
  and with `heapless` only.

### Security

- The crate forbids `unsafe` code with `#![forbid(unsafe_code)]`, and a
  `[lints]` table enforces the lints which the library and its tests satisfy.
- Documented that `with_capacity`, `with_capacity_in` and `reserve` panic or
  abort on a size which cannot be allocated, and that sizes from untrusted
  input have to go through `try_reserve` and `try_push`.
- Documented that the default hashers are not seeded randomly on
  `wasm32-unknown-unknown` and on bare-metal targets, where hash maps and hash
  sets of untrusted keys are open to hash flooding.
- `SECURITY.md` describes what is a vulnerability of this crate.
- Added `deny.toml` and the `Audit` workflow: `cargo deny check` fails on
  RustSec advisories, yanked or unmaintained dependencies, licences outside
  the allow-list, duplicate versions of a crate and sources other than
  crates.io; it runs on every push and pull request and once a week. The tests
  also run under Miri.
- The workflows have read-only permissions and use actions pinned to commit
  hashes, which Dependabot keeps up to date together with the dependencies.
  The lint workflow also runs on pushes to `master`.

### Fixed

- The `hashbrown` feature builds with `hashbrown 0.17`: `allocator-api2` stays
  on the `0.2` line which `hashbrown` requires.
- `C: Collection + CollectionAlloc` no longer makes `push`, `clear`, `is_empty`
  and the length ambiguous.
- `ascii` and `indexmap` build without `std`, docs.rs uses the supported
  `doc_cfg` feature, and README examples are compiled as doctests.
- CI exercises no-`std`, feature, target and documentation configurations.

### Removed

- The unused `wasm-bindgen` dependency and the mutually exclusive macro modules.
