// Copyright Kani Contributors
// SPDX-License-Identifier: Apache-2.0 OR MIT

// This test checks that values returned by `kani::slice::any_slice` satisfy the
// type invariant

// kani-flags: --default-unwind 4

extern crate kani;
use kani::slice::{any_slice, AnySlice};

#[kani::proof]
fn check_any_slice_valid() {
    let s: AnySlice<char, 3> = any_slice();
    for c in s.get_slice() {
        kani::assume(*c != char::MAX);
        assert!(*c < char::MAX);
    }
}
