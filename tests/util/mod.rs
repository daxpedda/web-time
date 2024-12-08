//! Utility types and functions.

#[cfg(not(target_arch = "wasm32"))]
mod std;
#[cfg(target_arch = "wasm32")]
mod web;

#[cfg(target_arch = "wasm32")]
use tests_web as _;
use web_time::Duration;

#[cfg(not(target_arch = "wasm32"))]
#[allow(
	clippy::allow_attributes,
	unused_imports,
	reason = "not used by all tests"
)]
pub(crate) use self::std::*;
#[cfg(target_arch = "wasm32")]
#[allow(
	clippy::allow_attributes,
	unused_imports,
	reason = "not used by all tests"
)]
pub(crate) use self::web::*;

/// Time to wait.
pub(crate) const WAIT: Duration = Duration::from_millis(50);
/// Difference to measure that time has passed.
#[allow(clippy::allow_attributes, dead_code, reason = "not used by all tests")]
pub(crate) const DIFF: Duration = Duration::from_millis(49);
/// Maximum difference that can't have been passed by [`DIFF`].
#[allow(clippy::allow_attributes, dead_code, reason = "not used by all tests")]
pub(crate) const MAX_DIFF: Duration = if let Some(duration) = WAIT.checked_mul(10) {
	duration
} else {
	panic!()
};
