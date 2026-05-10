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

/// Macro for quick & simple `&str` -> `String` conversion:
/// ```
/// #[macro_use]
/// extern crate collections;
///
/// # fn main() {
/// enum Error {
///     Io(String),
/// }
///
/// impl From<std::io::Error> for Error {
///     fn from(err: std::io::Error) -> Error {
///         Self::Io(s!("I/O error"))
///     }
/// }
/// # }
/// ```
#[macro_export]
macro_rules! s {
    ($str:literal) => {
        $crate::alloc::String::from($str)
    };
}

/// Macro for creating [`alloc::collections::BTreeMap`] in the same manner as
/// `vec!` is used for [`Vec`]:
/// ```
/// #[macro_use]
/// extern crate collections;
/// extern crate alloc;
///
/// # fn main() {
/// let map = bmap! {
///     s!("key") => 5,
///     s!("other_key") => 10
/// };
/// # }
/// ```
#[macro_export]
macro_rules! bmap {
    { } =>  {
        {
            $crate::alloc::collections::BTreeMap::new()
        }
    };

    { owned: $($key:expr => $value:expr),+ $(,)? } => {
        {
            let mut m = $crate::alloc::collections::BTreeMap::new();
            $(
                m.insert($key.to_owned(), $value.to_owned());
            )+
            m
        }
    };

    { $($key:expr => $value:expr),+ $(,)? } => {
        {
            let mut m = $crate::alloc::collections::BTreeMap::new();
            $(
                m.insert($key, $value);
            )+
            m
        }
    }
}

/// Macro for creating [`alloc::collections::BTreeSet`] in the same manner as
/// `vec!` is used for [`Vec`]:
/// ```
/// #[macro_use]
/// extern crate collections;
/// extern crate alloc;
///
/// # fn main() {
/// let map = bset![5, 6, 7];
/// # }
/// ```
///
/// NB: you can't use repeated values with [`alloc::collections::HashSet`],
/// unlike to [`Vec`]'s:
/// ```
/// #[macro_use]
/// extern crate collections;
/// extern crate alloc;
///
/// # fn main() {
/// assert_eq!(bset![1, 2, 3, 1], bset![1, 2, 3]);
/// # }
/// ```
#[macro_export]
macro_rules! bset {
    { } =>  {
        {
            $crate::alloc::collections::BTreeSet::new()
        }
    };

    { owned: $($value:expr),+ $(,)? } => {
        {
            let mut m = $crate::alloc::collections::BTreeSet::new();
            $(
                m.insert($value.to_owned());
            )+
            m
        }
    };

    { $($value:expr),+ $(,)? } => {
        {
            let mut m = $crate::alloc::collections::BTreeSet::new();
            $(
                m.insert($value);
            )+
            m
        }
    }
}

/// Macro for creating [`alloc::collections::LinkedList`] in the same manner as
/// `vec!` is used for [`Vec`]:
/// ```
/// #[macro_use]
/// extern crate collections;
/// extern crate alloc;
///
/// # fn main() {
/// let list = list! {
///     s!("item one") =>
///     s!("item two") =>
///     s!("item three")
/// };
/// # }
/// ```
#[macro_export]
macro_rules! list {
    { } =>  {
        {
            $crate::alloc::collections::LinkedList::new()
        }
    };

    { owned: $($value:expr)=>+ } => {
        {
            let mut m = $crate::alloc::collections::LinkedList::new();
            $(
                m.push_back($value.to_owned());
            )+
            m
        }
    };

    { $($value:expr)=>+ } => {
        {
            let mut m = $crate::alloc::collections::LinkedList::new();
            $(
                m.push_back($value);
            )+
            m
        }
    }
}
