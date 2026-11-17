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

//! Generic traits for working with heap-allocated, allocator-aware and
//! fixed-capacity collection types.
//!
//! [`CollectionBase`] is the constructor-free root for sequences, sets, maps
//! and heaps. [`Collection`] and [`CollectionAlloc`] add constructors,
//! [`CollectionBytes`] provides byte lengths for byte buffers, and
//! [`CollectionCapacity`] provides fallible reservation and insertion.
//! [`CollectionSet`] and [`CollectionMap`] add set and map operations, while
//! [`CollectionEntry`] and [`MapEntry`] provide generic access to native map
//! entry APIs.
//!
//! The traits themselves need neither `alloc` nor a global allocator. The
//! `alloc` feature (implied by the default `std` feature) adds the
//! implementations for the collections of the `alloc` crate, and the
//! `heapless` feature adds them for fixed-capacity collections which work on
//! targets without a heap.

#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg_attr(not(feature = "std"), no_std)]
#![forbid(unsafe_code)]
#![warn(missing_docs)]

/// Re-export of `alloc` for no-`std` callers and macro expansion.
#[cfg(feature = "alloc")]
pub extern crate alloc;

#[cfg(feature = "ascii")]
/// Re-export of the optional `ascii` dependency.
pub extern crate ascii;
#[cfg(feature = "hashbrown")]
/// Re-export of the optional `hashbrown` dependency.
pub extern crate hashbrown;
#[cfg(feature = "heapless")]
/// Re-export of the optional `heapless` dependency.
pub extern crate heapless;
#[cfg(feature = "indexmap")]
/// Re-export of the optional `indexmap` dependency.
pub extern crate indexmap;

mod traits;
#[cfg(feature = "alloc")]
mod linear;
#[cfg(feature = "alloc")]
mod btrees;
#[cfg(feature = "alloc")]
mod heap;
#[cfg(feature = "std")]
mod hashed;
#[cfg(feature = "hashbrown")]
mod hashbrowns;
#[cfg(feature = "ascii")]
mod asciistr;
#[cfg(feature = "indexmap")]
mod indexed;

#[cfg(feature = "alloc")]
mod macros;

#[cfg(feature = "heapless")]
mod fixed;

#[cfg(feature = "std")]
#[doc(hidden)]
pub extern crate std;

pub use traits::{
    Collection, CollectionAlloc, CollectionBase, CollectionBytes, CollectionCapacity,
    CollectionEntry, CollectionMap, CollectionSet, MapEntry,
};

#[cfg(all(doctest, feature = "std"))]
#[doc(hidden)]
#[doc = include_str!("../README.md")]
pub struct ReadmeDoctests;
