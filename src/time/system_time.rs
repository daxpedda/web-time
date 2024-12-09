//! Re-implementation of [`std::time::SystemTime`].
//!
//! See <https://github.com/rust-lang/rust/blob/1.83.0/library/std/src/time.rs#L470-L707>.
#![cfg_attr(
	not(feature = "std"),
	doc = "",
	doc = "[`std::time::SystemTime`]: https://doc.rust-lang.org/std/time/struct.SystemTime.html"
)]

#[cfg(all(all(doc, docsrs), not(feature = "std")))]
use core::error::Error;
use core::fmt::{self, Display, Formatter};
use core::ops::{Add, AddAssign, Sub, SubAssign};
use core::time::Duration;
#[cfg(feature = "std")]
use std::error::Error;

use super::js::Date;

/// See [`std::time::SystemTime`].
#[cfg_attr(
	not(feature = "std"),
	doc = "",
	doc = "[`std::time::SystemTime`]: https://doc.rust-lang.org/std/time/struct.SystemTime.html"
)]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct SystemTime(pub(crate) Duration);

impl SystemTime {
	/// See [`std::time::SystemTime::UNIX_EPOCH`].
	#[cfg_attr(
		not(feature = "std"),
		doc = "",
		doc = "[`std::time::SystemTime::UNIX_EPOCH`]: https://doc.rust-lang.org/std/time/struct.SystemTime.html#associatedconstant.UNIX_EPOCH"
	)]
	pub const UNIX_EPOCH: Self = Self(Duration::ZERO);

	/// See [`std::time::SystemTime::now()`].
	#[cfg_attr(
		not(feature = "std"),
		doc = "",
		doc = "[`std::time::SystemTime::now()`]: https://doc.rust-lang.org/std/time/struct.SystemTime.html#method.now"
	)]
	#[must_use]
	#[allow(clippy::missing_panics_doc)]
	pub fn now() -> Self {
		#[allow(clippy::as_conversions, clippy::cast_possible_truncation)]
		let ms = Date::now() as i64;
		let ms = ms.try_into().expect("found negative timestamp");

		Self(Duration::from_millis(ms))
	}

	/// See [`std::time::SystemTime::duration_since()`].
	#[cfg_attr(
		not(feature = "std"),
		doc = "",
		doc = "[`std::time::SystemTime::duration_since()`]: https://doc.rust-lang.org/std/time/struct.SystemTime.html#method.duration_since"
	)]
	#[allow(clippy::missing_errors_doc, clippy::trivially_copy_pass_by_ref)]
	pub fn duration_since(&self, earlier: Self) -> Result<Duration, SystemTimeError> {
		// See <https://github.com/rust-lang/rust/blob/1.83.0/library/std/src/sys/pal/unsupported/time.rs#L34-L36>.
		self.0
			.checked_sub(earlier.0)
			.ok_or_else(|| SystemTimeError(earlier.0 - self.0))
	}

	/// See [`std::time::SystemTime::elapsed()`].
	#[cfg_attr(
		not(feature = "std"),
		doc = "",
		doc = "[`std::time::SystemTime::elapsed()`]: https://doc.rust-lang.org/std/time/struct.SystemTime.html#method.elapsed"
	)]
	#[allow(clippy::missing_errors_doc, clippy::trivially_copy_pass_by_ref)]
	pub fn elapsed(&self) -> Result<Duration, SystemTimeError> {
		Self::now().duration_since(*self)
	}

	/// See [`std::time::SystemTime::checked_add()`].
	#[cfg_attr(
		not(feature = "std"),
		doc = "",
		doc = "[`std::time::SystemTime::checked_add()`]: https://doc.rust-lang.org/std/time/struct.SystemTime.html#method.checked_add"
	)]
	#[allow(clippy::trivially_copy_pass_by_ref)]
	pub fn checked_add(&self, duration: Duration) -> Option<Self> {
		self.0.checked_add(duration).map(SystemTime)
	}

	/// See [`std::time::SystemTime::checked_sub()`].
	#[cfg_attr(
		not(feature = "std"),
		doc = "",
		doc = "[`std::time::SystemTime::checked_sub()`]: https://doc.rust-lang.org/std/time/struct.SystemTime.html#method.checked_sub"
	)]
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
	fn add(self, rhs: Duration) -> Self {
		self.checked_add(rhs)
			.expect("overflow when adding duration to instant")
	}
}

impl AddAssign<Duration> for SystemTime {
	fn add_assign(&mut self, rhs: Duration) {
		*self = *self + rhs;
	}
}

impl Sub<Duration> for SystemTime {
	type Output = Self;

	fn sub(self, rhs: Duration) -> Self {
		self.checked_sub(rhs)
			.expect("overflow when subtracting duration from instant")
	}
}

impl SubAssign<Duration> for SystemTime {
	fn sub_assign(&mut self, rhs: Duration) {
		*self = *self - rhs;
	}
}

/// See [`std::time::SystemTimeError`].
#[cfg_attr(
	not(feature = "std"),
	doc = "",
	doc = "[`std::time::SystemTimeError`]: https://doc.rust-lang.org/std/time/struct.SystemTimeError.html"
)]
#[derive(Clone, Debug)]
#[allow(missing_copy_implementations)]
pub struct SystemTimeError(Duration);

impl SystemTimeError {
	/// See [`std::time::SystemTimeError::duration()`].
	#[cfg_attr(
		not(feature = "std"),
		doc = "",
		doc = "[`std::time::SystemTimeError::duration()`]: https://doc.rust-lang.org/std/time/struct.SystemTimeError.html#method.duration"
	)]
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

#[cfg(any(feature = "std", all(doc, docsrs)))]
#[cfg_attr(all(doc, docsrs), doc(cfg(feature = "std")))]
impl Error for SystemTimeError {}
