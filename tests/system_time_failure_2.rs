//! Failure tests have to be separated as `should_panic` can cause serious
//! problems with `panic = "abort"`.

#![cfg(test)]
#![cfg_attr(target_arch = "wasm32", no_main)]
#![cfg_attr(all(target_arch = "wasm32", not(feature = "std")), no_std)]

mod util;

use wasm_bindgen_test::wasm_bindgen_test;
use web_time::{Duration, SystemTime};

use self::util::{sleep, WAIT};

/// [`SystemTime::add_assign()`] failure.
#[wasm_bindgen_test(unsupported = pollster::test)]
#[should_panic = "overflow when adding duration to instant"]
async fn add_assign_failure() {
	sleep(WAIT).await;
	let mut time = SystemTime::now();
	time += Duration::MAX;
}
