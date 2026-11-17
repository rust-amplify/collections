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

use indexmap::{IndexMap, IndexSet, map};

use super::{
    Collection, CollectionBase, CollectionCapacity, CollectionEntry, CollectionMap, CollectionSet,
    MapEntry,
};

impl<T: Eq + Hash, S: BuildHasher> CollectionBase for IndexSet<T, S> {
    type Item = T;

    fn item_count(&self) -> usize {
        IndexSet::len(self)
    }

    fn push(&mut self, elem: Self::Item) {
        IndexSet::insert(self, elem);
    }

    fn clear(&mut self) {
        IndexSet::clear(self)
    }
}

impl<T: Eq + Hash, S: BuildHasher + Default> Collection for IndexSet<T, S> {
    fn with_capacity(capacity: usize) -> Self {
        IndexSet::with_capacity_and_hasher(capacity, S::default())
    }
}

impl<T: Eq + Hash, S: BuildHasher> CollectionSet for IndexSet<T, S> {
    fn contains(&self, item: &Self::Item) -> bool {
        IndexSet::contains(self, item)
    }

    fn get(&self, item: &Self::Item) -> Option<&Self::Item> {
        IndexSet::get(self, item)
    }

    fn iter(&self) -> impl Iterator<Item = &Self::Item> {
        IndexSet::iter(self)
    }

    fn insert(&mut self, item: Self::Item) -> bool {
        IndexSet::insert(self, item)
    }

    fn replace(&mut self, item: Self::Item) -> Option<Self::Item> {
        IndexSet::replace(self, item)
    }

    fn remove(&mut self, item: &Self::Item) -> bool {
        IndexSet::shift_remove(self, item)
    }

    fn take(&mut self, item: &Self::Item) -> Option<Self::Item> {
        IndexSet::shift_take(self, item)
    }

    fn retain(&mut self, f: impl FnMut(&Self::Item) -> bool) {
        IndexSet::retain(self, f)
    }
}

impl<T: Eq + Hash, S: BuildHasher> CollectionCapacity for IndexSet<T, S> {
    type ReserveError = indexmap::TryReserveError;

    fn capacity(&self) -> usize {
        IndexSet::capacity(self)
    }

    fn reserve(&mut self, additional: usize) {
        IndexSet::reserve(self, additional)
    }

    fn try_reserve(&mut self, additional: usize) -> Result<(), Self::ReserveError> {
        IndexSet::try_reserve(self, additional)
    }
}

impl<K: Eq + Hash, V, S: BuildHasher> CollectionBase for IndexMap<K, V, S> {
    type Item = (K, V);

    fn item_count(&self) -> usize {
        IndexMap::len(self)
    }

    fn push(&mut self, elem: Self::Item) {
        IndexMap::insert(self, elem.0, elem.1);
    }

    fn clear(&mut self) {
        IndexMap::clear(self)
    }
}

impl<K: Eq + Hash, V, S: BuildHasher + Default> Collection for IndexMap<K, V, S> {
    fn with_capacity(capacity: usize) -> Self {
        IndexMap::with_capacity_and_hasher(capacity, S::default())
    }
}

impl<K: Eq + Hash, V, S: BuildHasher> CollectionMap for IndexMap<K, V, S> {
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

    fn insert(&mut self, key: Self::Key, value: Self::Value) -> Option<Self::Value> {
        IndexMap::insert(self, key, value)
    }

    fn remove(&mut self, key: &Self::Key) -> Option<Self::Value> {
        IndexMap::shift_remove(self, key)
    }

    fn retain(&mut self, f: impl FnMut(&K, &mut V) -> bool) {
        IndexMap::retain(self, f)
    }
}

impl<K: Eq + Hash, V, S: BuildHasher> CollectionCapacity for IndexMap<K, V, S> {
    type ReserveError = indexmap::TryReserveError;

    fn capacity(&self) -> usize {
        IndexMap::capacity(self)
    }

    fn reserve(&mut self, additional: usize) {
        IndexMap::reserve(self, additional)
    }

    fn try_reserve(&mut self, additional: usize) -> Result<(), Self::ReserveError> {
        IndexMap::try_reserve(self, additional)
    }
}

impl<K: Eq + Hash, V, S: BuildHasher> CollectionEntry for IndexMap<K, V, S> {
    type Entry<'a>
        = map::Entry<'a, K, V>
    where
        K: 'a,
        V: 'a,
        S: 'a;

    fn entry(&mut self, key: Self::Key) -> Self::Entry<'_> {
        IndexMap::entry(self, key)
    }
}

impl<'a, K: 'a, V: 'a> MapEntry<'a> for map::Entry<'a, K, V> {
    type Key = K;
    type Value = V;

    fn key(&self) -> &Self::Key {
        map::Entry::key(self)
    }

    fn or_insert(self, default: Self::Value) -> &'a mut Self::Value {
        map::Entry::or_insert(self, default)
    }

    fn or_insert_with<F: FnOnce() -> Self::Value>(self, default: F) -> &'a mut Self::Value {
        map::Entry::or_insert_with(self, default)
    }

    fn or_default(self) -> &'a mut Self::Value
    where
        Self::Value: Default,
    {
        map::Entry::or_default(self)
    }

    fn and_modify<F: FnOnce(&mut Self::Value)>(self, f: F) -> Self {
        map::Entry::and_modify(self, f)
    }
}
