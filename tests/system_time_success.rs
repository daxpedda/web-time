#![cfg(test)]
#![allow(clippy::missing_assert_message)]

mod util;

use web_time::{Duration, SystemTime};

use self::util::{sleep, DIFF};

test! {
	/// [`SystemTime::UNIX_EPOCH`].
	#[allow(clippy::eq_op)]
	async fn unix_epoch() {
		let time = SystemTime::UNIX_EPOCH.elapsed().unwrap();
		assert_eq!(time - time, Duration::ZERO);
	}

	/// [`SystemTime::duration_since()`] success.
	async fn duration_since_success() {
		let time = SystemTime::now();
		sleep(DIFF).await;
		let duration = SystemTime::now().duration_since(time).unwrap();
		assert!(duration >= DIFF);
		assert!(duration <= DIFF * 2);
	}

	/// [`SystemTime::duration_since()`] failure.
	async fn duration_since_failure() {
		let time = SystemTime::now();
		sleep(DIFF).await;
		let error = time.duration_since(SystemTime::now()).unwrap_err();
		let duration = error.duration();
		assert!(duration >= DIFF);
		assert!(duration <= DIFF * 2);
	}

	/// [`SystemTime::elapsed()`] success.
	async fn elapsed_success() {
		let time = SystemTime::now();
		sleep(DIFF).await;
		let duration = time.elapsed().unwrap();
		assert!(duration >= DIFF);
		assert!(duration <= DIFF * 2);
	}

	/// [`SystemTime::elapsed()`] failure.
	async fn elapsed_failure() {
		let time = SystemTime::now() + DIFF;
		let error = time.elapsed().unwrap_err();
		assert!(error.duration() <= DIFF);
	}

	/// [`SystemTime::checked_add()`] success.
	async fn checked_add_success() {
		let time = SystemTime::now();
		sleep(DIFF).await;
		let now = SystemTime::now();
		assert!(time.checked_add(DIFF).unwrap() <= now);
		assert!(time.checked_add(DIFF * 2).unwrap() >= now);
	}

	/// [`SystemTime::checked_add()`] failure.
	async fn checked_add_failure() {
		sleep(DIFF).await;
		assert_eq!(SystemTime::now().checked_add(Duration::MAX), None);
	}

	/// [`SystemTime::checked_sub()`] success.
	async fn checked_sub_success() {
		let time = SystemTime::now();
		sleep(DIFF).await;
		let now = SystemTime::now();
		assert!(now.checked_sub(DIFF).unwrap() >= time);
		assert!(now.checked_sub(DIFF * 2).unwrap() <= time);
	}

	/// [`SystemTime::checked_sub()`] failure.
	async fn checked_sub_failure() {
		assert_eq!(SystemTime::now().checked_sub(Duration::MAX), None);
	}

	/// [`SystemTime::add()`] success.
	async fn add_success() {
		let time = SystemTime::now();
		sleep(DIFF).await;
		assert!(time + DIFF <= SystemTime::now());
		assert!(time + DIFF * 2 >= SystemTime::now());
	}

	/// [`SystemTime::add_assign()`] success.
	async fn add_assign_success() {
		let mut time = SystemTime::now();
		sleep(DIFF).await;
		time += DIFF;
		assert!(time <= SystemTime::now());
		time += DIFF * 2;
		assert!(time >= SystemTime::now());
	}

	/// [`SystemTime::sub()`] success.
	async fn sub_success() {
		let time = SystemTime::now();
		sleep(DIFF).await;
		let now = SystemTime::now();
		assert!(now - DIFF >= time);
		assert!(now - DIFF * 2 <= time);
	}

	/// [`SystemTime::sub_assign()`] success.
	async fn sub_assign_success() {
		let earlier = SystemTime::now();
		sleep(DIFF).await;
		let mut later = SystemTime::now();
		later -= DIFF;
		assert!(later >= earlier);
		later -= DIFF;
		assert!(later <= earlier);
	}
}
