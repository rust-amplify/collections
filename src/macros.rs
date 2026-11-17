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

/// Converts a string literal into an allocated `String`.
///
/// ```
/// use collections::s;
///
/// let text = s!("hello");
/// assert_eq!(text, "hello");
/// ```
#[macro_export]
macro_rules! s {
    ($str:literal) => {
        $crate::alloc::string::String::from($str)
    };
}

/// Creates an allocated `BTreeMap` from key-value pairs.
///
/// The `owned:` form calls `to_owned()` on each key and value.
///
/// ```
/// use collections::bmap;
///
/// let map = bmap! { 1 => "one", 2 => "two" };
/// assert_eq!(map.len(), 2);
///
/// let owned = bmap! { owned: "key" => "value" };
/// assert_eq!(owned["key"], "value");
/// ```
#[macro_export]
macro_rules! bmap {
    {} => {
        $crate::alloc::collections::BTreeMap::new()
    };

    { owned: $($key:expr_2021 => $value:expr_2021),+ $(,)? } => {{
        use $crate::alloc::borrow::ToOwned as _;

        let mut map = $crate::alloc::collections::BTreeMap::new();
        $(map.insert($key.to_owned(), $value.to_owned());)+
        map
    }};

    { $($key:expr_2021 => $value:expr_2021),+ $(,)? } => {{
        let mut map = $crate::alloc::collections::BTreeMap::new();
        $(map.insert($key, $value);)+
        map
    }};
}

/// Creates an allocated `BTreeSet` from values.
///
/// Repeated values are stored once. The `owned:` form calls `to_owned()` on
/// each value.
///
/// ```
/// use collections::bset;
///
/// assert_eq!(bset![1, 2, 3, 1], bset![1, 2, 3]);
/// let owned = bset! { owned: "one", "two" };
/// assert!(owned.contains("one"));
/// ```
#[macro_export]
macro_rules! bset {
    {} => {
        $crate::alloc::collections::BTreeSet::new()
    };

    { owned: $($value:expr_2021),+ $(,)? } => {{
        use $crate::alloc::borrow::ToOwned as _;

        let mut set = $crate::alloc::collections::BTreeSet::new();
        $(set.insert($value.to_owned());)+
        set
    }};

    { $($value:expr_2021),+ $(,)? } => {{
        let mut set = $crate::alloc::collections::BTreeSet::new();
        $(set.insert($value);)+
        set
    }};
}

/// Creates an allocated `LinkedList` from values, appending each value to the
/// end of the list.
///
/// The `owned:` form calls `to_owned()` on each value.
///
/// ```
/// use collections::list;
///
/// let items = list! { "first" => "second" };
/// assert_eq!(items.len(), 2);
/// let owned = list! { owned: "first" => "second" };
/// assert_eq!(owned.front().unwrap(), "first");
/// ```
#[macro_export]
macro_rules! list {
    {} => {
        $crate::alloc::collections::LinkedList::new()
    };

    { owned: $($value:expr_2021)=>+ } => {{
        use $crate::alloc::borrow::ToOwned as _;

        let mut list = $crate::alloc::collections::LinkedList::new();
        $(list.push_back($value.to_owned());)+
        list
    }};

    { $($value:expr_2021)=>+ } => {{
        let mut list = $crate::alloc::collections::LinkedList::new();
        $(list.push_back($value);)+
        list
    }};
}

/// Creates a `HashMap` from key-value pairs. Available with the `std` feature.
///
/// The `owned:` form calls `to_owned()` on each key and value.
///
/// ```
/// use collections::map;
///
/// let map = map! { 1 => "one", 2 => "two" };
/// assert_eq!(map.len(), 2);
/// let owned = map! { owned: "key" => "value" };
/// assert_eq!(owned["key"], "value");
/// ```
#[cfg(feature = "std")]
#[macro_export]
macro_rules! map {
    {} => {
        $crate::std::collections::HashMap::new()
    };

    { owned: $($key:expr_2021 => $value:expr_2021),+ $(,)? } => {{
        use $crate::alloc::borrow::ToOwned as _;

        let mut map = $crate::std::collections::HashMap::new();
        $(map.insert($key.to_owned(), $value.to_owned());)+
        map
    }};

    { $($key:expr_2021 => $value:expr_2021),+ $(,)? } => {{
        let mut map = $crate::std::collections::HashMap::new();
        $(map.insert($key, $value);)+
        map
    }};
}

/// Creates a `HashSet` from values. Available with the `std` feature.
///
/// Repeated values are stored once. The `owned:` form calls `to_owned()` on
/// each value.
///
/// ```
/// use collections::set;
///
/// assert_eq!(set![1, 2, 3, 1], set![1, 2, 3]);
/// let owned = set! { owned: "one", "two" };
/// assert!(owned.contains("one"));
/// ```
#[cfg(feature = "std")]
#[macro_export]
macro_rules! set {
    {} => {
        $crate::std::collections::HashSet::new()
    };

    { owned: $($value:expr_2021),+ $(,)? } => {{
        use $crate::alloc::borrow::ToOwned as _;

        let mut set = $crate::std::collections::HashSet::new();
        $(set.insert($value.to_owned());)+
        set
    }};

    { $($value:expr_2021),+ $(,)? } => {{
        let mut set = $crate::std::collections::HashSet::new();
        $(set.insert($value);)+
        set
    }};
}
