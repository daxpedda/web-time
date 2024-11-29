//! Re-implementation of [`std::time`].
#![cfg_attr(
	not(feature = "std"),
	doc = "",
	doc = "[`std::time`]: https://doc.rust-lang.org/std/time"
)]

mod instant;
mod js;
#[cfg(feature = "serde")]
mod serde;
mod system_time;

#[cfg(not(feature = "std"))]
pub use core::time::*;
#[cfg(feature = "std")]
pub use std::time::*;

pub use self::instant::Instant;
pub use self::system_time::{SystemTime, SystemTimeError};

/// See [`std::time::UNIX_EPOCH`].
#[cfg_attr(
	not(feature = "std"),
	doc = "",
	doc = "[`std::time::UNIX_EPOCH`]: https://doc.rust-lang.org/std/time/constant.UNIX_EPOCH.html"
)]
pub const UNIX_EPOCH: SystemTime = SystemTime::UNIX_EPOCH;
