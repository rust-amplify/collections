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

use core::hash::{BuildHasher, Hash};

use heapless::binary_heap::Kind;
use heapless::{
    BinaryHeap, CapacityError, Deque, IndexMap, IndexSet, LenType, LinearMap, String, Vec,
    index_map,
};

use super::{
    Collection, CollectionBase, CollectionBytes, CollectionCapacity, CollectionEntry,
    CollectionMap, MapEntry,
};

/// Reports an infallible insertion into a full fixed-capacity collection.
#[cold]
#[inline(never)]
#[track_caller]
fn capacity_exceeded() -> ! {
    panic!("fixed-capacity collection is full")
}

/// Checks whether `additional` more storage units fit into the capacity.
fn check_room(len: usize, capacity: usize, additional: usize) -> Result<(), CapacityError> {
    match len.checked_add(additional) {
        Some(required) if required <= capacity => Ok(()),
        _ => Err(CapacityError::default()),
    }
}

impl<T, LenT: LenType, const N: usize> CollectionBase for Vec<T, N, LenT> {
    type Item = T;

    fn item_count(&self) -> usize {
        self.as_slice().len()
    }

    #[track_caller]
    fn push(&mut self, elem: Self::Item) {
        if Vec::push(self, elem).is_err() {
            capacity_exceeded()
        }
    }

    fn clear(&mut self) {
        Vec::clear(self)
    }
}

impl<LenT: LenType, const N: usize> CollectionBytes for Vec<u8, N, LenT> {
    fn byte_len(&self) -> usize {
        self.as_slice().len()
    }

    fn item_byte_len(_item: &Self::Item) -> usize {
        1
    }
}

impl<T, LenT: LenType, const N: usize> Collection for Vec<T, N, LenT> {
    /// The capacity is fixed by the type, so this argument is ignored.
    fn with_capacity(_capacity: usize) -> Self {
        Vec::new()
    }
}

impl<T, LenT: LenType, const N: usize> CollectionCapacity for Vec<T, N, LenT> {
    type ReserveError = CapacityError;

    fn capacity(&self) -> usize {
        Vec::capacity(self)
    }

    #[track_caller]
    fn reserve(&mut self, additional: usize) {
        if check_room(self.as_slice().len(), Vec::capacity(self), additional).is_err() {
            capacity_exceeded()
        }
    }

    fn try_reserve(&mut self, additional: usize) -> Result<(), Self::ReserveError> {
        check_room(self.as_slice().len(), Vec::capacity(self), additional)
    }

    fn try_push(&mut self, elem: Self::Item) -> Result<(), Self::Item> {
        Vec::push(self, elem)
    }
}

impl<LenT: LenType, const N: usize> CollectionBase for String<N, LenT> {
    type Item = char;

    fn item_count(&self) -> usize {
        self.as_str().chars().count()
    }

    fn is_empty(&self) -> bool {
        self.as_str().is_empty()
    }

    #[track_caller]
    fn push(&mut self, elem: Self::Item) {
        if String::push(self, elem).is_err() {
            capacity_exceeded()
        }
    }

    fn clear(&mut self) {
        String::clear(self)
    }
}

impl<LenT: LenType, const N: usize> CollectionBytes for String<N, LenT> {
    fn byte_len(&self) -> usize {
        self.as_str().len()
    }

    fn item_byte_len(item: &Self::Item) -> usize {
        item.len_utf8()
    }
}

impl<LenT: LenType, const N: usize> CollectionCapacity for String<N, LenT> {
    type ReserveError = CapacityError;

    fn capacity(&self) -> usize {
        String::capacity(self)
    }

    #[track_caller]
    fn reserve(&mut self, additional: usize) {
        if check_room(self.as_str().len(), String::capacity(self), additional).is_err() {
            capacity_exceeded()
        }
    }

    fn try_reserve(&mut self, additional: usize) -> Result<(), Self::ReserveError> {
        check_room(self.as_str().len(), String::capacity(self), additional)
    }

    fn try_push(&mut self, elem: Self::Item) -> Result<(), Self::Item> {
        String::push(self, elem).map_err(|_| elem)
    }
}

impl<T, const N: usize> CollectionBase for Deque<T, N> {
    type Item = T;

    fn item_count(&self) -> usize {
        Deque::len(self)
    }

    #[track_caller]
    fn push(&mut self, elem: Self::Item) {
        if Deque::push_back(self, elem).is_err() {
            capacity_exceeded()
        }
    }

    fn clear(&mut self) {
        Deque::clear(self)
    }
}

impl<T, const N: usize> CollectionCapacity for Deque<T, N> {
    type ReserveError = CapacityError;

    fn capacity(&self) -> usize {
        Deque::capacity(self)
    }

    #[track_caller]
    fn reserve(&mut self, additional: usize) {
        if check_room(Deque::len(self), Deque::capacity(self), additional).is_err() {
            capacity_exceeded()
        }
    }

    fn try_reserve(&mut self, additional: usize) -> Result<(), Self::ReserveError> {
        check_room(Deque::len(self), Deque::capacity(self), additional)
    }

    fn try_push(&mut self, elem: Self::Item) -> Result<(), Self::Item> {
        Deque::push_back(self, elem)
    }
}

impl<T: Ord, K: Kind, const N: usize> CollectionBase for BinaryHeap<T, K, N> {
    type Item = T;

    fn item_count(&self) -> usize {
        BinaryHeap::len(self)
    }

    #[track_caller]
    fn push(&mut self, elem: Self::Item) {
        if BinaryHeap::push(self, elem).is_err() {
            capacity_exceeded()
        }
    }

    fn clear(&mut self) {
        BinaryHeap::clear(self)
    }
}

impl<T: Ord, K: Kind, const N: usize> CollectionCapacity for BinaryHeap<T, K, N> {
    type ReserveError = CapacityError;

    fn capacity(&self) -> usize {
        BinaryHeap::capacity(self)
    }

    #[track_caller]
    fn reserve(&mut self, additional: usize) {
        if check_room(
            BinaryHeap::len(self),
            BinaryHeap::capacity(self),
            additional,
        )
        .is_err()
        {
            capacity_exceeded()
        }
    }

    fn try_reserve(&mut self, additional: usize) -> Result<(), Self::ReserveError> {
        check_room(
            BinaryHeap::len(self),
            BinaryHeap::capacity(self),
            additional,
        )
    }

    fn try_push(&mut self, elem: Self::Item) -> Result<(), Self::Item> {
        BinaryHeap::push(self, elem)
    }
}

impl<T: Eq + Hash, S: BuildHasher, const N: usize> CollectionBase for IndexSet<T, S, N> {
    type Item = T;

    fn item_count(&self) -> usize {
        IndexSet::len(self)
    }

    #[track_caller]
    fn push(&mut self, elem: Self::Item) {
        if IndexSet::insert(self, elem).is_err() {
            capacity_exceeded()
        }
    }

    fn clear(&mut self) {
        IndexSet::clear(self)
    }
}

impl<T: Eq + Hash, S: BuildHasher + Default, const N: usize> Collection for IndexSet<T, S, N> {
    /// The capacity is fixed by the type, so this argument is ignored.
    fn with_capacity(_capacity: usize) -> Self {
        IndexSet::default()
    }
}

impl<T: Eq + Hash, S: BuildHasher, const N: usize> CollectionCapacity for IndexSet<T, S, N> {
    type ReserveError = CapacityError;

    fn capacity(&self) -> usize {
        IndexSet::capacity(self)
    }

    #[track_caller]
    fn reserve(&mut self, additional: usize) {
        if check_room(IndexSet::len(self), IndexSet::capacity(self), additional).is_err() {
            capacity_exceeded()
        }
    }

    fn try_reserve(&mut self, additional: usize) -> Result<(), Self::ReserveError> {
        check_room(IndexSet::len(self), IndexSet::capacity(self), additional)
    }

    fn try_push(&mut self, elem: Self::Item) -> Result<(), Self::Item> {
        IndexSet::insert(self, elem).map(|_| ())
    }
}

impl<K: Eq + Hash, V, S: BuildHasher, const N: usize> CollectionBase for IndexMap<K, V, S, N> {
    type Item = (K, V);

    fn item_count(&self) -> usize {
        IndexMap::len(self)
    }

    #[track_caller]
    fn push(&mut self, elem: Self::Item) {
        if IndexMap::insert(self, elem.0, elem.1).is_err() {
            capacity_exceeded()
        }
    }

    fn clear(&mut self) {
        IndexMap::clear(self)
    }
}

impl<K: Eq + Hash, V, S: BuildHasher + Default, const N: usize> Collection
    for IndexMap<K, V, S, N>
{
    /// The capacity is fixed by the type, so this argument is ignored.
    fn with_capacity(_capacity: usize) -> Self {
        IndexMap::default()
    }
}

impl<K: Eq + Hash, V, S: BuildHasher, const N: usize> CollectionCapacity for IndexMap<K, V, S, N> {
    type ReserveError = CapacityError;

    fn capacity(&self) -> usize {
        IndexMap::capacity(self)
    }

    #[track_caller]
    fn reserve(&mut self, additional: usize) {
        if check_room(IndexMap::len(self), IndexMap::capacity(self), additional).is_err() {
            capacity_exceeded()
        }
    }

    fn try_reserve(&mut self, additional: usize) -> Result<(), Self::ReserveError> {
        check_room(IndexMap::len(self), IndexMap::capacity(self), additional)
    }

    fn try_push(&mut self, elem: Self::Item) -> Result<(), Self::Item> {
        IndexMap::insert(self, elem.0, elem.1).map(|_| ())
    }
}

impl<K: Eq + Hash, V, S: BuildHasher, const N: usize> CollectionMap for IndexMap<K, V, S, N> {
    type Key = K;
    type Value = V;

    fn contains_key(&self, key: &Self::Key) -> bool {
        IndexMap::contains_key(self, key)
    }

    fn get(&self, key: &Self::Key) -> Option<&Self::Value> {
        IndexMap::get(self, key)
    }

    fn get_mut(&mut self, key: &Self::Key) -> Option<&mut Self::Value> {
        IndexMap::get_mut(self, key)
    }

    fn iter(&self) -> impl Iterator<Item = (&Self::Key, &Self::Value)> {
        IndexMap::iter(self)
    }

    fn keys(&self) -> impl Iterator<Item = &Self::Key> {
        IndexMap::keys(self)
    }

    fn values(&self) -> impl Iterator<Item = &Self::Value> {
        IndexMap::values(self)
    }

    fn iter_mut(&mut self) -> impl Iterator<Item = (&Self::Key, &mut Self::Value)> {
        IndexMap::iter_mut(self)
    }

    fn values_mut(&mut self) -> impl Iterator<Item = &mut Self::Value> {
        IndexMap::values_mut(self)
    }

    #[track_caller]
    fn insert(&mut self, key: Self::Key, value: Self::Value) -> Option<Self::Value> {
        match IndexMap::insert(self, key, value) {
            Ok(previous) => previous,
            Err(_) => capacity_exceeded(),
        }
    }

    fn remove(&mut self, key: &Self::Key) -> Option<Self::Value> {
        IndexMap::remove(self, key)
    }

    fn retain(&mut self, f: impl FnMut(&K, &mut V) -> bool) {
        IndexMap::retain(self, f)
    }
}

impl<K: Eq + Hash, V, S: BuildHasher, const N: usize> CollectionEntry for IndexMap<K, V, S, N> {
    type Entry<'a>
        = index_map::Entry<'a, K, V, N>
    where
        K: 'a,
        V: 'a,
        S: 'a;

    fn entry(&mut self, key: Self::Key) -> Self::Entry<'_> {
        IndexMap::entry(self, key)
    }
}

impl<'a, K: Eq + Hash + 'a, V: 'a, const N: usize> MapEntry<'a> for index_map::Entry<'a, K, V, N> {
    type Key = K;
    type Value = V;

    fn key(&self) -> &Self::Key {
        index_map::Entry::key(self)
    }

    #[track_caller]
    fn or_insert(self, default: Self::Value) -> &'a mut Self::Value {
        match index_map::Entry::or_insert(self, default) {
            Ok(value) => value,
            Err(_) => capacity_exceeded(),
        }
    }

    #[track_caller]
    fn or_insert_with<F: FnOnce() -> Self::Value>(self, default: F) -> &'a mut Self::Value {
        match index_map::Entry::or_insert_with(self, default) {
            Ok(value) => value,
            Err(_) => capacity_exceeded(),
        }
    }

    #[track_caller]
    fn or_default(self) -> &'a mut Self::Value
    where
        Self::Value: Default,
    {
        match index_map::Entry::or_default(self) {
            Ok(value) => value,
            Err(_) => capacity_exceeded(),
        }
    }

    fn and_modify<F: FnOnce(&mut Self::Value)>(self, f: F) -> Self {
        index_map::Entry::and_modify(self, f)
    }
}

impl<K: Eq, V, const N: usize> CollectionBase for LinearMap<K, V, N> {
    type Item = (K, V);

    fn item_count(&self) -> usize {
        LinearMap::len(self)
    }

    #[track_caller]
    fn push(&mut self, elem: Self::Item) {
        if LinearMap::insert(self, elem.0, elem.1).is_err() {
            capacity_exceeded()
        }
    }

    fn clear(&mut self) {
        LinearMap::clear(self)
    }
}

impl<K: Eq, V, const N: usize> CollectionCapacity for LinearMap<K, V, N> {
    type ReserveError = CapacityError;

    fn capacity(&self) -> usize {
        LinearMap::capacity(self)
    }

    #[track_caller]
    fn reserve(&mut self, additional: usize) {
        if check_room(LinearMap::len(self), LinearMap::capacity(self), additional).is_err() {
            capacity_exceeded()
        }
    }

    fn try_reserve(&mut self, additional: usize) -> Result<(), Self::ReserveError> {
        check_room(LinearMap::len(self), LinearMap::capacity(self), additional)
    }

    fn try_push(&mut self, elem: Self::Item) -> Result<(), Self::Item> {
        LinearMap::insert(self, elem.0, elem.1).map(|_| ())
    }
}

impl<K: Eq, V, const N: usize> CollectionMap for LinearMap<K, V, N> {
    type Key = K;
    type Value = V;

    fn contains_key(&self, key: &Self::Key) -> bool {
        LinearMap::contains_key(self, key)
    }

    fn get(&self, key: &Self::Key) -> Option<&Self::Value> {
        LinearMap::get(self, key)
    }

    fn get_mut(&mut self, key: &Self::Key) -> Option<&mut Self::Value> {
        LinearMap::get_mut(self, key)
    }

    fn iter(&self) -> impl Iterator<Item = (&Self::Key, &Self::Value)> {
        LinearMap::iter(self)
    }

    fn keys(&self) -> impl Iterator<Item = &Self::Key> {
        LinearMap::keys(self)
    }

    fn values(&self) -> impl Iterator<Item = &Self::Value> {
        LinearMap::values(self)
    }

    fn iter_mut(&mut self) -> impl Iterator<Item = (&Self::Key, &mut Self::Value)> {
        LinearMap::iter_mut(self)
    }

    fn values_mut(&mut self) -> impl Iterator<Item = &mut Self::Value> {
        LinearMap::values_mut(self)
    }

    #[track_caller]
    fn insert(&mut self, key: Self::Key, value: Self::Value) -> Option<Self::Value> {
        match LinearMap::insert(self, key, value) {
            Ok(previous) => previous,
            Err(_) => capacity_exceeded(),
        }
    }

    fn remove(&mut self, key: &Self::Key) -> Option<Self::Value> {
        LinearMap::remove(self, key)
    }

    fn retain(&mut self, f: impl FnMut(&K, &mut V) -> bool) {
        LinearMap::retain(self, f)
    }
}
