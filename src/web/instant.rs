//! Re-implementation of [`std::time::Instant`].

use std::ops::{Add, AddAssign, Sub, SubAssign};
use std::time::Duration;

use super::js::PERFORMANCE;

#[cfg(target_feature = "atomics")]
thread_local! {
	static ORIGIN: f64 = PERFORMANCE.with(super::js::Performance::time_origin);
}

/// See [`std::time::Instant`].
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Instant(Duration);

impl Instant {
	/// See [`std::time::Instant::now()`].
	#[must_use]
	pub fn now() -> Self {
		let now = PERFORMANCE.with(|performance| {
			#[cfg(target_feature = "atomics")]
			return ORIGIN.with(|origin| performance.now() + origin);

			#[cfg(not(target_feature = "atomics"))]
			performance.now()
		});

		Self(time_stamp_to_duration(now))
	}

	/// See [`std::time::Instant::duration_since()`].
	#[must_use]
	pub fn duration_since(&self, earlier: Self) -> Duration {
		self.checked_duration_since(earlier).unwrap_or_default()
	}

	/// See [`std::time::Instant::checked_duration_since()`].
	#[must_use]
	#[allow(clippy::missing_const_for_fn)]
	pub fn checked_duration_since(&self, earlier: Self) -> Option<Duration> {
		self.0.checked_sub(earlier.0)
	}

	/// See [`std::time::Instant::saturating_duration_since()`].
	#[must_use]
	pub fn saturating_duration_since(&self, earlier: Self) -> Duration {
		self.checked_duration_since(earlier).unwrap_or_default()
	}

	/// See [`std::time::Instant::elapsed()`].
	#[must_use]
	pub fn elapsed(&self) -> Duration {
		Self::now() - *self
	}

	/// See [`std::time::Instant::checked_add()`].
	pub fn checked_add(&self, duration: Duration) -> Option<Self> {
		self.0.checked_add(duration).map(Instant)
	}

	/// See [`std::time::Instant::checked_sub()`].
	pub fn checked_sub(&self, duration: Duration) -> Option<Self> {
		self.0.checked_sub(duration).map(Instant)
	}
}

impl Add<Duration> for Instant {
	type Output = Self;

	/// # Panics
	///
	/// This function may panic if the resulting point in time cannot be
	/// represented by the underlying data structure. See
	/// [`Instant::checked_add`] for a version without panic.
	fn add(self, other: Duration) -> Self {
		self.checked_add(other)
			.expect("overflow when adding duration to instant")
	}
}

impl AddAssign<Duration> for Instant {
	fn add_assign(&mut self, other: Duration) {
		*self = *self + other;
	}
}

impl Sub<Duration> for Instant {
	type Output = Self;

	fn sub(self, other: Duration) -> Self {
		self.checked_sub(other)
			.expect("overflow when subtracting duration from instant")
	}
}

impl Sub<Self> for Instant {
	type Output = Duration;

	/// Returns the amount of time elapsed from another instant to this one,
	/// or zero duration if that instant is later than this one.
	fn sub(self, other: Self) -> Duration {
		self.duration_since(other)
	}
}

impl SubAssign<Duration> for Instant {
	fn sub_assign(&mut self, other: Duration) {
		*self = *self - other;
	}
}

/// Converts a `DOMHighResTimeStamp` to a [`Duration`].
///
/// # Note
///
/// This is trying to emulate exactly what [`Duration::from_secs_f64()`] does,
/// but accounting for the conversion from milliseconds instead of seconds which
/// also makes it slightly more expensive.
///
/// Keep in mind that like [`Duration::from_secs_f64()`] this doesn't do perfect
/// rounding.
#[allow(
	clippy::as_conversions,
	clippy::cast_possible_truncation,
	clippy::min_ident_chars,
	clippy::missing_docs_in_private_items
)]
fn time_stamp_to_duration(time_stamp: f64) -> Duration {
	// CHANGED: 1G to 1M.
	const NANOS_PER_MILLI: u32 = 1_000_000;

	// See <https://doc.rust-lang.org/1.73.0/src/core/time.rs.html#1477-1484>
	const MANT_BITS: i16 = 52;
	const EXP_BITS: i16 = 11;
	const OFFSET: i16 = 44;

	// See <https://doc.rust-lang.org/1.73.0/src/core/time.rs.html#1262-1340>
	const MIN_EXP: i16 = 1 - (1_i16 << EXP_BITS) / 2;
	const MANT_MASK: u64 = (1 << MANT_BITS) - 1;
	const EXP_MASK: u64 = (1 << 1_i16 << EXP_BITS) - 1;

	assert!(
		time_stamp >= 0.0,
		"can not convert float seconds to Duration: value is negative"
	);

	let bits = time_stamp.to_bits();
	let mant = (bits & MANT_MASK) | (MANT_MASK + 1);
	let exp = ((bits >> MANT_BITS) & EXP_MASK) as i16 + MIN_EXP;

	// CHANGED: 31 to 21 bits, because the fractional part only handles
	// microseconds, not milliseconds.
	let (millis, mut nanos) = if exp < -21 {
		// the input represents less than 1ns and can not be rounded to it
		// CHANGED: Return early.
		return Duration::ZERO;
	} else if exp < 0 {
		// the input is less than 1 millisecond
		let t = <u128>::from(mant) << (OFFSET + exp);
		let nanos_offset = MANT_BITS + OFFSET;
		let nanos_tmp = u128::from(NANOS_PER_MILLI) * t;
		let nanos = (nanos_tmp >> nanos_offset) as u32;

		let rem_mask = (1 << nanos_offset) - 1;
		let rem_msb_mask = 1 << (nanos_offset - 1);
		let rem = nanos_tmp & rem_mask;
		let is_tie = rem == rem_msb_mask;
		let is_even = (nanos & 1) == 0;
		let rem_msb = nanos_tmp & rem_msb_mask == 0;
		let add_ns = !(rem_msb || (is_even && is_tie));

		let nanos = nanos + u32::from(add_ns);
		// CHANGED: Removed `f32` handling.
		// CHANGED: Return early.
		#[allow(clippy::if_not_else)]
		return if nanos != NANOS_PER_MILLI {
			Duration::new(0, nanos)
		} else {
			// CHANGED: Do second to millisecond conversion right here because of the early
			// return.
			Duration::from_millis(1)
		};
	} else if exp < MANT_BITS {
		let millis = mant >> (MANT_BITS - exp);
		let t = <u128>::from((mant << exp) & MANT_MASK);
		let nanos_offset = MANT_BITS;
		let nanos_tmp = <u128>::from(NANOS_PER_MILLI) * t;
		let nanos = (nanos_tmp >> nanos_offset) as u32;

		let rem_mask = (1 << nanos_offset) - 1;
		let rem_msb_mask = 1 << (nanos_offset - 1);
		let rem = nanos_tmp & rem_mask;
		let is_tie = rem == rem_msb_mask;
		let is_even = (nanos & 1) == 0;
		let rem_msb = nanos_tmp & rem_msb_mask == 0;
		let add_ns = !(rem_msb || (is_even && is_tie));

		let nanos = nanos + u32::from(add_ns);
		// CHANGED: Removed `f32` handling.
		#[allow(clippy::if_not_else)]
		if nanos != NANOS_PER_MILLI {
			(millis, nanos)
		} else {
			(millis + 1, 0)
		}
	}
	// NOTE: Theoretically milliseconds can be bigger then `u64` when trying to cover
	// `Duration::MAX`, but `Performance.now` is a monotonic clock, so we don't need to cover that.
	else if exp < 64 {
		// the input has no fractional part
		let millis = mant << (exp - MANT_BITS);
		(millis, 0)
	} else {
		panic!("can not convert float seconds to Duration: value is either too big or NaN")
	};

	let secs = millis / 1000;
	let carry = millis - secs * 1000;
	let extra_nanos = carry * u64::from(NANOS_PER_MILLI);
	nanos += extra_nanos as u32;

	debug_assert!(
		nanos < 1_000_000_000,
		"impossible amount of nanoseconds found"
	);

	Duration::new(secs, nanos)
}

#[cfg(test)]
mod test {
	use std::time::Duration;

	use rand::distributions::Uniform;
	use rand::Rng;
	use wasm_bindgen_test::wasm_bindgen_test;

	wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

	// According to <https://www.w3.org/TR/2023/WD-hr-time-3-20230719/#introduction>.
	const MAXIMUM_ACCURATE_SECS: u64 = 285_616 * 365 * 24 * 60 * 60;
	#[allow(clippy::as_conversions, clippy::cast_precision_loss)]
	const MAXIMUM_ACCURATE_MILLIS: f64 = MAXIMUM_ACCURATE_SECS as f64 * 1_000.;

	#[derive(Debug)]
	struct ControlDuration(Duration);

	impl ControlDuration {
		fn new(time_stamp: f64) -> Self {
			// See <https://doc.rust-lang.org/1.73.0/src/core/time.rs.html#657-668>.
			let time_stamp = Duration::from_secs_f64(time_stamp);
			let secs = time_stamp.as_secs() / 1000;
			let carry = time_stamp.as_secs() - secs * 1000;
			#[allow(clippy::as_conversions, clippy::cast_possible_truncation)]
			let extra_nanos = (carry * 1_000_000_000 / 1000) as u32;
			// CHANGED: Added rounding.
			let nanos = time_stamp.subsec_micros()
				+ u32::from(time_stamp.subsec_nanos() % 1000 > 499)
				+ extra_nanos;
			// CHANGED: Removed check that would fail because of the additional time added
			// by rounding.
			Self(Duration::new(secs, nanos))
		}
	}

	impl PartialEq<Duration> for ControlDuration {
		fn eq(&self, duration: &Duration) -> bool {
			// Our control `Duration` has perfect accuracy, unlike
			// [`super::time_stamp_to_duration()`].
			if self.0 == *duration {
				true
			} else if let Some(diff) = self.0.checked_sub(*duration) {
				diff == Duration::from_nanos(1)
			} else {
				false
			}
		}
	}

	#[wasm_bindgen_test]
	fn sanity() {
		#[track_caller]
		fn assert(time_stamp: f64, result: Duration) {
			let control = ControlDuration::new(time_stamp);
			let duration = super::time_stamp_to_duration(time_stamp);

			assert_eq!(control, result, "control and expected result are different");
			assert_eq!(control, duration);
		}

		assert(0.000_000, Duration::ZERO);
		assert(0.000_000_4, Duration::ZERO);
		assert(0.000_000_5, Duration::from_nanos(1));
		assert(0.000_001, Duration::from_nanos(1));
		assert(0.000_001_4, Duration::from_nanos(1));
		assert(0.000_001_5, Duration::from_nanos(2));
		assert(0.999_999, Duration::from_nanos(999_999));
		assert(0.999_999_4, Duration::from_nanos(999_999));
		assert(0.999_999_5, Duration::from_millis(1));
		assert(1., Duration::from_millis(1));
		assert(1.000_000_4, Duration::from_millis(1));
		assert(1.000_000_5, Duration::from_nanos(1_000_001));
		assert(1.000_001, Duration::from_nanos(1_000_001));
		assert(1.000_001_4, Duration::from_nanos(1_000_001));
		assert(1.000_001_5, Duration::from_nanos(1_000_002));
		assert(999.999_999, Duration::from_nanos(999_999_999));
		assert(999.999_999_4, Duration::from_nanos(999_999_999));
		assert(999.999_999_5, Duration::from_secs(1));
		assert(1000., Duration::from_secs(1));
		assert(1_000.000_000_4, Duration::from_secs(1));
		assert(1_000.000_000_5, Duration::from_nanos(1_000_000_001));
		assert(1_000.000_001, Duration::from_nanos(1_000_000_001));
		assert(1_000.000_001_4, Duration::from_nanos(1_000_000_001));
		assert(1_000.000_001_5, Duration::from_nanos(1_000_000_002));
		assert(
			MAXIMUM_ACCURATE_MILLIS,
			Duration::from_secs(MAXIMUM_ACCURATE_SECS),
		);
	}

	#[wasm_bindgen_test]
	fn fuzzing() {
		let mut random =
			rand::thread_rng().sample_iter(Uniform::new_inclusive(0., MAXIMUM_ACCURATE_MILLIS));

		for _ in 0..10_000_000 {
			let time_stamp = random.next().unwrap();

			let control = ControlDuration::new(time_stamp);
			let duration = super::time_stamp_to_duration(time_stamp);

			assert_eq!(control, duration);
		}
	}
}
