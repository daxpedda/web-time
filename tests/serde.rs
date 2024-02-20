#![cfg(test)]
#![cfg(all(target_family = "wasm", feature = "serde"))]

use wasm_bindgen_test::wasm_bindgen_test;
use web_time::SystemTime;

wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
/// (De)serialization of [`SystemTime`].
fn system_time() {
	let time = SystemTime::now();
	let serialized = serde_json::to_string(&time).unwrap();
	let deserialized: SystemTime = serde_json::from_str(&serialized).unwrap();
	assert_eq!(time, deserialized);
}

#[wasm_bindgen_test]
/// (De)serialization of [`SystemTime`] with [`UNIX_EPOCH`].
fn unix_epoch() {
	let time = SystemTime::UNIX_EPOCH;
	let serialized = serde_json::to_string(&time).unwrap();
	let deserialized: SystemTime = serde_json::from_str(&serialized).unwrap();
	assert_eq!(time, deserialized);
}

#[wasm_bindgen_test]
/// (De)serialization compatibility with [`std::time::SystemTime`].
fn std_compatibility() {
	let time = SystemTime::now();
	let serialized = serde_json::to_string(&time).unwrap();
	let deserialized: std::time::SystemTime = serde_json::from_str(&serialized).unwrap();
	assert_eq!(
		time.duration_since(SystemTime::UNIX_EPOCH).unwrap(),
		deserialized.duration_since(std::time::UNIX_EPOCH).unwrap()
	);

	let time = std::time::SystemTime::UNIX_EPOCH + std::time::Duration::from_secs(1_000_000);
	let serialized = serde_json::to_string(&time).unwrap();
	let deserialized: SystemTime = serde_json::from_str(&serialized).unwrap();
	assert_eq!(
		time.duration_since(std::time::UNIX_EPOCH).unwrap(),
		deserialized.duration_since(SystemTime::UNIX_EPOCH).unwrap()
	);
}
