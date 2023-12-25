#[cfg(not(all(target_family = "wasm", target_os = "unknown")))]
mod std;
#[cfg(all(target_family = "wasm", target_os = "unknown"))]
mod web;

use web_time::Duration;

#[cfg(not(all(target_family = "wasm", target_os = "unknown")))]
pub(crate) use self::std::*;
#[cfg(all(target_family = "wasm", target_os = "unknown"))]
pub(crate) use self::web::*;

pub(crate) const DIFF: Duration = Duration::from_millis(50);

#[macro_export]
macro_rules! test {
	($($test:item)*) => {
		#[cfg(all(target_family = "wasm", target_os = "unknown"))]
		wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

		$(
			#[cfg_attr(
				not(all(target_family = "wasm", target_os = "unknown")),
				pollster::test
			)]
			#[cfg_attr(
				all(target_family = "wasm", target_os = "unknown"),
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
