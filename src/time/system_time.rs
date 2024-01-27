//! Re-implementation of [`std::time::SystemTime`].

use std::error::Error;
use std::fmt::{self, Display, Formatter};
use std::ops::{Add, AddAssign, Sub, SubAssign};
use std::time::Duration;

/// See [`std::time::SystemTime`].
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct SystemTime(pub(crate) Duration);

impl SystemTime {
	/// See [`std::time::SystemTime::UNIX_EPOCH`].
	pub const UNIX_EPOCH: Self = Self(Duration::ZERO);

	/// See [`std::time::SystemTime::now()`].
	#[must_use]
	#[allow(clippy::missing_panics_doc)]
	pub fn now() -> Self {
		#[allow(clippy::as_conversions, clippy::cast_possible_truncation)]
		let ms = js_sys::Date::now() as i64;
		let ms = ms.try_into().expect("found negative timestamp");

		Self(Duration::from_millis(ms))
	}

	/// See [`std::time::SystemTime::duration_since()`].
	#[allow(clippy::missing_errors_doc, clippy::trivially_copy_pass_by_ref)]
	pub fn duration_since(&self, earlier: Self) -> Result<Duration, SystemTimeError> {
		if self.0 < earlier.0 {
			Err(SystemTimeError(earlier.0 - self.0))
		} else {
			Ok(self.0 - earlier.0)
		}
	}

	/// See [`std::time::SystemTime::elapsed()`].
	#[allow(clippy::missing_errors_doc, clippy::trivially_copy_pass_by_ref)]
	pub fn elapsed(&self) -> Result<Duration, SystemTimeError> {
		Self::now().duration_since(*self)
	}

	/// See [`std::time::SystemTime::checked_add()`].
	#[allow(clippy::trivially_copy_pass_by_ref)]
	pub fn checked_add(&self, duration: Duration) -> Option<Self> {
		self.0.checked_add(duration).map(SystemTime)
	}

	/// See [`std::time::SystemTime::checked_sub()`].
	#[allow(clippy::trivially_copy_pass_by_ref)]
	pub fn checked_sub(&self, duration: Duration) -> Option<Self> {
		self.0.checked_sub(duration).map(SystemTime)
	}
}

impl Add<Duration> for SystemTime {
	type Output = Self;

	/// # Panics
	///
	/// This function may panic if the resulting point in time cannot be
	/// represented by the underlying data structure. See
	/// [`SystemTime::checked_add`] for a version without panic.
	fn add(self, dur: Duration) -> Self {
		self.checked_add(dur)
			.expect("overflow when adding duration to instant")
	}
}

impl AddAssign<Duration> for SystemTime {
	fn add_assign(&mut self, other: Duration) {
		*self = *self + other;
	}
}

impl Sub<Duration> for SystemTime {
	type Output = Self;

	fn sub(self, dur: Duration) -> Self {
		self.checked_sub(dur)
			.expect("overflow when subtracting duration from instant")
	}
}

impl SubAssign<Duration> for SystemTime {
	fn sub_assign(&mut self, other: Duration) {
		*self = *self - other;
	}
}

#[cfg(feature = "serde")]
impl serde::Serialize for SystemTime {
	fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
		use serde::ser::{Error, SerializeStruct};

		let duration_since_epoch = match self.duration_since(SystemTime::UNIX_EPOCH) {
			Ok(duration_since_epoch) => duration_since_epoch,
			Err(_) => return Err(S::Error::custom("SystemTime must be later than UNIX_EPOCH")),
		};
		let mut state = serializer.serialize_struct("SystemTime", 2)?;
		state.serialize_field("secs_since_epoch", &duration_since_epoch.as_secs())?;
		state.serialize_field("nanos_since_epoch", &duration_since_epoch.subsec_nanos())?;
		state.end()
	}
}

#[cfg(feature = "serde")]
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
		#[cfg(not(no_systemtime_checked_add))]
		let ret = SystemTime::UNIX_EPOCH
			.checked_add(duration)
			.ok_or_else(|| D::Error::custom("overflow deserializing SystemTime"));
		#[cfg(no_systemtime_checked_add)]
		let ret = Ok(UNIX_EPOCH + duration);
		ret
	}
}

/// See [`std::time::SystemTimeError`].
#[derive(Clone, Debug)]
#[allow(missing_copy_implementations)]
pub struct SystemTimeError(Duration);

impl SystemTimeError {
	/// See [`std::time::SystemTimeError::duration()`].
	#[must_use]
	#[allow(clippy::missing_const_for_fn)]
	pub fn duration(&self) -> Duration {
		self.0
	}
}

impl Display for SystemTimeError {
	fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
		write!(formatter, "second time provided was later than self")
	}
}

impl Error for SystemTimeError {}

#[cfg(all(test, feature = "serde"))]
mod tests {
	use wasm_bindgen_test::wasm_bindgen_test;

	use super::*;

	wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

	#[wasm_bindgen_test]
	fn test_serde() {
		let unix_epoch = SystemTime::UNIX_EPOCH;
		let serialized = serde_json::to_string(&unix_epoch).unwrap();
		let deserialized: SystemTime = serde_json::from_str(&serialized).unwrap();
		assert_eq!(unix_epoch, deserialized);

		let now = SystemTime::now();
		let serialized = serde_json::to_string(&now).unwrap();
		let deserialized: SystemTime = serde_json::from_str(&serialized).unwrap();
		assert_eq!(now, deserialized);
	}
}
