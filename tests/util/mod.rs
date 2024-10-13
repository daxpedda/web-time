#[cfg(not(target_family = "wasm"))]
mod std;
#[cfg(target_family = "wasm")]
mod web;

use web_time::Duration;

#[cfg(not(target_family = "wasm"))]
#[allow(unused)]
pub(crate) use self::std::*;
#[cfg(target_family = "wasm")]
#[allow(unused)]
pub(crate) use self::web::*;

pub(crate) const DIFF: Duration = Duration::from_millis(50);
#[allow(dead_code)]
pub(crate) const MAX_DIFF: Duration = if let Some(duration) = DIFF.checked_mul(10) {
	duration
} else {
	panic!()
};
