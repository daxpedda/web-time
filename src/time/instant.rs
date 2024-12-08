//! Re-implementation of [`std::time::Instant`].
#![cfg_attr(
	not(feature = "std"),
	doc = "",
	doc = "[`std::time::Instant`]: https://doc.rust-lang.org/std/time/struct.Instant.html"
)]

use core::ops::{Add, AddAssign, Sub, SubAssign};
use core::time::Duration;

use super::js::PERFORMANCE;
#[cfg(target_feature = "atomics")]
use super::js::TIME_ORIGIN;

/// See [`std::time::Instant`].
#[cfg_attr(
	not(feature = "std"),
	doc = "",
	doc = "[`std::time::Instant`]: https://doc.rust-lang.org/std/time/struct.Instant.html"
)]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Instant(Duration);

impl Instant {
	/// See [`std::time::Instant::now()`].
	///
	/// # Panics
	///
	/// This call will panic if the [`Performance` object] was not found, e.g.
	/// calling from a [worklet].
	///
	/// [`Performance` object]: https://developer.mozilla.org/en-US/docs/Web/API/performance_property
	/// [worklet]: https://developer.mozilla.org/en-US/docs/Web/API/Worklet
	#[cfg_attr(
		not(feature = "std"),
		doc = "[`std::time::Instant::now()`]: https://doc.rust-lang.org/std/time/struct.Instant.html#method.now"
	)]
	#[must_use]
	pub fn now() -> Self {
		let now = PERFORMANCE.with(|performance| {
			let performance = performance
				.as_ref()
				.expect("`Performance` object not found");

			#[cfg(not(target_feature = "atomics"))]
			return performance.now();
			#[cfg(target_feature = "atomics")]
			TIME_ORIGIN.with(|origin| performance.now() + origin)
		});

		Self(time_stamp_to_duration(now))
	}

	/// See [`std::time::Instant::duration_since()`].
	#[cfg_attr(
		not(feature = "std"),
		doc = "",
		doc = "[`std::time::Instant::duration_since()`]: https://doc.rust-lang.org/std/time/struct.Instant.html#method.duration_since"
	)]
	#[must_use]
	pub fn duration_since(&self, earlier: Self) -> Duration {
		self.checked_duration_since(earlier).unwrap_or_default()
	}

	/// See [`std::time::Instant::checked_duration_since()`].
	#[cfg_attr(
		not(feature = "std"),
		doc = "",
		doc = "[`std::time::Instant::checked_duration_since()`]: https://doc.rust-lang.org/std/time/struct.Instant.html#method.checked_duration_since"
	)]
	#[must_use]
	#[allow(clippy::missing_const_for_fn)]
	pub fn checked_duration_since(&self, earlier: Self) -> Option<Duration> {
		self.0.checked_sub(earlier.0)
	}

	/// See [`std::time::Instant::saturating_duration_since()`].
	#[cfg_attr(
		not(feature = "std"),
		doc = "",
		doc = "[`std::time::Instant::saturating_duration_since()`]: https://doc.rust-lang.org/std/time/struct.Instant.html#method.saturating_duration_since"
	)]
	#[must_use]
	pub fn saturating_duration_since(&self, earlier: Self) -> Duration {
		self.checked_duration_since(earlier).unwrap_or_default()
	}

	/// See [`std::time::Instant::elapsed()`].
	#[cfg_attr(
		not(feature = "std"),
		doc = "",
		doc = "[`std::time::Instant::elapsed()`]: https://doc.rust-lang.org/std/time/struct.Instant.html#method.elapsed"
	)]
	#[must_use]
	pub fn elapsed(&self) -> Duration {
		Self::now() - *self
	}

	/// See [`std::time::Instant::checked_add()`].
	#[cfg_attr(
		not(feature = "std"),
		doc = "",
		doc = "[`std::time::Instant::checked_add()`]: https://doc.rust-lang.org/std/time/struct.Instant.html#method.checked_add"
	)]
	pub fn checked_add(&self, duration: Duration) -> Option<Self> {
		self.0.checked_add(duration).map(Instant)
	}

	/// See [`std::time::Instant::checked_sub()`].
	#[cfg_attr(
		not(feature = "std"),
		doc = "",
		doc = "[`std::time::Instant::checked_sub()`]: https://doc.rust-lang.org/std/time/struct.Instant.html#method.checked_sub"
	)]
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
	fn add(self, rhs: Duration) -> Self {
		self.checked_add(rhs)
			.expect("overflow when adding duration to instant")
	}
}

impl AddAssign<Duration> for Instant {
	fn add_assign(&mut self, rhs: Duration) {
		*self = *self + rhs;
	}
}

impl Sub<Duration> for Instant {
	type Output = Self;

	fn sub(self, rhs: Duration) -> Self {
		self.checked_sub(rhs)
			.expect("overflow when subtracting duration from instant")
	}
}

impl Sub<Self> for Instant {
	type Output = Duration;

	/// Returns the amount of time elapsed from another instant to this one,
	/// or zero duration if that instant is later than this one.
	fn sub(self, rhs: Self) -> Duration {
		self.duration_since(rhs)
	}
}

impl SubAssign<Duration> for Instant {
	fn sub_assign(&mut self, rhs: Duration) {
		*self = *self - rhs;
	}
}

/// Converts a `DOMHighResTimeStamp` to a [`Duration`].
///
/// # Note
///
/// Keep in mind that like [`Duration::from_secs_f64()`] this doesn't do perfect
/// rounding.
#[allow(
	clippy::as_conversions,
	clippy::cast_possible_truncation,
	clippy::cast_sign_loss
)]
fn time_stamp_to_duration(time_stamp: f64) -> Duration {
	let time_stamp = F64(time_stamp);

	Duration::from_millis(time_stamp.trunc() as u64)
		+ Duration::from_nanos(F64(time_stamp.fract() * 1.0e6).round() as u64)
}

/// [`f64`] `no_std` compatibility wrapper.
#[derive(Clone, Copy)]
struct F64(f64);

impl F64 {
	/// See [`f64::trunc()`].
	#[cfg(feature = "std")]
	#[allow(clippy::disallowed_methods)]
	fn trunc(self) -> f64 {
		self.0.trunc()
	}

	#[cfg(not(feature = "std"))]
	#[allow(warnings)]
	fn trunc(self) -> f64 {
		// See <https://github.com/rust-lang/libm/blob/libm-v0.2.11/src/math/trunc.rs>.

		let x1p120 = f64::from_bits(0x4770000000000000); // `0x1p120f === 2 ^ 120`

		let mut i: u64 = self.0.to_bits();
		let mut e: i64 = (i >> 52 & 0x7ff) as i64 - 0x3ff + 12;
		let m: u64;

		if e >= 52 + 12 {
			return self.0;
		}
		if e < 12 {
			e = 1;
		}
		m = -1i64 as u64 >> e;
		if (i & m) == 0 {
			return self.0;
		}
		#[allow(unsafe_code)]
		unsafe {
			::core::ptr::read_volatile(&(self.0 + x1p120))
		};
		i &= !m;
		f64::from_bits(i)
	}

	/// See [`f64::fract()`].
	#[cfg(feature = "std")]
	#[allow(clippy::disallowed_methods)]
	fn fract(self) -> f64 {
		self.0.fract()
	}

	#[cfg(not(feature = "std"))]
	fn fract(self) -> f64 {
		self.0 - self.trunc()
	}

	/// See [`f64::copysign()`].
	///
	/// [`f64::copysign()`]: https://doc.rust-lang.org/std/primitive.f64.html#method.copysign
	#[cfg(not(feature = "std"))]
	fn copysign(self, sign: f64) -> f64 {
		// See <https://github.com/rust-lang/libm/blob/libm-v0.2.11/src/math/copysign.rs>.

		let mut ux = self.0.to_bits();
		let uy = sign.to_bits();
		ux &= (!0) >> 1;
		ux |= uy & (1 << 63);
		f64::from_bits(ux)
	}

	/// See [`f64::round()`].
	#[cfg(feature = "std")]
	#[allow(clippy::disallowed_methods)]
	fn round(self) -> f64 {
		self.0.round()
	}

	#[cfg(not(feature = "std"))]
	fn round(self) -> f64 {
		// See <https://github.com/rust-lang/libm/blob/libm-v0.2.11/src/math/round.rs>.

		Self(self.0 + Self(0.5 - 0.25 * f64::EPSILON).copysign(self.0)).trunc()
	}
}

#[cfg(test)]
#[cfg_attr(wasm_bindgen_unstable_test_coverage, coverage(off))]
mod test {
	//! Testing internal code.

	use core::time::Duration;

	use rand::distributions::Uniform;
	use rand::rngs::{OsRng, StdRng};
	use rand::{Rng, SeedableRng};
	use wasm_bindgen_test::wasm_bindgen_test;

	/// Range to maximum accurately representable integer.
	const MAXIMUM_ACCURATE_F64: u64 = u64::pow(2, f64::MANTISSA_DIGITS);

	/// [`Duration`] wrapper to simulate [`std`] behavior.
	#[derive(Debug)]
	struct ControlDuration(Duration);

	impl ControlDuration {
		/// Implements conversion from `DOMHighResTimeStamp` to [`Duration`] by
		/// using [`Duration::checked_div()`].
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
		fn eq(&self, other: &Duration) -> bool {
			// Our control `Duration` has perfect accuracy, unlike
			// [`super::time_stamp_to_duration()`].
			if self.0 == *other {
				true
			} else if let Some(diff) = self.0.checked_sub(*other) {
				diff == Duration::from_nanos(1)
			} else {
				false
			}
		}
	}

	/// Compare [`super::time_stamp_to_duration()`] against a pre-determined set
	/// of [`Durations`]s.
	#[wasm_bindgen_test]
	fn sanity() {
		/// Do the comparison for this test.
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
		#[expect(
			clippy::as_conversions,
			clippy::cast_precision_loss,
			reason = "no conversion available"
		)]
		assert(
			MAXIMUM_ACCURATE_F64 as f64,
			Duration::from_secs(MAXIMUM_ACCURATE_F64) / 1000,
		);
	}

	/// Compare [`super::time_stamp_to_duration()`] against random
	/// [`Duration`]s.
	#[wasm_bindgen_test]
	fn fuzzing() {
		#[expect(
			clippy::as_conversions,
			clippy::cast_precision_loss,
			reason = "no conversion available"
		)]
		let mut random = StdRng::from_rng(OsRng)
			.unwrap()
			.sample_iter(Uniform::new_inclusive(
				0.,
				(MAXIMUM_ACCURATE_F64 / 1000) as f64,
			));

		for _ in 0..10_000_000 {
			let time_stamp = random.next().unwrap();

			let control = ControlDuration::new(time_stamp);
			let duration = super::time_stamp_to_duration(time_stamp);

			assert_eq!(control, duration);
		}
	}
}
