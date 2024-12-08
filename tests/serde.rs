//! [`serde`] tests for [`SystemTime`].

#![cfg(test)]
#![cfg_attr(target_arch = "wasm32", no_main)]
#![cfg_attr(all(target_arch = "wasm32", not(feature = "std")), no_std)]

extern crate alloc;

mod util;

use alloc::string::ToString;
#[cfg(feature = "std")]
use std::time::{Duration, SystemTime as StdSystemTime};

use serde_test::Token;
use wasm_bindgen_test::wasm_bindgen_test;
use web_time::SystemTime;

/// De/Serialization of [`SystemTime`].
#[wasm_bindgen_test(unsupported = test)]
fn system_time_json() {
	let time = SystemTime::now();
	let serialized = serde_json::to_string(&time).unwrap();
	let deserialized: SystemTime = serde_json::from_str(&serialized).unwrap();
	assert_eq!(time, deserialized);
}

/// De/Serialization of [`SystemTime`] with
/// [`UNIX_EPOCH`](SystemTime::UNIX_EPOCH).
#[wasm_bindgen_test(unsupported = test)]
fn unix_epoch_json() {
	let time = SystemTime::UNIX_EPOCH;
	let serialized = serde_json::to_string(&time).unwrap();
	let deserialized: SystemTime = serde_json::from_str(&serialized).unwrap();
	assert_eq!(time, deserialized);
}

/// De/Serialization compatibility with [`std::time::SystemTime`].
#[cfg(feature = "std")]
#[wasm_bindgen_test(unsupported = test)]
fn std_compatibility_json() {
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

/// Deserialization from a sequence.
#[wasm_bindgen_test(unsupported = test)]
fn sequence() {
	serde_test::assert_de_tokens::<SystemTime>(
		&SystemTime::UNIX_EPOCH,
		&[
			Token::Seq { len: Some(2) },
			Token::U64(0),
			Token::U32(0),
			Token::SeqEnd,
		],
	);
}

/// Deserialization from a map.
#[wasm_bindgen_test(unsupported = test)]
fn map() {
	serde_test::assert_de_tokens::<SystemTime>(
		&SystemTime::UNIX_EPOCH,
		&[
			Token::Map { len: Some(2) },
			Token::Str("secs_since_epoch"),
			Token::U64(0),
			Token::Str("nanos_since_epoch"),
			Token::U32(0),
			Token::MapEnd,
		],
	);

	serde_test::assert_de_tokens::<SystemTime>(
		&SystemTime::UNIX_EPOCH,
		&[
			Token::Map { len: Some(2) },
			Token::Bytes(b"secs_since_epoch"),
			Token::U64(0),
			Token::Bytes(b"nanos_since_epoch"),
			Token::U32(0),
			Token::MapEnd,
		],
	);
}

/// Deserialization failures from a sequence.
#[wasm_bindgen_test(unsupported = test)]
fn failure_sequence() {
	serde_test::assert_de_tokens_error::<SystemTime>(
		&[Token::Seq { len: Some(0) }, Token::SeqEnd],
		"invalid length 0, expected struct SystemTime",
	);

	serde_test::assert_de_tokens_error::<SystemTime>(
		&[Token::Seq { len: Some(1) }, Token::Unit],
		"invalid type: unit value, expected u64",
	);

	serde_test::assert_de_tokens_error::<SystemTime>(
		&[Token::Seq { len: Some(1) }, Token::U64(0), Token::SeqEnd],
		"invalid length 1, expected struct SystemTime",
	);

	serde_test::assert_de_tokens_error::<SystemTime>(
		&[Token::Seq { len: Some(2) }, Token::U64(0), Token::Unit],
		"invalid type: unit value, expected u32",
	);

	serde_test::assert_de_tokens_error::<SystemTime>(
		&[
			Token::Seq { len: Some(2) },
			Token::U64(u64::MAX),
			Token::U32(u32::MAX),
			Token::SeqEnd,
		],
		"overflow deserializing SystemTime epoch offset",
	);
}

/// Deserialization failures from a map.
#[wasm_bindgen_test(unsupported = test)]
fn failure_map() {
	serde_test::assert_de_tokens_error::<SystemTime>(
		&[Token::Map { len: Some(1) }, Token::Unit],
		"invalid type: unit value, expected `secs_since_epoch` or `nanos_since_epoch`",
	);

	serde_test::assert_de_tokens_error::<SystemTime>(
		&[Token::Map { len: Some(1) }, Token::Str("test")],
		"unknown field `test`, expected `secs_since_epoch` or `nanos_since_epoch`",
	);

	serde_test::assert_de_tokens_error::<SystemTime>(
		&[Token::Map { len: Some(1) }, Token::Bytes(b"test")],
		"unknown field `test`, expected `secs_since_epoch` or `nanos_since_epoch`",
	);

	serde_test::assert_de_tokens_error::<SystemTime>(
		&[
			Token::Map { len: Some(2) },
			Token::Str("secs_since_epoch"),
			Token::U64(0),
			Token::Str("secs_since_epoch"),
		],
		"duplicate field `secs_since_epoch`",
	);

	serde_test::assert_de_tokens_error::<SystemTime>(
		&[
			Token::Map { len: Some(2) },
			Token::Str("nanos_since_epoch"),
			Token::U64(0),
			Token::Str("nanos_since_epoch"),
		],
		"duplicate field `nanos_since_epoch`",
	);

	serde_test::assert_de_tokens_error::<SystemTime>(
		&[
			Token::Map { len: Some(1) },
			Token::Str("nanos_since_epoch"),
			Token::U64(0),
			Token::MapEnd,
		],
		"missing field `secs_since_epoch`",
	);

	serde_test::assert_de_tokens_error::<SystemTime>(
		&[
			Token::Map { len: Some(1) },
			Token::Str("secs_since_epoch"),
			Token::U64(0),
			Token::MapEnd,
		],
		"missing field `nanos_since_epoch`",
	);

	serde_test::assert_de_tokens_error::<SystemTime>(
		&[
			Token::Map { len: Some(1) },
			Token::Str("secs_since_epoch"),
			Token::Unit,
			Token::MapEnd,
		],
		"invalid type: unit value, expected u64",
	);

	serde_test::assert_de_tokens_error::<SystemTime>(
		&[
			Token::Map { len: Some(1) },
			Token::Str("nanos_since_epoch"),
			Token::Unit,
			Token::MapEnd,
		],
		"invalid type: unit value, expected u32",
	);

	serde_test::assert_de_tokens_error::<SystemTime>(
		&[
			Token::Map { len: Some(2) },
			Token::Str("secs_since_epoch"),
			Token::U64(u64::MAX),
			Token::Str("nanos_since_epoch"),
			Token::U32(u32::MAX),
			Token::MapEnd,
		],
		"overflow deserializing SystemTime epoch offset",
	);
}

/// Serializing failures.
#[wasm_bindgen_test(unsupported = test)]
fn failure_serialize() {
	let mut serialized = [0; 0];
	let error =
		serde_json_core::to_slice(&SystemTime::UNIX_EPOCH, serialized.as_mut()).unwrap_err();

	assert_eq!(error.to_string(), "Buffer is full");

	let mut serialized = [0; 1];
	let error =
		serde_json_core::to_slice(&SystemTime::UNIX_EPOCH, serialized.as_mut()).unwrap_err();

	assert_eq!(error.to_string(), "Buffer is full");

	let mut serialized = [0; 21];
	let error =
		serde_json_core::to_slice(&SystemTime::UNIX_EPOCH, serialized.as_mut()).unwrap_err();

	assert_eq!(error.to_string(), "Buffer is full");
}
