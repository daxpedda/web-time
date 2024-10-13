//! Failure tests have to be separated as `should_panic` can cause serious
//! problems with `panic = "abort"`.

#![cfg(test)]

mod util;

use wasm_bindgen_test::wasm_bindgen_test;
use web_time::{Duration, Instant};

use self::util::{sleep, DIFF};

wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

/// [`Instant::add()`] failure.
#[wasm_bindgen_test(unsupported = pollster::test)]
#[should_panic = "overflow when adding duration to instant"]
async fn add_failure() {
	sleep(DIFF).await;
	let _ = Instant::now() + Duration::MAX;
}

/// [`Instant::sub()`] failure.
#[wasm_bindgen_test(unsupported = test)]
#[allow(clippy::unchecked_duration_subtraction)]
#[should_panic = "overflow when subtracting duration from instant"]
fn sub_failure() {
	let _ = Instant::now() - Duration::MAX;
}

/// [`Instant::sub_assign()`] failure.
#[wasm_bindgen_test(unsupported = test)]
#[should_panic = "overflow when subtracting duration from instant"]
fn sub_assign_failure() {
	let mut instant = Instant::now();
	instant -= Duration::MAX;
}
