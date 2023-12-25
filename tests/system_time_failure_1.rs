//! Failure tests have to be separated as `should_panic` can cause serious
//! problems with `panic = "abort"`.

#![cfg(test)]

mod util;

use web_time::{Duration, SystemTime};

use self::util::{sleep, DIFF};

test! {
	/// [`SystemTime::add()`] failure.
	#[should_panic = "overflow when adding duration to instant"]
	async fn add_failure() {
		sleep(DIFF).await;
		let _ = SystemTime::now() + Duration::MAX;
	}

	/// [`SystemTime::sub()`] failure.
	#[should_panic = "overflow when subtracting duration from instant"]
	async fn sub_failure() {
		let _ = SystemTime::now() - Duration::MAX;
	}

	/// [`SystemTime::sub_assign()`] failure.
	#[should_panic = "overflow when subtracting duration from instant"]
	async fn sub_assign_failure() {
		let time = SystemTime::now();
		let _ = time - Duration::MAX;
	}
}
