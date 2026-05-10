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

use core::hash::Hash;

use indexmap::{IndexMap, IndexSet, map};

use super::{Collection, KeyedCollection};

impl<T: Eq + Hash> Collection for IndexSet<T> {
    type Item = T;

    fn with_capacity(capacity: usize) -> Self {
        IndexSet::with_capacity(capacity)
    }

    fn len(&self) -> usize {
        IndexSet::len(self)
    }

    fn push(&mut self, elem: Self::Item) {
        IndexSet::insert(self, elem);
    }

    fn clear(&mut self) {
        IndexSet::clear(self)
    }
}

impl<K: Eq + Hash, V> Collection for IndexMap<K, V> {
    type Item = (K, V);

    fn with_capacity(capacity: usize) -> Self {
        IndexMap::with_capacity(capacity)
    }

    fn len(&self) -> usize {
        IndexMap::len(self)
    }

    fn push(&mut self, elem: Self::Item) {
        IndexMap::insert(self, elem.0, elem.1);
    }

    fn clear(&mut self) {
        IndexMap::clear(self)
    }
}

impl<K: Eq + Hash, V> KeyedCollection for IndexMap<K, V> {
    type Key = K;
    type Value = V;
    type Entry<'a>
        = map::Entry<'a, K, V>
    where
        K: 'a,
        V: 'a;

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

    fn entry(&mut self, key: Self::Key) -> Self::Entry<'_> {
        IndexMap::entry(self, key)
    }

    fn retain(&mut self, f: impl FnMut(&K, &mut V) -> bool) {
        IndexMap::retain(self, f)
    }
}
