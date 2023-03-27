#[cfg(not(all(
	target_family = "wasm",
	not(any(target_os = "emscripten", target_os = "wasi"))
)))]
mod std;
#[cfg(all(
	target_family = "wasm",
	not(any(target_os = "emscripten", target_os = "wasi"))
))]
mod web;

use web_time::Duration;

#[cfg(not(all(
	target_family = "wasm",
	not(any(target_os = "emscripten", target_os = "wasi"))
)))]
pub(crate) use self::std::*;
#[cfg(all(
	target_family = "wasm",
	not(any(target_os = "emscripten", target_os = "wasi"))
))]
pub(crate) use self::web::*;

pub(crate) const DIFF: Duration = Duration::from_millis(10);
