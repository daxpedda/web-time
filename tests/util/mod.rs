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

#[macro_export]
macro_rules! test {
	($($test:item)*) => {
		$(
			#[cfg_attr(
				not(all(
					target_family = "wasm",
					not(any(target_os = "emscripten", target_os = "wasi"))
				)),
				pollster::test
			)]
			#[cfg_attr(
				all(
					target_family = "wasm",
					not(any(target_os = "emscripten", target_os = "wasi"))
				),
				wasm_bindgen_test::wasm_bindgen_test
			)]
			#[allow(
				clippy::cognitive_complexity,
				clippy::semicolon_if_nothing_returned,
				clippy::unchecked_duration_subtraction,
				clippy::unused_async
			)]
			$test
		)*
	};
}
