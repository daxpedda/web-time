//! Serde serialization and deserialization for `SystemTime`.
//! It aims to be compatible with the `serde`'s implementation for
//! `std::time::SystemTime`.
//! This module is only available when the `serde` feature is enabled.
//!
//! The `Serialize` implementation was copied from [`serde`'s
//! upstream](https://github.com/serde-rs/serde/blob/5fa711d75d91173aafc6019e03cf8af6ac9ba7b2/serde/src/ser/impls.rs#L730-L747),
//! as well as [the `Deserialize`
//! one](https://github.com/serde-rs/serde/blob/5fa711d75d91173aafc6019e03cf8af6ac9ba7b2/serde/src/de/impls.rs#L2168-L2314)

use std::fmt::{self, Formatter};
use std::time::Duration;

use super::SystemTime;

impl serde::Serialize for SystemTime {
	fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
		use serde::ser::SerializeStruct;

		// CHANGED: we already have the `Duration` available in `self.0`
		let duration_since_epoch = self.0;
		let mut state = serializer.serialize_struct("SystemTime", 2)?;
		state.serialize_field("secs_since_epoch", &duration_since_epoch.as_secs())?;
		state.serialize_field("nanos_since_epoch", &duration_since_epoch.subsec_nanos())?;
		state.end()
	}
}

impl<'de> serde::Deserialize<'de> for SystemTime {
	fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
		use serde::de::{Error, MapAccess, SeqAccess, Visitor};

		// Reuse duration
		enum Field {
			Secs,
			Nanos,
		}

		impl<'de> serde::Deserialize<'de> for Field {
			fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
			where
				D: serde::Deserializer<'de>,
			{
				struct FieldVisitor;

				// CHANGED: using '_ lifetime to avoid warning with recent compilers
				impl Visitor<'_> for FieldVisitor {
					type Value = Field;

					fn expecting(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
						formatter.write_str("`secs_since_epoch` or `nanos_since_epoch`")
					}

					fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
					where
						E: Error,
					{
						match value {
							"secs_since_epoch" => Ok(Field::Secs),
							"nanos_since_epoch" => Ok(Field::Nanos),
							_ => Err(Error::unknown_field(value, FIELDS)),
						}
					}

					fn visit_bytes<E>(self, value: &[u8]) -> Result<Self::Value, E>
					where
						E: Error,
					{
						match value {
							b"secs_since_epoch" => Ok(Field::Secs),
							b"nanos_since_epoch" => Ok(Field::Nanos),
							_ => {
								let value = String::from_utf8_lossy(value);
								Err(Error::unknown_field(&value, FIELDS))
							}
						}
					}
				}

				deserializer.deserialize_identifier(FieldVisitor)
			}
		}

		fn check_overflow<E>(secs: u64, nanos: u32) -> Result<(), E>
		where
			E: Error,
		{
			static NANOS_PER_SEC: u32 = 1_000_000_000;
			match secs.checked_add((nanos / NANOS_PER_SEC) as u64) {
				Some(_) => Ok(()),
				None => Err(E::custom("overflow deserializing SystemTime epoch offset")),
			}
		}

		struct DurationVisitor;

		impl<'de> Visitor<'de> for DurationVisitor {
			type Value = Duration;

			fn expecting(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
				formatter.write_str("struct SystemTime")
			}

			fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
			where
				A: SeqAccess<'de>,
			{
				let secs: u64 = match seq.next_element()? {
					Some(value) => value,
					None => {
						return Err(Error::invalid_length(0, &self));
					}
				};
				let nanos: u32 = match seq.next_element()? {
					Some(value) => value,
					None => {
						return Err(Error::invalid_length(1, &self));
					}
				};
				check_overflow(secs, nanos)?;
				Ok(Duration::new(secs, nanos))
			}

			fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
			where
				A: MapAccess<'de>,
			{
				let mut secs: Option<u64> = None;
				let mut nanos: Option<u32> = None;
				while let Some(key) = map.next_key()? {
					match key {
						Field::Secs => {
							if secs.is_some() {
								return Err(<A::Error as Error>::duplicate_field(
									"secs_since_epoch",
								));
							}
							secs = Some(map.next_value()?);
						}
						Field::Nanos => {
							if nanos.is_some() {
								return Err(<A::Error as Error>::duplicate_field(
									"nanos_since_epoch",
								));
							}
							nanos = Some(map.next_value()?);
						}
					}
				}
				let secs = match secs {
					Some(secs) => secs,
					None => return Err(<A::Error as Error>::missing_field("secs_since_epoch")),
				};
				let nanos = match nanos {
					Some(nanos) => nanos,
					None => return Err(<A::Error as Error>::missing_field("nanos_since_epoch")),
				};
				check_overflow(secs, nanos)?;
				Ok(Duration::new(secs, nanos))
			}
		}

		const FIELDS: &[&str] = &["secs_since_epoch", "nanos_since_epoch"];
		let duration = deserializer.deserialize_struct("SystemTime", FIELDS, DurationVisitor)?;
		// CHANGED: we can put the duration directly here, and do not have to go through
		// addition
		let ret = Ok(SystemTime(duration));
		ret
	}
}
