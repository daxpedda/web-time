//! Test Web-specific API exported in [`web_time::web`].

#![cfg(test)]
#![cfg(target_family = "wasm")]

use std::time;
use std::time::Duration;

use wasm_bindgen_test::wasm_bindgen_test;
use web_time::web::SystemTimeExt;

wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn to_std() {
	assert_eq!(
		web_time::SystemTime::UNIX_EPOCH.to_std(),
		time::SystemTime::UNIX_EPOCH,
	);

	let duration = Duration::from_secs(60 * 60 * 24 * 365);
	assert_eq!(
		(web_time::SystemTime::UNIX_EPOCH + duration).to_std(),
		time::SystemTime::UNIX_EPOCH + duration,
	);
}

#[wasm_bindgen_test]
fn from_std() {
	assert_eq!(
		web_time::SystemTime::from_std(time::SystemTime::UNIX_EPOCH),
		web_time::SystemTime::UNIX_EPOCH,
	);

	let duration = Duration::from_secs(60 * 60 * 24 * 365);
	assert_eq!(
		web_time::SystemTime::from_std(time::SystemTime::UNIX_EPOCH + duration),
		web_time::SystemTime::UNIX_EPOCH + duration,
	);
}
