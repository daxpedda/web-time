//! Failure tests have to be separated as `should_panic` can cause serious
//! problems with `panic = "abort"`.

#![cfg(test)]

mod util;

use web_time::{Duration, Instant};

use self::util::{sleep, DIFF};

test! {
	/// [`Instant::add()`] failure.
	#[should_panic = "overflow when adding duration to instant"]
	async fn add_failure() {
		sleep(DIFF).await;
		let _ = Instant::now() + Duration::MAX;
	}

	/// [`Instant::sub()`] failure.
	#[should_panic = "overflow when subtracting duration from instant"]
	async fn sub_failure() {
		let _ = Instant::now() - Duration::MAX;
	}

	/// [`Instant::sub_assign()`] failure.
	#[should_panic = "overflow when subtracting duration from instant"]
	async fn sub_assign_failure() {
		let mut instant = Instant::now();
		instant -= Duration::MAX;
	}
}
