#![cfg(test)]

mod util;

use std::time::{Duration, SystemTime as StdSystemTime};

use serde_test::Token;
use web_time::SystemTime;

test! {
	/// De/Serialization of [`SystemTime`].
	async fn system_time_json() {
		let time = SystemTime::now();
		let serialized = serde_json::to_string(&time).unwrap();
		let deserialized: SystemTime = serde_json::from_str(&serialized).unwrap();
		assert_eq!(time, deserialized);
	}

	/// De/Serialization of [`SystemTime`] with
	/// [`UNIX_EPOCH`](SystemTime::UNIX_EPOCH).
	async fn unix_epoch_json() {
		let time = SystemTime::UNIX_EPOCH;
		let serialized = serde_json::to_string(&time).unwrap();
		let deserialized: SystemTime = serde_json::from_str(&serialized).unwrap();
		assert_eq!(time, deserialized);
	}

	/// De/Serialization compatibility with [`std::time::SystemTime`].
	async fn std_compatibility_json() {
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
	async fn sequence() {
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
	async fn map() {
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
	async fn failure_sequence() {
		serde_test::assert_de_tokens_error::<SystemTime>(
			&[
				Token::Seq { len: Some(0) },
				Token::SeqEnd,
			],
			"invalid length 0, expected struct SystemTime",
		);

		serde_test::assert_de_tokens_error::<SystemTime>(
			&[
				Token::Seq { len: Some(1) },
				Token::Unit,
			],
			"invalid type: unit value, expected u64",
		);

		serde_test::assert_de_tokens_error::<SystemTime>(
			&[
				Token::Seq { len: Some(1) },
				Token::U64(0),
				Token::SeqEnd,
			],
			"invalid length 1, expected struct SystemTime",
		);

		serde_test::assert_de_tokens_error::<SystemTime>(
			&[
				Token::Seq { len: Some(2) },
				Token::U64(0),
				Token::Unit,
			],
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
	async fn failure_map() {
		serde_test::assert_de_tokens_error::<SystemTime>(
			&[
				Token::Map { len: Some(1) },
				Token::Unit,
			],
			"invalid type: unit value, expected `secs_since_epoch` or `nanos_since_epoch`",
		);

		serde_test::assert_de_tokens_error::<SystemTime>(
			&[
				Token::Map { len: Some(1) },
				Token::Str("test"),
			],
			"unknown field `test`, expected `secs_since_epoch` or `nanos_since_epoch`",
		);

		serde_test::assert_de_tokens_error::<SystemTime>(
			&[
				Token::Map { len: Some(1) },
				Token::Bytes(b"test"),
			],
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
	async fn failure_serialize() {
		let mut serialized = [0; 0];
		let error = serde_json::to_writer(serialized.as_mut(), &SystemTime::UNIX_EPOCH).unwrap_err();

		assert_eq!(error.to_string(), "failed to write whole buffer");

		let mut serialized = [0; 1];
		let error = serde_json::to_writer(serialized.as_mut(), &SystemTime::UNIX_EPOCH).unwrap_err();

		assert_eq!(error.to_string(), "failed to write whole buffer");

		let mut serialized = [0; 21];
		let error = serde_json::to_writer(serialized.as_mut(), &SystemTime::UNIX_EPOCH).unwrap_err();

		assert_eq!(error.to_string(), "failed to write whole buffer");
	}
}
