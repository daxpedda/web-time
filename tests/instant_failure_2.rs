//! Failure tests have to be separated as `should_panic` can cause serious
//! problems with `panic = "abort"`.

#![cfg(test)]

mod util;

use web_time::{Duration, Instant};

use self::util::{sleep, DIFF};

test! {
	/// [`Instant::add_assign()`] failure.
	#[should_panic = "overflow when adding duration to instant"]
	async fn add_assign_failure() {
		sleep(DIFF).await;
		let mut instant = Instant::now();
		instant += Duration::MAX;
	}
}
