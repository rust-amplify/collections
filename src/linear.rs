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

use alloc::collections::VecDeque;
use alloc::string::String;
use alloc::vec::Vec;

use super::Collection;

impl Collection for String {
    type Item = char;

    fn with_capacity(capacity: usize) -> Self {
        String::with_capacity(capacity)
    }

    fn len(&self) -> usize {
        String::len(self)
    }

    fn push(&mut self, elem: Self::Item) {
        String::push(self, elem)
    }

    fn clear(&mut self) {
        String::clear(self)
    }
}

impl<T> Collection for Vec<T> {
    type Item = T;

    fn with_capacity(capacity: usize) -> Self {
        Vec::with_capacity(capacity)
    }

    fn len(&self) -> usize {
        Vec::len(self)
    }

    fn push(&mut self, elem: Self::Item) {
        Vec::push(self, elem)
    }

    fn clear(&mut self) {
        Vec::clear(self)
    }
}

impl<T> Collection for VecDeque<T> {
    type Item = T;

    fn with_capacity(capacity: usize) -> Self {
        VecDeque::with_capacity(capacity)
    }

    fn len(&self) -> usize {
        VecDeque::len(self)
    }

    fn push(&mut self, elem: Self::Item) {
        VecDeque::push_back(self, elem)
    }

    fn clear(&mut self) {
        VecDeque::clear(self)
    }
}
