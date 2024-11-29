//! A crate for running tests on native with the default test harness.

#![cfg_attr(all(target_family = "wasm", not(feature = "std")), no_std)]
#![cfg_attr(all(test, target_family = "wasm"), no_main)]

#[cfg(all(test, target_family = "wasm"))]
use tests_web as _;

#[cfg(all(test, not(target_family = "wasm")))]
fn main() {}
