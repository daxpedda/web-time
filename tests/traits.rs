//! Test for traits on all exported types.

#![cfg(test)]
#![cfg_attr(target_arch = "wasm32", no_main)]
#![cfg_attr(all(target_arch = "wasm32", not(feature = "std")), no_std)]

mod util;

use core::fmt::{Debug, Display};
use core::hash::Hash;
use core::ops::{Add, AddAssign, Sub, SubAssign};
use core::panic::{RefUnwindSafe, UnwindSafe};
#[cfg(feature = "std")]
use std::error::Error;

use static_assertions::{assert_impl_all, assert_not_impl_any};
use web_time::{Duration, Instant, SystemTime, SystemTimeError};

/// Testing all traits on all types.
#[wasm_bindgen_test::wasm_bindgen_test(unsupported = test)]
const fn test() {
	assert_impl_all!(Instant: Clone, Copy, Debug, Hash, Eq, PartialEq, Ord, PartialOrd, Send, Sync, Unpin, RefUnwindSafe, UnwindSafe);
	assert_impl_all!(Instant: Add<Duration>, AddAssign<Duration>, Sub<Duration>, SubAssign<Duration>);

	assert_impl_all!(SystemTime: Clone, Copy, Debug, Hash, Eq, PartialEq, Ord, PartialOrd, Send, Sync, Unpin, RefUnwindSafe, UnwindSafe);
	assert_impl_all!(SystemTime: Add<Duration>, AddAssign<Duration>, Sub<Duration>, SubAssign<Duration>);

	assert_impl_all!(SystemTimeError: Clone, Debug, Display, Send, Sync, Unpin, RefUnwindSafe, UnwindSafe);
	#[cfg(feature = "std")]
	assert_impl_all!(SystemTimeError: Error);
	assert_not_impl_any!(SystemTimeError: Copy, Hash, Eq, PartialEq, Ord, PartialOrd);
}
