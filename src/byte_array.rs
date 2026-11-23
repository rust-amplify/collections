// SPDX-License-Identifier: Apache-2.0
//
// Written in 2020-2026 by Dr. Maxim Orlovsky <orlovsky@ubideco.org>
//
// Copyright 2020-2026 Laboratories for Ubiquitous and Deterministic Computing,
// Institute for Distributed and Cognitive Computing (InDCS), Switzerland.
// All rights reserved.
//
// Copyright (C) 2020-2026 Dr Maxim Orlovsky.
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

use core::fmt::{self, Display, Formatter};

/// Error when slice size mismatches array length.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct FromSliceError {
    /// Expected slice length.
    pub expected: usize,
    /// Actual slice length.
    pub actual: usize,
}

impl Display for FromSliceError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "the provided slice length {} doesn't match array size {}",
            self.actual, self.expected
        )
    }
}

impl core::error::Error for FromSliceError {}

/// Trait for fixed-size byte array types and wrappers around them.
pub trait ByteArray<const LEN: usize>: Sized {
    /// Constructs a value from a byte array.
    fn from_byte_array(val: impl Into<[u8; LEN]>) -> Self;

    /// Constructs a byte array value from a slice.
    ///
    /// Errors if the slice length doesn't match the `LEN` constant generic.
    fn from_slice(slice: impl AsRef<[u8]>) -> Result<Self, FromSliceError>;

    /// Constructs a byte array value from a slice, panicking if the slice
    /// length doesn't match the `LEN` constant generic.
    fn from_slice_checked(slice: impl AsRef<[u8]>) -> Self {
        Self::from_slice(slice).expect("slice length not matching type requirements")
    }

    /// Returns the fixed-size byte array representation of `self`.
    fn to_byte_array(&self) -> [u8; LEN];
}

impl<const LEN: usize> ByteArray<LEN> for [u8; LEN] {
    #[inline]
    fn from_byte_array(val: impl Into<[u8; LEN]>) -> Self {
        val.into()
    }

    #[inline]
    fn from_slice(slice: impl AsRef<[u8]>) -> Result<Self, FromSliceError> {
        let slice = slice.as_ref();
        Self::try_from(slice).map_err(|_| FromSliceError {
            expected: LEN,
            actual: slice.len(),
        })
    }

    #[inline]
    fn to_byte_array(&self) -> [u8; LEN] {
        *self
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn raw_array_implements_byte_array() {
        let arr = <[u8; 4]>::from_byte_array([1, 2, 3, 4]);
        assert_eq!(arr.to_byte_array(), [1, 2, 3, 4]);
        assert_eq!(<[u8; 4]>::from_slice([1, 2, 3, 4]), Ok(arr));
        assert_eq!(<[u8; 4]>::from_slice_checked([1, 2, 3, 4]), arr);
        assert_eq!(
            <[u8; 4]>::from_slice([1, 2, 3]),
            Err(FromSliceError {
                expected: 4,
                actual: 3,
            })
        );
    }

    #[test]
    #[should_panic(expected = "slice length not matching type requirements")]
    fn from_slice_checked_panics_on_mismatch() {
        let _ = <[u8; 4]>::from_slice_checked([1, 2]);
    }
}
