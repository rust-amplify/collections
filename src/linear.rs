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

use alloc::collections::{LinkedList, VecDeque};
use alloc::string::String;
use alloc::vec::Vec;
use core::alloc::Allocator;

use super::{Collection, CollectionAlloc, CollectionBase, CollectionBytes, CollectionCapacity};

impl CollectionBase for String {
    type Item = char;

    #[inline]
    fn item_count(&self) -> usize {
        self.chars().count()
    }

    #[inline]
    fn is_empty(&self) -> bool {
        String::is_empty(self)
    }

    #[inline]
    fn push(&mut self, elem: Self::Item) {
        String::push(self, elem)
    }

    #[inline]
    fn clear(&mut self) {
        String::clear(self)
    }
}

impl CollectionBytes for String {
    #[inline]
    fn byte_len(&self) -> usize {
        String::len(self)
    }

    #[inline]
    fn item_byte_len(item: &Self::Item) -> usize {
        item.len_utf8()
    }
}

impl Collection for String {
    #[inline]
    fn with_capacity(capacity: usize) -> Self {
        String::with_capacity(capacity)
    }
}

impl CollectionCapacity for String {
    type ReserveError = alloc::collections::TryReserveError;

    #[inline]
    fn capacity(&self) -> usize {
        String::capacity(self)
    }

    #[inline]
    fn reserve(&mut self, additional: usize) {
        String::reserve(self, additional)
    }

    #[inline]
    fn try_reserve(&mut self, additional: usize) -> Result<(), Self::ReserveError> {
        String::try_reserve(self, additional)
    }

    #[inline]
    fn try_push(&mut self, elem: Self::Item) -> Result<(), Self::Item> {
        if String::try_reserve(self, elem.len_utf8()).is_err() {
            return Err(elem);
        }
        String::push(self, elem);
        Ok(())
    }
}

impl<T, A: Allocator> CollectionBase for Vec<T, A> {
    type Item = T;

    fn item_count(&self) -> usize {
        Vec::len(self)
    }

    fn push(&mut self, elem: Self::Item) {
        Vec::push(self, elem)
    }

    fn clear(&mut self) {
        Vec::clear(self)
    }
}

impl<T> Collection for Vec<T> {
    fn with_capacity(capacity: usize) -> Self {
        Vec::with_capacity(capacity)
    }
}

impl<T, A: Allocator> CollectionAlloc for Vec<T, A> {
    type Allocator = A;

    fn with_capacity_in(capacity: usize, allocator: Self::Allocator) -> Self {
        Vec::with_capacity_in(capacity, allocator)
    }

    fn allocator(&self) -> &Self::Allocator {
        Vec::allocator(self)
    }
}

impl<T, A: Allocator> CollectionCapacity for Vec<T, A> {
    type ReserveError = alloc::collections::TryReserveError;

    fn capacity(&self) -> usize {
        Vec::capacity(self)
    }

    fn reserve(&mut self, additional: usize) {
        Vec::reserve(self, additional)
    }

    fn try_reserve(&mut self, additional: usize) -> Result<(), Self::ReserveError> {
        Vec::try_reserve(self, additional)
    }
}

impl<A: Allocator> CollectionBytes for Vec<u8, A> {
    fn byte_len(&self) -> usize {
        Vec::len(self)
    }

    fn item_byte_len(_item: &Self::Item) -> usize {
        1
    }
}

impl<T> CollectionBase for VecDeque<T> {
    type Item = T;

    fn item_count(&self) -> usize {
        VecDeque::len(self)
    }

    fn push(&mut self, elem: Self::Item) {
        VecDeque::push_back(self, elem)
    }

    fn clear(&mut self) {
        VecDeque::clear(self)
    }
}

impl<T> Collection for VecDeque<T> {
    fn with_capacity(capacity: usize) -> Self {
        VecDeque::with_capacity(capacity)
    }
}

impl<T> CollectionCapacity for VecDeque<T> {
    type ReserveError = alloc::collections::TryReserveError;

    fn capacity(&self) -> usize {
        VecDeque::capacity(self)
    }

    fn reserve(&mut self, additional: usize) {
        VecDeque::reserve(self, additional)
    }

    fn try_reserve(&mut self, additional: usize) -> Result<(), Self::ReserveError> {
        VecDeque::try_reserve(self, additional)
    }
}

impl<T> CollectionBase for LinkedList<T> {
    type Item = T;

    fn item_count(&self) -> usize {
        LinkedList::len(self)
    }

    fn push(&mut self, elem: Self::Item) {
        LinkedList::push_back(self, elem)
    }

    fn clear(&mut self) {
        LinkedList::clear(self)
    }
}

impl<T> Collection for LinkedList<T> {
    fn with_capacity(_capacity: usize) -> Self {
        LinkedList::new()
    }
}
