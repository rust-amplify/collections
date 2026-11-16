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

use std::collections::{HashMap, HashSet, hash_map};
use std::hash::{BuildHasher, Hash};

use super::{Collection, KeyedCollection, MapEntry};

impl<T: Eq + Hash, S: BuildHasher + Default> Collection for HashSet<T, S> {
    type Item = T;

    fn with_capacity(capacity: usize) -> Self {
        HashSet::with_capacity_and_hasher(capacity, S::default())
    }

    fn len(&self) -> usize {
        HashSet::len(self)
    }

    fn push(&mut self, elem: Self::Item) {
        HashSet::insert(self, elem);
    }

    fn clear(&mut self) {
        HashSet::clear(self)
    }
}

impl<K: Eq + Hash, V, S: BuildHasher + Default> Collection for HashMap<K, V, S> {
    type Item = (K, V);

    fn with_capacity(capacity: usize) -> Self {
        HashMap::with_capacity_and_hasher(capacity, S::default())
    }

    fn len(&self) -> usize {
        HashMap::len(self)
    }

    fn push(&mut self, elem: Self::Item) {
        HashMap::insert(self, elem.0, elem.1);
    }

    fn clear(&mut self) {
        HashMap::clear(self)
    }
}

impl<K: Eq + Hash, V, S: BuildHasher + Default> KeyedCollection for HashMap<K, V, S> {
    type Key = K;
    type Value = V;
    type Entry<'a>
        = hash_map::Entry<'a, K, V>
    where
        K: 'a,
        V: 'a,
        S: 'a;

    fn contains_key(&self, key: &Self::Key) -> bool {
        HashMap::contains_key(self, key)
    }

    fn get(&self, key: &Self::Key) -> Option<&Self::Value> {
        HashMap::get(self, key)
    }

    fn get_mut(&mut self, key: &Self::Key) -> Option<&mut Self::Value> {
        HashMap::get_mut(self, key)
    }

    fn iter(&self) -> impl Iterator<Item = (&Self::Key, &Self::Value)> {
        HashMap::iter(self)
    }

    fn keys(&self) -> impl Iterator<Item = &Self::Key> {
        HashMap::keys(self)
    }

    fn values(&self) -> impl Iterator<Item = &Self::Value> {
        HashMap::values(self)
    }

    fn iter_mut(&mut self) -> impl Iterator<Item = (&Self::Key, &mut Self::Value)> {
        HashMap::iter_mut(self)
    }

    fn values_mut(&mut self) -> impl Iterator<Item = &mut Self::Value> {
        HashMap::values_mut(self)
    }

    fn insert(&mut self, key: Self::Key, value: Self::Value) -> Option<Self::Value> {
        HashMap::insert(self, key, value)
    }

    fn remove(&mut self, key: &Self::Key) -> Option<Self::Value> {
        HashMap::remove(self, key)
    }

    fn entry(&mut self, key: Self::Key) -> Self::Entry<'_> {
        HashMap::entry(self, key)
    }

    fn retain(&mut self, f: impl FnMut(&K, &mut V) -> bool) {
        HashMap::retain(self, f)
    }
}

impl<'a, K: 'a, V: 'a> MapEntry<'a> for hash_map::Entry<'a, K, V> {
    type Key = K;
    type Value = V;

    fn key(&self) -> &Self::Key {
        hash_map::Entry::key(self)
    }

    fn or_insert(self, default: Self::Value) -> &'a mut Self::Value {
        hash_map::Entry::or_insert(self, default)
    }

    fn or_insert_with<F: FnOnce() -> Self::Value>(self, default: F) -> &'a mut Self::Value {
        hash_map::Entry::or_insert_with(self, default)
    }

    fn or_default(self) -> &'a mut Self::Value
    where
        Self::Value: Default,
    {
        hash_map::Entry::or_default(self)
    }

    fn and_modify<F: FnOnce(&mut Self::Value)>(self, f: F) -> Self {
        hash_map::Entry::and_modify(self, f)
    }
}
