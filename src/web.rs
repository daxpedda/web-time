//! Platform-specific extensions to [`web-time`](crate) for the Web platform.

#![allow(clippy::absolute_paths)]

use std::time::SystemTime as StdSystemTime;

use crate::SystemTime;

#[cfg(all(
	target_family = "wasm",
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
			target_family = "wasm",
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
			target_family = "wasm",
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
