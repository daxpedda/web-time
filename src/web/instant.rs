//! Re-implementation of [`std::time::Instant`].

use std::ops::{Add, AddAssign, Sub, SubAssign};
use std::time::Duration;

use super::js::PERFORMANCE;

#[cfg(target_feature = "atomics")]
thread_local! {
	static ORIGIN: f64 = PERFORMANCE.with(|performance| performance.time_origin());
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

		#[allow(
			clippy::as_conversions,
			clippy::cast_possible_truncation,
			clippy::cast_sign_loss
		)]
		let duration = Duration::from_millis(now.trunc() as u64)
			+ Duration::from_nanos((now.fract() * 1.0e6) as u64);

		Self(duration)
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
