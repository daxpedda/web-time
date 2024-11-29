//! Utility types and functions.

#[cfg(not(target_family = "wasm"))]
mod std;
#[cfg(target_family = "wasm")]
mod web;

#[cfg(target_family = "wasm")]
use tests_web as _;
use web_time::Duration;

#[cfg(not(target_family = "wasm"))]
#[allow(
	clippy::allow_attributes,
	unused_imports,
	reason = "not used by all tests"
)]
pub(crate) use self::std::*;
#[cfg(target_family = "wasm")]
#[allow(
	clippy::allow_attributes,
	unused_imports,
	reason = "not used by all tests"
)]
pub(crate) use self::web::*;

wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

/// Difference to measure that time has passed.
pub(crate) const DIFF: Duration = Duration::from_millis(50);
/// Maximum difference that can't have been passed by [`DIFF`].
#[allow(clippy::allow_attributes, dead_code, reason = "not used by all tests")]
pub(crate) const MAX_DIFF: Duration = if let Some(duration) = DIFF.checked_mul(10) {
	duration
} else {
	panic!()
};
