use std::future::{self, Ready};
use std::thread;

use web_time::Duration;

/// Sleeps for the given [`Duration`].
#[allow(unused)]
pub(crate) fn sleep(duration: Duration) -> Ready<()> {
	thread::sleep(duration);
	future::ready(())
}
