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
	/// [`Instant::add()`] failure.
	#[should_panic]
	async fn add_failure() {
		sleep(DIFF).await;
		let _ = Instant::now() + Duration::MAX;
	}

	/// [`Instant::add_assign()`] failure.
	#[should_panic]
	async fn add_assign_failure() {
		sleep(DIFF).await;
		let mut instant = Instant::now();
		instant += Duration::MAX;
	}

	/// [`Instant::sub()`] failure.
	#[should_panic]
	async fn sub_failure() {
		let _ = Instant::now() - Duration::MAX;
	}

	/// [`Instant::sub_assign()`] failure.
	#[should_panic]
	async fn sub_assign_failure() {
		let mut instant = Instant::now();
		instant -= Duration::MAX;
	}
}
