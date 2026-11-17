// SPDX-License-Identifier: Apache-2.0
//
// Written in 2022-2026 by Dr. Maxim Orlovsky <orlovsky@ubideco.org>
//
// Copyright 2022-2026 Laboratories for Ubiquitous and Deterministic Computing,
// Institute for Distributed and Cognitive Computing (InDCS), Switzerland.
// All rights reserved.
//
// Copyright (C) 2022-2026 Dr Maxim Orlovsky.
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
/// `item_count()` returns the number of values of `Item` held by the
/// collection. For UTF-8 strings this counts `char`s and is O(n); it is O(1)
/// for other collections. `is_empty()` is O(1) for every implementation.
/// `push()` appends to sequences and linked lists, adds to a `BinaryHeap`,
/// inserts into sets and maps, and appends a `char` to strings. Duplicate set
/// values do not increase the item count; inserting an existing map key
/// replaces its value.
///
/// `push()` is infallible: growable collections abort or panic when the
/// allocation fails, and fixed-capacity collections panic when they are full.
/// Use [`CollectionCapacity::try_push`] to get the item back instead.
///
/// No length method is defined by this trait, so a concrete type's inherent
/// `len()` remains unambiguous. Use [`CollectionBytes`] when the byte length is
/// needed.
///
/// ```compile_fail,E0599
/// use collections::CollectionBase;
///
/// fn length<C: CollectionBase>(collection: &C) -> usize {
///     collection.len()
/// }
/// ```
pub trait CollectionBase {
    /// Item type contained within the collection.
    type Item;

    /// Returns the number of `Item`s in the collection.
    ///
    /// This is O(n) for UTF-8 strings, where it counts `char`s, and O(1) for
    /// all other collection types.
    fn item_count(&self) -> usize;

    /// Detects whether a collection is empty. This is O(1) for every
    /// implementation.
    #[inline]
    fn is_empty(&self) -> bool {
        self.item_count() == 0
    }

    /// Appends or inserts an element into the collection.
    ///
    /// # Panics
    ///
    /// Panics if a fixed-capacity collection is full. Growable collections
    /// panic or abort if the required memory cannot be allocated.
    fn push(&mut self, elem: Self::Item);

    /// Removes all elements from the collection.
    fn clear(&mut self);
}

/// Trait for contiguous byte collections.
///
/// It is implemented for UTF-8 strings and byte buffers, but not for general
/// collections whose items happen to be bytes.
///
/// ```compile_fail,E0277
/// use collections::CollectionBytes;
///
/// fn bytes<C: CollectionBytes>(collection: &C) -> usize {
///     collection.byte_len()
/// }
///
/// let values = vec![1_u32, 2];
/// bytes(&values);
/// ```
pub trait CollectionBytes: CollectionBase {
    /// Returns the number of bytes stored in the collection. This is O(1).
    fn byte_len(&self) -> usize;

    /// Returns the number of bytes added by pushing `item`.
    fn item_byte_len(item: &Self::Item) -> usize;
}

/// Trait implemented by collections with a default constructor and iterator
/// support.
///
/// `with_capacity()` reserves space for the requested number of items for
/// sequences, heaps, sets and maps, or bytes for strings. `BTreeSet`,
/// `BTreeMap` and `LinkedList` cannot reserve capacity and ignore the argument,
/// as do fixed-capacity collections, whose capacity is a part of the type.
///
/// An empty collection is created with `Default::default()`.
pub trait Collection:
    CollectionBase + Default + FromIterator<Self::Item> + Extend<Self::Item>
{
    /// Creates a new collection with the requested capacity where supported.
    ///
    /// The capacity must not come from an untrusted source: growable
    /// collections panic or abort if it cannot be allocated. Create an empty
    /// collection and use [`CollectionCapacity::try_reserve`] instead.
    fn with_capacity(capacity: usize) -> Self;
}

/// Trait implemented by collections that support custom allocators.
///
/// Unlike [`Collection`], this trait does not require `FromIterator`, since
/// allocator-aware collections cannot always be constructed by `collect()`.
pub trait CollectionAlloc: CollectionBase + Extend<Self::Item> + Sized {
    /// Allocator used by the collection.
    type Allocator;

    /// Creates a new collection with the requested capacity and allocator.
    fn with_capacity_in(capacity: usize, allocator: Self::Allocator) -> Self;

    /// Creates an empty collection with the given allocator.
    fn new_in(allocator: Self::Allocator) -> Self {
        Self::with_capacity_in(0, allocator)
    }

    /// Returns a reference to the collection's allocator.
    fn allocator(&self) -> &Self::Allocator;
}

/// Trait implemented by collections that can reserve storage capacity.
///
/// Capacity and reservation amounts use the collection's native storage unit:
/// bytes for strings and items for other collections. A string with a capacity
/// of `n` may therefore hold fewer than `n` `char`s.
///
/// Fixed-capacity collections never allocate: their capacity is constant, and
/// a reservation only checks that the requested room is left.
///
/// # Untrusted sizes
///
/// [`Collection::with_capacity`], [`CollectionAlloc::with_capacity_in`] and
/// [`reserve`](Self::reserve) panic or abort the process when the requested
/// memory cannot be allocated, which makes a size taken from untrusted input a
/// denial-of-service vector. Use [`try_reserve`](Self::try_reserve) and
/// [`try_push`](Self::try_push) for such sizes and data.
pub trait CollectionCapacity: CollectionBase {
    /// Error returned when a reservation cannot be satisfied.
    type ReserveError;

    /// Returns the current capacity in the collection's native storage unit.
    fn capacity(&self) -> usize;

    /// Reserves space for at least `additional` more storage units.
    ///
    /// # Panics
    ///
    /// Panics or aborts if the allocation fails or the new capacity overflows.
    /// Panics if a fixed-capacity collection has less room left.
    fn reserve(&mut self, additional: usize);

    /// Attempts to reserve space for at least `additional` more storage units
    /// without panicking or aborting on failure.
    fn try_reserve(&mut self, additional: usize) -> Result<(), Self::ReserveError>;

    /// Attempts to append or insert an item without panicking or aborting when
    /// there is no room for it.
    ///
    /// Returns the item if the required capacity cannot be reserved. Inserting
    /// an already present set value or map key into a full fixed-capacity
    /// collection succeeds, since it needs no room.
    fn try_push(&mut self, elem: Self::Item) -> Result<(), Self::Item> {
        if self.try_reserve(1).is_err() {
            return Err(elem);
        }
        self.push(elem);
        Ok(())
    }
}

/// Trait implemented by key-value maps.
pub trait CollectionMap: CollectionBase<Item = (Self::Key, Self::Value)> {
    /// Key type for the collection.
    type Key;
    /// Value type for the collection.
    type Value;

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

    /// Retains only the elements specified by the predicate.
    ///
    /// In other words, remove all pairs `(k, v)` for which `f(&k, &mut v)`
    /// returns `false`. The elements are visited in unsorted (and
    /// unspecified) order.
    fn retain(&mut self, f: impl FnMut(&Self::Key, &mut Self::Value) -> bool);
}

/// Trait implemented by set collections.
pub trait CollectionSet: CollectionBase {
    /// Checks whether an equal item is present in the set.
    fn contains(&self, item: &Self::Item) -> bool;

    /// Returns a reference to the stored item equal to the given item.
    fn get(&self, item: &Self::Item) -> Option<&Self::Item>;

    /// Returns an iterator over the set's items.
    fn iter(&self) -> impl Iterator<Item = &Self::Item>;

    /// Inserts an item, returning whether the set did not already contain it.
    fn insert(&mut self, item: Self::Item) -> bool;

    /// Inserts an item or replaces an equal one, returning the previous item if
    /// any.
    fn replace(&mut self, item: Self::Item) -> Option<Self::Item>;

    /// Removes an equal item, returning whether it was present.
    fn remove(&mut self, item: &Self::Item) -> bool;

    /// Removes an equal item and returns the stored item if present.
    fn take(&mut self, item: &Self::Item) -> Option<Self::Item>;

    /// Retains only the items specified by the predicate.
    fn retain(&mut self, f: impl FnMut(&Self::Item) -> bool);
}

/// Trait implemented by maps that expose an in-place entry API.
pub trait CollectionEntry: CollectionMap {
    /// Native entry type used for in-place map operations.
    type Entry<'a>: MapEntry<'a, Key = Self::Key, Value = Self::Value>
    where
        Self: 'a;

    /// Gets the given key's corresponding entry in the map for in-place
    /// manipulation.
    fn entry(&mut self, key: Self::Key) -> Self::Entry<'_>;
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
