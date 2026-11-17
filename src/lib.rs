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

//! Generic traits for working with collection types from `alloc` and `std`.
//!
//! [`Collection`] provides a common interface for sequences, sets, maps and
//! heaps. [`CollectionAlloc`] covers allocator-aware collections, while
//! [`CollectionSet`] and [`CollectionMap`] add set and map operations; maps
//! also expose their native entry type through [`MapEntry`]. The crate works in
//! `no_std` environments when an allocator is available.

#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg_attr(not(feature = "std"), no_std)]
#![warn(missing_docs)]

/// Re-export of `alloc` for no-`std` callers and macro expansion.
pub extern crate alloc;

#[cfg(feature = "hashbrown")]
/// Re-export of the optional `allocator-api2` dependency used by `hashbrown`.
pub extern crate allocator_api2;
#[cfg(feature = "ascii")]
/// Re-export of the optional `ascii` dependency.
pub extern crate ascii;
#[cfg(feature = "hashbrown")]
/// Re-export of the optional `hashbrown` dependency.
pub extern crate hashbrown;
#[cfg(feature = "indexmap")]
/// Re-export of the optional `indexmap` dependency.
pub extern crate indexmap;

mod traits;
mod linear;
mod btrees;
mod heap;
#[cfg(feature = "std")]
mod hashed;
#[cfg(feature = "hashbrown")]
mod hashbrowns;
#[cfg(feature = "ascii")]
mod asciistr;
#[cfg(feature = "indexmap")]
mod indexed;

mod macros;

#[cfg(feature = "std")]
#[doc(hidden)]
pub extern crate std;

pub use traits::{Collection, CollectionAlloc, CollectionMap, CollectionSet, MapEntry};

#[cfg(all(doctest, feature = "std"))]
#[doc(hidden)]
#[doc = include_str!("../README.md")]
pub struct ReadmeDoctests;
