use std::future::{self, Ready};
use std::thread;

use web_time::Duration;

/// Sleeps for the given [`Duration`].
#[allow(clippy::allow_attributes, dead_code, reason = "not used by all tests")]
pub(crate) fn sleep(duration: Duration) -> Ready<()> {
	thread::sleep(duration);
	future::ready(())
}
