//! Platform-specific extensions to [`web-time`](crate) for the Web platform.

#![allow(clippy::absolute_paths)]

use core::time::Duration;
use std::time::SystemTime as StdSystemTime;

use crate::{Instant, SystemTime};

#[cfg(all(
	target_arch = "wasm32",
	any(target_os = "unknown", target_os = "none"),
	not(feature = "std"),
))]
#[doc(hidden)]
mod std {
	pub mod time {
		pub struct SystemTime;
	}
}

/// Web-specific extension to [`web_time::SystemTime`](crate::SystemTime).
pub trait SystemTimeExt {
	/// Convert [`web_time::SystemTime`](crate::SystemTime) to
	/// [`std::time::SystemTime`].
	///
	/// # Note
	///
	/// This might give a misleading impression of compatibility!
	///
	/// Considering this functionality will probably be used to interact with
	/// incompatible APIs of other dependencies, care should be taken that the
	/// dependency in question doesn't call [`std::time::SystemTime::now()`]
	/// internally, which would panic.
	#[cfg_attr(
		all(
			target_arch = "wasm32",
			any(target_os = "unknown", target_os = "none"),
			not(feature = "std"),
		),
		doc = "",
		doc = "[`std::time::SystemTime`]: https://doc.rust-lang.org/std/time/struct.SystemTime.html",
		doc = "[`std::time::SystemTime::now()`]: https://doc.rust-lang.org/std/time/struct.SystemTime.html#method.now"
	)]
	fn to_std(self) -> std::time::SystemTime;

	/// Convert [`std::time::SystemTime`] to
	/// [`web_time::SystemTime`](crate::SystemTime).
	///
	/// # Note
	///
	/// This might give a misleading impression of compatibility!
	///
	/// Considering this functionality will probably be used to interact with
	/// incompatible APIs of other dependencies, care should be taken that the
	/// dependency in question doesn't call [`std::time::SystemTime::now()`]
	/// internally, which would panic.
	#[cfg_attr(
		all(
			target_arch = "wasm32",
			any(target_os = "unknown", target_os = "none"),
			not(feature = "std"),
		),
		doc = "",
		doc = "[`std::time::SystemTime`]: https://doc.rust-lang.org/std/time/struct.SystemTime.html",
		doc = "[`std::time::SystemTime::now()`]: https://doc.rust-lang.org/std/time/struct.SystemTime.html#method.now"
	)]
	fn from_std(time: std::time::SystemTime) -> SystemTime;
}

impl SystemTimeExt for SystemTime {
	fn to_std(self) -> std::time::SystemTime {
		StdSystemTime::UNIX_EPOCH + self.0
	}

	fn from_std(time: std::time::SystemTime) -> SystemTime {
		Self::UNIX_EPOCH
			+ time
				.duration_since(StdSystemTime::UNIX_EPOCH)
				.expect("found `SystemTime` earlier than unix epoch")
	}
}

/// Web-specific extension to [`web_time::Instant`](crate::Instant).
pub trait InstantExt {
	/// Create a [`web_time::Instant`](crate::Instant) from a [`Duration`]
	/// representing the monotonic time elapsed since an arbitrary, fixed
	/// origin.
	///
	/// # Note
	///
	/// The resulting [`Instant`](crate::Instant) is only meaningful relative to
	/// other [`Instant`](crate::Instant)s that share the same origin (whether
	/// obtained via [`Instant::now()`](crate::Instant::now) or this method).
	/// Mixing timestamps from different origins will yield nonsensical
	/// [`Duration`]s.
	///
	/// [`Performance.now()`]: https://developer.mozilla.org/en-US/docs/Web/API/Performance/now
	/// [`Performance.timeOrigin`]: https://developer.mozilla.org/en-US/docs/Web/API/Performance/timeOrigin
	#[must_use]
	fn from_duration(duration: Duration) -> Self;

	/// Returns the [`Duration`] backing this [`Instant`](crate::Instant), i.e.
	/// the monotonic time elapsed since the origin it was created from.
	///
	/// # Note
	///
	/// Like [`InstantExt::from_duration`], the returned [`Duration`] is only
	/// meaningful relative to other [`Instant`](crate::Instant)s sharing the
	/// same origin.
	#[must_use]
	fn as_duration(&self) -> Duration;
}

impl InstantExt for Instant {
	fn from_duration(duration: Duration) -> Self {
		Self(duration)
	}

	fn as_duration(&self) -> Duration {
		self.0
	}
}
