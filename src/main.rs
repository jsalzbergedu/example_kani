// Copyright Kani Contributors
// SPDX-License-Identifier: Apache-2.0 OR MIT
// kani-flags: -Zfunction-contracts

// Test that a modifies clause works when a (function call)
// expression is provided
fn infinite_looper() -> usize {
    loop {};
    1
}

#[kani::requires(arr[idx] == 1)]
#[kani::modifies(&arr[infinite_looper()])]
#[kani::ensures(arr[idx] == 2)]
fn modify(idx: usize, arr: &mut [i64]) {
    arr[idx] += 1;
}

#[kani::proof_for_contract(modify)]
fn main() {
    let mut arr: [i64; 4] = [1, 2, 3, 4];
    modify(1, &mut arr);
}
