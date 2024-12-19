//! Re-implementation of [`std::time::Instant`].
//!
//! See <https://github.com/rust-lang/rust/blob/1.83.0/library/std/src/time.rs#L271-L468>.
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

		assert!(
			now.is_sign_positive(),
			"negative `DOMHighResTimeStamp`s are not supported"
		);
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
		+ Duration::from_nanos(F64(time_stamp.fract() * 1.0e6).internal_round_ties_even() as u64)
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

	#[cfg(all(not(feature = "std"), nightly))]
	fn trunc(self) -> f64 {
		let output;
		// SAFETY: No side effects.
		#[allow(unsafe_code)]
		unsafe {
			core::arch::asm! {
				"local.get {}",
				"f64.trunc",
				"local.set {}",
				in(local) self.0,
				lateout(local) output,
				options(pure, nomem),
			};
		}

		output
	}

	#[cfg(all(not(feature = "std"), not(nightly)))]
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

	/// See [`f64::round_ties_even()`].
	#[allow(clippy::disallowed_methods)]
	#[cfg(all(feature = "std", v1_77))]
	fn internal_round_ties_even(self) -> f64 {
		#[allow(clippy::incompatible_msrv)]
		self.0.round_ties_even()
	}

	#[cfg(all(not(all(feature = "std", v1_77)), nightly))]
	fn internal_round_ties_even(self) -> f64 {
		let output;
		// SAFETY: No side effects.
		#[allow(unsafe_code)]
		unsafe {
			core::arch::asm! {
				"local.get {}",
				"f64.nearest",
				"local.set {}",
				in(local) self.0,
				lateout(local) output,
				options(pure, nomem),
			};
		}

		output
	}

	/// A specialized version of [`f64::round_ties_even()`]. [`f64`] must be
	/// positive and have an exponent smaller than `52`.
	///
	/// - We expect `DOMHighResTimeStamp` to always be positive. We check that
	///   in [`Instant::now()`].
	/// - We only round the fractional part after multiplying it by `1e6`. A
	///   fraction always has a negative exponent. `1e6` has an exponent of
	///   `19`. Therefor the resulting exponent can at most be `19`.
	///
	/// [`f64::round_ties_even()`]: https://doc.rust-lang.org/1.83.0/std/primitive.f64.html#method.round_ties_even
	#[cfg(not(any(all(feature = "std", v1_77), nightly)))]
	fn internal_round_ties_even(self) -> f64 {
		/// Put `debug_assert!` in a function to clap `coverage(off)` on it.
		///
		/// See <https://github.com/rust-lang/rust/issues/80549>.
		#[cfg_attr(web_time_test_coverage, coverage(off))]
		fn check(this: f64) {
			debug_assert!(this.is_sign_positive(), "found negative input");
			debug_assert!(
				{
					let exponent: u64 = this.to_bits() >> 52 & 0x7ff;
					exponent < 0x3ff + 52
				},
				"found number with exponent bigger than 51"
			);
		}

		check(self.0);

		// See <https://github.com/rust-lang/libm/blob/libm-v0.2.11/src/math/rint.rs>.

		let one_over_e = 1.0 / f64::EPSILON;
		// REMOVED: We don't support numbers with exponents bigger than 51.
		// REMOVED: We don't support negative numbers.
		// REMOVED: We don't support numbers with exponents bigger than 51.
		let xplusoneovere = self.0 + one_over_e;
		xplusoneovere - one_over_e
		// REMOVED: We don't support negative numbers.
	}
}

#[cfg(test)]
#[cfg_attr(web_time_test_coverage, coverage(off))]
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
		/// Implements perfect but expensive conversion from
		/// `DOMHighResTimeStamp` to [`Duration`].
		fn new(time_stamp: f64) -> Self {
			// Inspired by <https://github.com/rust-lang/rust/blob/1.83.0/library/core/src/time.rs#L822-L833>.
			/// Number of nanoseconds in a second.
			const NANOS_PER_SEC: u64 = 1_000_000_000;
			let rhs: u32 = 1000;
			let time_stamp = Duration::from_secs_f64(time_stamp);
			let (secs, extra_secs) = (
				time_stamp.as_secs() / u64::from(rhs),
				time_stamp.as_secs() % u64::from(rhs),
			);
			let (mut nanos, extra_nanos) = (
				time_stamp.subsec_nanos() / rhs,
				time_stamp.subsec_nanos() % rhs,
			);
			// CHANGED: Extracted part of the calculation into variable.
			let extra = extra_secs * NANOS_PER_SEC + u64::from(extra_nanos);
			nanos += u32::try_from(extra / u64::from(rhs)).unwrap();
			// CHANGED: Added rounding.
			nanos += u32::from(extra % u64::from(rhs) >= u64::from(rhs / 2));
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
