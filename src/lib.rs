//! # Description
//!
//! Complete drop-in replacement for [`std::time`] that works in browsers.
//!
//! Currently [`Instant::now()`] and [`SystemTime::now()`] will simply panic
//! when using the `wasm32-unknown-unknown` target. This implementation uses
//! [`Performance.now()`] for [`Instant`] and [`Date.now()`] for [`SystemTime`]
//! to offer a drop-in replacement that works in browsers.
//!
//! At the same time the library will simply re-export [`std::time`] when not
//! using the `wasm32-unknown-unknown` or `wasm32v1-none` target and will not
//! pull in any dependencies.
//!
//! Additionally, if compiled with `target-feature = "atomics"` it will
//! synchronize the timestamps to account for different context's, like web
//! workers. See [`Performance.timeOrigin`] for more information.
//!
//! # Target
//!
//! This library specifically targets browsers, that support
//! [`Performance.now()`], with the `wasm32-unknown-unknown` or `wasm32v1-none`
//! target. Emscripten is not supported. WASI doesn't require support as it has
//! it's own native API to deal with [`std::time`].
//!
//! Furthermore it depends on [`wasm-bindgen`], which is required. This library
//! will continue to depend on it until a viable alternative presents itself, in
//! which case multiple ecosystems could be supported.
//!
//! # Note
//!
//! ## Ticking during sleep
//!
//! Currently a known bug is affecting browsers on operating system other then
//! Windows. This bug prevents [`Instant`] from continuing to tick when the
//! context is asleep. While this doesn't conflict with Rusts requirements of
//! [`Instant`], by chance Rust's Std
//! [has the same problem](https://github.com/rust-lang/rust/issues/79462).
//!
//! See [the MDN documentation on this](https://developer.mozilla.org/en-US/docs/Web/API/Performance/now#ticking_during_sleep) for more information.
//!
//! ## Context support
//!
//! The implementation of [`Instant::now()`] relies on the availability of the
//! [`Performance` object], a lack thereof will cause a panic. This can happen
//! if called from a [worklet].
//!
//! # Usage
//!
//! You can simply import the types you need:
//! ```
//! # #![cfg_attr(all(target_arch = "wasm32", not(feature = "std")), no_std, no_main)]
//! #
//! use web_time::{Instant, SystemTime};
//! # #[cfg(target_arch = "wasm32")]
//! # use tests_web as _;
//! #
//! # wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);
//! #
//! # #[wasm_bindgen_test::wasm_bindgen_test]
//! # fn main() {
//! let now = Instant::now();
//! let time = SystemTime::now();
//! # }
//! ```
//!
//! Using `-Ctarget-feature=+nontrapping-fptoint` will improve the performance
//! of [`Instant::now()`] and [`SystemTime::now()`], but the vast majority of
//! the time is still spent going through JS.
//!
//! # Features
//!
//! ## `std` (enabled by default)
//!
//! Enables the corresponding crate feature in all dependencies and allows for
//! some optimized instruction output.
//!
//! Without this crate feature compilation the standard library is not included.
//! Has no effect on targets other then `wasm32-unknown-unknown` or
//! `wasm32v1-none`.
//!
//! ## `serde`
//!
//! Implements [`serde::Deserialize`] and [`serde::Serialize`] for
//! [`SystemTime`].
//!
//! # Conditional Configurations
//!
//! ## `docsrs`
//!
//! This requires Rust nightly and enhances the documentation. It must only be
//! used with `RUSTDOCFLAGS`, not with `RUSTFLAGS`.
//!
//! # MSRV
//!
//! As this library heavily relies on [`wasm-bindgen`] the MSRV depends on it.
//! At the point of time this was written the MSRV is 1.60.
//!
//! # Contributing
//!
//! See the [CONTRIBUTING] file for details.
//!
//! # Attribution
//!
//! Inspiration was taken from the [instant](https://github.com/sebcrozet/instant/tree/v0.1.12) project.
//!
//! Additional insight was taken from the [time](https://github.com/time-rs/time/tree/v0.3.20) project.
//!
//! # Changelog
//!
//! See the [CHANGELOG] file for details.
//!
//! # License
//!
//! Licensed under either of
//!
//! - Apache License, Version 2.0 ([LICENSE-APACHE] or <http://www.apache.org/licenses/LICENSE-2.0>)
//! - MIT license ([LICENSE-MIT] or <http://opensource.org/licenses/MIT>)
//!
//! at your option.
//!
//! ## Copyright
//!
//! A majority of the code and documentation was taken from [`std::time`]. For
//! license information see [#License](https://github.com/rust-lang/rust/tree/1.68.1#license).
//!
//! ## Contribution
//!
//! Unless you explicitly state otherwise, any contribution intentionally
//! submitted for inclusion in the work by you, as defined in the Apache-2.0
//! license, shall be dual licensed as above, without any additional terms or
//! conditions.
//!
//! [CHANGELOG]: https://github.com/daxpedda/web-time/blob/v1.1.0/CHANGELOG.md
//! [CONTRIBUTING]: https://github.com/daxpedda/web-time/blob/v1.1.0/CONTRIBUTING.md
//! [LICENSE-MIT]: https://github.com/daxpedda/web-time/blob/v1.1.0/LICENSE-MIT
//! [LICENSE-APACHE]: https://github.com/daxpedda/web-time/blob/v1.1.0/LICENSE-APACHE
//! [worklet]: https://developer.mozilla.org/en-US/docs/Web/API/Worklet
//! [`Date.now()`]: https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/now
//! [`Instant`]: https://doc.rust-lang.org/std/time/struct.Instant.html
//! [`Instant::now()`]: https://doc.rust-lang.org/std/time/struct.Instant.html#method.now
//! [`SystemTime`]: https://doc.rust-lang.org/std/time/struct.SystemTime.html
//! [`SystemTime::now()`]: https://doc.rust-lang.org/std/time/struct.SystemTime.html#method.now
//! [`std::time`]: https://doc.rust-lang.org/std/time/
//! [`performance.now()`]: https://developer.mozilla.org/en-US/docs/Web/API/Performance/now
//! [`Performance.timeOrigin`]: https://developer.mozilla.org/en-US/docs/Web/API/Performance/timeOrigin
//! [`Performance` object]: https://developer.mozilla.org/en-US/docs/Web/API/performance_property
#![cfg_attr(
	any(not(feature = "serde"), not(target_arch = "wasm32")),
	doc = "[`serde::Deserialize`]: https://docs.rs/serde/1/serde/trait.Deserialize.html",
	doc = "[`serde::Serialize`]: https://docs.rs/serde/1/serde/trait.Serialize.html"
)]
//! [`wasm-bindgen`]: https://crates.io/crates/wasm-bindgen

#![cfg_attr(all(target_arch = "wasm32", not(feature = "std")), no_std)]
#![cfg_attr(all(test, target_arch = "wasm32"), no_main)]
#![cfg_attr(all(doc, docsrs), feature(doc_cfg))]
#![cfg_attr(wasm_bindgen_unstable_test_coverage, feature(coverage_attribute))]
#![cfg_attr(
	all(
		target_arch = "wasm32",
		any(target_os = "unknown", target_os = "none"),
		target_feature = "atomics",
		not(feature = "std"),
	),
	feature(thread_local)
)]

#[cfg(all(target_arch = "wasm32", any(target_os = "unknown", target_os = "none")))]
mod time;
#[cfg(any(
	all(
		target_arch = "wasm32",
		any(target_os = "unknown", target_os = "none"),
		feature = "std"
	),
	all(doc, docsrs)
))]
#[cfg_attr(all(doc, docsrs), doc(cfg(all(Web, feature = "std"))))]
pub mod web;

#[cfg(not(all(target_arch = "wasm32", any(target_os = "unknown", target_os = "none"))))]
pub use std::time::*;

#[cfg(all(test, target_arch = "wasm32"))]
use tests_web as _;

#[cfg(all(target_arch = "wasm32", any(target_os = "unknown", target_os = "none")))]
pub use self::time::*;

#[cfg(all(not(doc), docsrs))]
compile_error!("`--cfg docsrs` must only be used via `RUSTDOCFLAGS`, not `RUSTFLAGS`");

#[cfg(all(test, target_arch = "wasm32"))]
wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

#[cfg(all(test, not(target_arch = "wasm32")))]
fn main() {}
