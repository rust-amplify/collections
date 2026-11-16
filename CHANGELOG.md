Change Log
==========

## [Unreleased]

### Added

- `MapEntry` for generic map entry operations, plus `keys()` and `values()` on
  `KeyedCollection`.
- `Collection` implementations for `LinkedList` and `BinaryHeap`.
- Generic contract tests for supported collection types and macros.

### Changed

- Hash-based and index-based collection implementations now accept any hasher
  that implements `BuildHasher + Default`.
- `BTreeMap` requires only `Ord` for its keys.
- Collection macros now use a single module and are available consistently
  across feature combinations.

### Fixed

- `ascii` and `indexmap` build without `std`, docs.rs uses the supported
  `doc_cfg` feature, and README examples are compiled as doctests.
- The crate lockfile is pinned to an MSRV-compatible `indexmap` release; CI
  exercises no-`std`, feature, target and documentation configurations.

### Removed

- The unused `wasm-bindgen` dependency and the mutually exclusive macro modules.
