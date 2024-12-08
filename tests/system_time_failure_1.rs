//! Failure tests have to be separated as `should_panic` can cause serious
//! problems with `panic = "abort"`.

#![cfg(test)]
#![cfg_attr(target_arch = "wasm32", no_main)]
#![cfg_attr(all(target_arch = "wasm32", not(feature = "std")), no_std)]

mod util;

use wasm_bindgen_test::wasm_bindgen_test;
use web_time::{Duration, SystemTime};

use self::util::{sleep, WAIT};

/// [`SystemTime::add()`] failure.
#[wasm_bindgen_test(unsupported = pollster::test)]
#[should_panic = "overflow when adding duration to instant"]
async fn add_failure() {
	sleep(WAIT).await;
	let _ = SystemTime::now() + Duration::MAX;
}

/// [`SystemTime::sub()`] failure.
#[wasm_bindgen_test(unsupported = test)]
#[should_panic = "overflow when subtracting duration from instant"]
fn sub_failure() {
	let _ = SystemTime::now() - Duration::MAX;
}

/// [`SystemTime::sub_assign()`] failure.
#[wasm_bindgen_test(unsupported = test)]
#[should_panic = "overflow when subtracting duration from instant"]
fn sub_assign_failure() {
	let time = SystemTime::now();
	let _ = time - Duration::MAX;
}
