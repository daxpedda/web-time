//! Failure tests have to be separated as `should_panic` can cause serious
//! problems with `panic = "abort"`.

#![cfg(test)]

mod util;

use web_time::{Duration, Instant};

use self::util::{sleep, DIFF};

#[cfg(all(
	target_family = "wasm",
	not(any(target_os = "emscripten", target_os = "wasi"))
))]
wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

test! {
	/// [`Instant::add_assign()`] failure.
	#[should_panic]
	async fn add_assign_failure() {
		sleep(DIFF).await;
		let mut instant = Instant::now();
		instant += Duration::MAX;
	}
}
