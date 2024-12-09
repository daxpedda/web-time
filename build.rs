//! [`rustversion`] can't be used in inner attributes, so we use `cfg`s as a
//! workaround.
//!
//! See <https://github.com/rust-lang/rust/issues/54726>.

#![allow(clippy::missing_const_for_fn)]

#[cfg(feature = "msrv")]
fn main() {
	v1_77();
	nightly();
}

/// Enabling [`f64::round_ties_even()`].
#[cfg(feature = "msrv")]
#[rustversion::since(1.77)]
fn v1_77() {
	println!("cargo:rustc-cfg=v1_77");
}

#[cfg(feature = "msrv")]
#[rustversion::before(1.77)]
fn v1_77() {}

/// Enabling various [`f64`] instructions via [`asm!`](std::arch::asm).
#[cfg(feature = "msrv")]
#[rustversion::nightly]
fn nightly() {
	println!("cargo:rustc-cfg=nightly");
}

#[cfg(feature = "msrv")]
#[rustversion::not(nightly)]
fn nightly() {}

#[cfg(not(feature = "msrv"))]
fn main() {}
