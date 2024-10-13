//! Test for traits on all exported types.

use std::error::Error;
use std::fmt::{Debug, Display};
use std::hash::Hash;
use std::ops::{Add, AddAssign, Sub, SubAssign};
use std::panic::{RefUnwindSafe, UnwindSafe};

use static_assertions::{assert_impl_all, assert_not_impl_any};
use web_time::{Duration, Instant, SystemTime, SystemTimeError};

#[cfg(target_family = "wasm")]
wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

#[cfg_attr(not(target_family = "wasm"), test)]
#[cfg_attr(target_family = "wasm", wasm_bindgen_test::wasm_bindgen_test)]
const fn test() {
	assert_impl_all!(Instant: Clone, Copy, Debug, Hash, Eq, PartialEq, Ord, PartialOrd, Send, Sync, Unpin, RefUnwindSafe, UnwindSafe);
	assert_impl_all!(Instant: Add<Duration>, AddAssign<Duration>, Sub<Duration>, SubAssign<Duration>);

	assert_impl_all!(SystemTime: Clone, Copy, Debug, Hash, Eq, PartialEq, Ord, PartialOrd, Send, Sync, Unpin, RefUnwindSafe, UnwindSafe);
	assert_impl_all!(SystemTime: Add<Duration>, AddAssign<Duration>, Sub<Duration>, SubAssign<Duration>);

	assert_impl_all!(SystemTimeError: Clone, Debug, Display, Error, Send, Sync, Unpin, RefUnwindSafe, UnwindSafe);
	assert_not_impl_any!(SystemTimeError: Copy, Hash, Eq, PartialEq, Ord, PartialOrd);
}
