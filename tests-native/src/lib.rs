//! A crate for running tests on native with the default test harness.

#![cfg_attr(all(target_arch = "wasm32", not(feature = "std")), no_std)]
#![cfg_attr(all(test, target_arch = "wasm32"), no_main)]

#[cfg(all(test, target_arch = "wasm32"))]
use tests_web as _;

#[cfg(all(test, not(target_arch = "wasm32")))]
fn main() {}
