// SPDX-License-Identifier: Apache-2.0
//
// Written in 2022-2026 by Dr. Maxim Orlovsky <orlovsky@ubideco.org>
//
// Copyright 2022-2026 Laboratories for Ubiquitous and Deterministic Computing,
// Institute for Distributed and Cognitive Computing (InDCS), Switzerland.
// All rights reserved.
//
// Copyright (C) 2022-2026 Dr Maxim Orlovsky.// All rights reserved.
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

use alloc::collections::BinaryHeap;

use super::{Collection, CollectionBase, CollectionCapacity};

impl<T: Ord> CollectionBase for BinaryHeap<T> {
    type Item = T;

    fn item_count(&self) -> usize {
        BinaryHeap::len(self)
    }

    fn push(&mut self, elem: Self::Item) {
        BinaryHeap::push(self, elem)
    }

    fn clear(&mut self) {
        BinaryHeap::clear(self)
    }
}

impl<T: Ord> Collection for BinaryHeap<T> {
    fn with_capacity(capacity: usize) -> Self {
        BinaryHeap::with_capacity(capacity)
    }
}

impl<T: Ord> CollectionCapacity for BinaryHeap<T> {
    type ReserveError = alloc::collections::TryReserveError;

    fn capacity(&self) -> usize {
        BinaryHeap::capacity(self)
    }

    fn reserve(&mut self, additional: usize) {
        BinaryHeap::reserve(self, additional)
    }

    fn try_reserve(&mut self, additional: usize) -> Result<(), Self::ReserveError> {
        BinaryHeap::try_reserve(self, additional)
    }
}
