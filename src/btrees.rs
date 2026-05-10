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

use alloc::collections::{BTreeMap, BTreeSet, btree_map};
use core::hash::Hash;

use super::{Collection, KeyedCollection};

impl<T: Ord> Collection for BTreeSet<T> {
    type Item = T;

    #[doc(hidden)]
    fn with_capacity(_capacity: usize) -> Self {
        BTreeSet::new()
    }

    fn len(&self) -> usize {
        BTreeSet::len(self)
    }

    fn push(&mut self, elem: Self::Item) {
        BTreeSet::insert(self, elem);
    }

    fn clear(&mut self) {
        BTreeSet::clear(self)
    }
}

impl<K: Ord + Hash, V> Collection for BTreeMap<K, V> {
    type Item = (K, V);

    #[doc(hidden)]
    fn with_capacity(_capacity: usize) -> Self {
        BTreeMap::new()
    }

    fn len(&self) -> usize {
        BTreeMap::len(self)
    }

    fn push(&mut self, elem: Self::Item) {
        BTreeMap::insert(self, elem.0, elem.1);
    }

    fn clear(&mut self) {
        BTreeMap::clear(self)
    }
}

impl<K: Ord + Hash, V> KeyedCollection for BTreeMap<K, V> {
    type Key = K;
    type Value = V;
    type Entry<'a>
        = btree_map::Entry<'a, K, V>
    where
        K: 'a,
        V: 'a;

    fn contains_key(&self, key: &Self::Key) -> bool {
        BTreeMap::contains_key(self, key)
    }

    fn get(&self, key: &Self::Key) -> Option<&Self::Value> {
        BTreeMap::get(self, key)
    }

    fn get_mut(&mut self, key: &Self::Key) -> Option<&mut Self::Value> {
        BTreeMap::get_mut(self, key)
    }

    fn iter(&self) -> impl Iterator<Item = (&Self::Key, &Self::Value)> {
        BTreeMap::iter(self)
    }

    fn iter_mut(&mut self) -> impl Iterator<Item = (&Self::Key, &mut Self::Value)> {
        BTreeMap::iter_mut(self)
    }

    fn values_mut(&mut self) -> impl Iterator<Item = &mut Self::Value> {
        BTreeMap::values_mut(self)
    }

    fn insert(&mut self, key: Self::Key, value: Self::Value) -> Option<Self::Value> {
        BTreeMap::insert(self, key, value)
    }

    fn remove(&mut self, key: &Self::Key) -> Option<Self::Value> {
        BTreeMap::remove(self, key)
    }

    fn entry(&mut self, key: Self::Key) -> Self::Entry<'_> {
        BTreeMap::entry(self, key)
    }

    fn retain(&mut self, f: impl FnMut(&K, &mut V) -> bool) {
        BTreeMap::retain(self, f)
    }
}
