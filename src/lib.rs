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

#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![cfg_attr(not(feature = "std"), no_std)]

pub extern crate alloc;
extern crate core;

#[cfg(feature = "ascii")]
pub extern crate ascii;
#[cfg(feature = "indexmap")]
pub extern crate indexmap;

mod traits;
mod linear;
mod btrees;
#[cfg(feature = "std")]
mod hashed;
#[cfg(feature = "ascii")]
mod asciistr;
#[cfg(feature = "indexmap")]
mod indexed;

#[cfg(feature = "std")]
#[macro_use]
mod macro_std;
#[cfg(not(feature = "std"))]
#[macro_use]
mod macro_alloc;

pub use traits::{Collection, KeyedCollection};
