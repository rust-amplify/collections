// SPDX-License-Identifier: Apache-2.0
//
// Written in 2022-2026 by Dr. Maxim Orlovsky <orlovsky@ubideco.org>
//
// Copyright 2022-2026 Laboratories for Ubiquitous and Deterministic Computing,
// Institute for Distributed and Cognitive Computing (InDCS), Switzerland.
// All rights reserved.
//
// Copyright (C) 2022-2025 Dr Maxim Orlovsky.
// All rights reserved.
//
// Licensed under the Apache License, Version 2.0 (the "License"); you may not
// use this file except in compliance with the License. You may obtain a copy of
// the License at <http://www.apache.org/licenses/LICENSE-2.0>
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS, WITHOUT
// WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied. See the
// License for the specific language governing permissions and limitations under
// the License.

/// Trait implemented by all collection types.
///
/// # Collection contract
///
/// `len()` reports the number of items for collections and the number of bytes
/// for `String`. `push()` appends to sequences and linked lists, adds to a
/// `BinaryHeap`, inserts into sets and maps, and appends a UTF-8 encoded `char`
/// to `String`. Duplicate set values do not increase the length; inserting an
/// existing map key replaces its value.
///
/// `with_capacity()` reserves space for the requested number of items for
/// sequences, heaps, sets and maps, or bytes for `String`. `BTreeSet`,
/// `BTreeMap` and `LinkedList` cannot reserve capacity and ignore the argument.
pub trait Collection: FromIterator<Self::Item> + Extend<Self::Item> {
    /// Item type contained within the collection.
    type Item;

    /// Creates a new collection with the requested capacity where supported.
    fn with_capacity(capacity: usize) -> Self;

    /// Returns the number of items, or the number of bytes for `String`.
    fn len(&self) -> usize;

    /// Detects whether a collection is empty.
    #[inline]
    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Appends or inserts an element into the collection.
    fn push(&mut self, elem: Self::Item);

    /// Removes all elements from the collection.
    fn clear(&mut self);
}

/// Trait implemented by key-value maps.
pub trait KeyedCollection: Collection<Item = (Self::Key, Self::Value)> {
    /// Key type for the collection.
    type Key;
    /// Value type for the collection.
    type Value;
    /// Native entry type used for in-place map operations.
    type Entry<'a>: MapEntry<'a, Key = Self::Key, Value = Self::Value>
    where
        Self: 'a;

    /// Checks whether a given key is contained in the collection.
    fn contains_key(&self, key: &Self::Key) -> bool;

    /// Gets a value of the collection.
    fn get(&self, key: &Self::Key) -> Option<&Self::Value>;

    /// Gets a mutable value of the collection.
    fn get_mut(&mut self, key: &Self::Key) -> Option<&mut Self::Value>;

    /// Returns iterator over keys and values.
    fn iter(&self) -> impl Iterator<Item = (&Self::Key, &Self::Value)>;

    /// Returns an iterator over the keys.
    fn keys(&self) -> impl Iterator<Item = &Self::Key> {
        self.iter().map(|(key, _)| key)
    }

    /// Returns an iterator over the values.
    fn values(&self) -> impl Iterator<Item = &Self::Value> {
        self.iter().map(|(_, value)| value)
    }

    /// Returns iterator over keys and mutable values.
    fn iter_mut(&mut self) -> impl Iterator<Item = (&Self::Key, &mut Self::Value)>;

    /// Constructs iterator over mutable values.
    fn values_mut(&mut self) -> impl Iterator<Item = &mut Self::Value>;

    /// Inserts a new value under a key. Returns previous value if a value under
    /// the key was already present in the collection.
    fn insert(&mut self, key: Self::Key, value: Self::Value) -> Option<Self::Value>;

    /// Removes a value stored under a given key, returning an owned value if
    /// it was in the collection.
    fn remove(&mut self, key: &Self::Key) -> Option<Self::Value>;

    /// Gets the given key's corresponding entry in the map for in-place
    /// manipulation.
    fn entry(&mut self, key: Self::Key) -> Self::Entry<'_>;

    /// Retains only the elements specified by the predicate.
    ///
    /// In other words, remove all pairs `(k, v)` for which `f(&k, &mut v)`
    /// returns `false`. The elements are visited in unsorted (and
    /// unspecified) order.
    fn retain(&mut self, f: impl FnMut(&Self::Key, &mut Self::Value) -> bool);
}

/// Common interface over the native entry types of maps.
pub trait MapEntry<'a>: Sized {
    /// Key type stored in the map.
    type Key;
    /// Value type stored in the map.
    type Value: 'a;

    /// Returns the key associated with this entry.
    fn key(&self) -> &Self::Key;

    /// Inserts the provided value if the entry is vacant, then returns it.
    fn or_insert(self, default: Self::Value) -> &'a mut Self::Value;

    /// Inserts a value produced by `default` if the entry is vacant.
    fn or_insert_with<F: FnOnce() -> Self::Value>(self, default: F) -> &'a mut Self::Value;

    /// Inserts the default value if the entry is vacant.
    fn or_default(self) -> &'a mut Self::Value
    where
        Self::Value: Default;

    /// Applies `f` to the value if the entry is occupied.
    fn and_modify<F: FnOnce(&mut Self::Value)>(self, f: F) -> Self;
}
