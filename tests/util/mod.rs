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

#[macro_export]
macro_rules! test {
	($($test:item)*) => {
		#[cfg(target_family = "wasm")]
		wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

		$(
			#[cfg_attr(
				not(target_family = "wasm"),
				pollster::test
			)]
			#[cfg_attr(
				target_family = "wasm",
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
