#![cfg(test)]
#![cfg(target_family = "wasm")]

use std::time::{Duration, SystemTime as StdSystemTime};

use wasm_bindgen_test::wasm_bindgen_test;
use web_time::SystemTime;

wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

/// De/Serialization of [`SystemTime`].
#[wasm_bindgen_test]
fn system_time() {
	let time = SystemTime::now();
	let serialized = serde_json::to_string(&time).unwrap();
	let deserialized: SystemTime = serde_json::from_str(&serialized).unwrap();
	assert_eq!(time, deserialized);
}

/// De/Serialization of [`SystemTime`] with
/// [`UNIX_EPOCH`](SystemTime::UNIX_EPOCH).
#[wasm_bindgen_test]
fn unix_epoch() {
	let time = SystemTime::UNIX_EPOCH;
	let serialized = serde_json::to_string(&time).unwrap();
	let deserialized: SystemTime = serde_json::from_str(&serialized).unwrap();
	assert_eq!(time, deserialized);
}

#[wasm_bindgen_test]
/// De/Serialization compatibility with [`std::time::SystemTime`].
fn std_compatibility() {
	let time = SystemTime::now();
	let serialized = serde_json::to_string(&time).unwrap();
	let deserialized: StdSystemTime = serde_json::from_str(&serialized).unwrap();
	assert_eq!(
		time.duration_since(SystemTime::UNIX_EPOCH).unwrap(),
		deserialized
			.duration_since(StdSystemTime::UNIX_EPOCH)
			.unwrap()
	);

	let time = StdSystemTime::UNIX_EPOCH + Duration::from_secs(1_000_000);
	let serialized = serde_json::to_string(&time).unwrap();
	let deserialized: SystemTime = serde_json::from_str(&serialized).unwrap();
	assert_eq!(
		time.duration_since(StdSystemTime::UNIX_EPOCH).unwrap(),
		deserialized.duration_since(SystemTime::UNIX_EPOCH).unwrap()
	);
}
