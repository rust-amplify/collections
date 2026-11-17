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

use ascii::{AsciiChar, AsciiString};

use super::{Collection, CollectionBase, CollectionBytes};

impl CollectionBase for AsciiString {
    type Item = AsciiChar;

    #[inline]
    fn item_count(&self) -> usize {
        AsciiString::len(self)
    }

    #[inline]
    fn push(&mut self, elem: Self::Item) {
        AsciiString::push(self, elem)
    }

    #[inline]
    fn clear(&mut self) {
        AsciiString::clear(self)
    }
}

impl CollectionBytes for AsciiString {
    #[inline]
    fn byte_len(&self) -> usize {
        AsciiString::len(self)
    }

    #[inline]
    fn item_byte_len(_item: &Self::Item) -> usize {
        1
    }
}

impl Collection for AsciiString {
    #[inline]
    fn with_capacity(capacity: usize) -> Self {
        AsciiString::with_capacity(capacity)
    }
}
