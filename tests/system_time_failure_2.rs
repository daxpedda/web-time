//! Failure tests have to be separated as `should_panic` can cause serious
//! problems with `panic = "abort"`.

#![cfg(test)]

mod util;

use web_time::{Duration, SystemTime};

use self::util::{sleep, DIFF};

#[cfg(all(
	target_family = "wasm",
	not(any(target_os = "emscripten", target_os = "wasi"))
))]
wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

test! {
	/// [`SystemTime::add_assign()`] failure.
	#[should_panic = "overflow when adding duration to instant"]
	async fn add_assign_failure() {
		sleep(DIFF).await;
		let mut time = SystemTime::now();
		time += Duration::MAX;
	}
}
