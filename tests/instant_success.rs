mod util;

use web_time::{Duration, Instant};

use self::util::{sleep, DIFF};

#[cfg(all(
	target_family = "wasm",
	not(any(target_os = "emscripten", target_os = "wasi"))
))]
wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

test! {
	/// [`Instant::duration_since()`] success.
	async fn duration_success() {
		let instant = Instant::now();
		sleep(DIFF).await;
		assert!(Instant::now().duration_since(instant) >= DIFF);
	}

	/// [`Instant::duration_since()`] failure.
	async fn duration_failure() {
		let instant = Instant::now();
		sleep(DIFF).await;
		assert_eq!(instant.duration_since(Instant::now()), Duration::ZERO);
	}

	/// [`Instant::checked_duration_since()`] success.
	async fn checked_duration_success() {
		let instant = Instant::now();
		sleep(DIFF).await;
		assert!(Instant::now().checked_duration_since(instant) >= Some(DIFF));
	}

	/// [`Instant::checked_duration_since()`] failure.
	async fn checked_duration_failure() {
		let instant = Instant::now();
		sleep(DIFF).await;
		assert_eq!(instant.checked_duration_since(Instant::now()), None);
	}

	/// [`Instant::saturating_duration_since()`] success.
	async fn saturating_duration_success() {
		let instant = Instant::now();
		sleep(DIFF).await;
		assert!(Instant::now().saturating_duration_since(instant) >= DIFF);
	}

	/// [`Instant::saturating_duration_fail()`] success.
	async fn saturating_duration_failure() {
		let instant = Instant::now();
		sleep(DIFF).await;
		assert_eq!(
			instant.saturating_duration_since(Instant::now()),
			Duration::ZERO
		);
	}

	/// [`Instant::elapsed()`].
	async fn elapsed() {
		let instant = Instant::now();
		sleep(DIFF).await;
		assert!(instant.elapsed() >= DIFF);
	}

	/// [`Instant::checked_add()`] success.
	async fn checked_add_success() {
		let instant = Instant::now();
		sleep(DIFF).await;
		assert!(instant.checked_add(DIFF).unwrap() <= Instant::now());
	}

	/// [`Instant::checked_add()`] failure.
	async fn checked_add_failure() {
		sleep(DIFF).await;
		assert_eq!(Instant::now().checked_add(Duration::MAX), None);
	}

	/// [`Instant::checked_sub()`] success.
	async fn checked_sub_success() {
		let instant = Instant::now();
		sleep(DIFF).await;
		assert!(Instant::now().checked_sub(DIFF).unwrap() >= instant);
	}

	/// [`Instant::checked_sub()`] failure.
	async fn checked_sub_failure() {
		assert_eq!(Instant::now().checked_sub(Duration::MAX), None);
	}

	/// [`Instant::add()`] success.
	async fn add_success() {
		let instant = Instant::now();
		sleep(DIFF).await;
		assert!(instant + DIFF <= Instant::now());
	}

	/// [`Instant::add_assign()`] success.
	async fn add_assign_success() {
		let mut instant = Instant::now();
		sleep(DIFF).await;
		instant += DIFF;
		assert!(instant <= Instant::now());
	}

	/// [`Instant::sub()`] success.
	async fn sub_success() {
		let instant = Instant::now();
		sleep(DIFF).await;
		assert!(Instant::now() - DIFF >= instant);
	}

	/// [`Instant::sub_assign()`] success.
	async fn sub_assign_success() {
		let earlier = Instant::now();
		sleep(DIFF).await;
		let mut later = Instant::now();
		later -= DIFF;
		assert!(later >= earlier);
	}

	/// [`Self`] comparisons.
	async fn comparison() {
		let earlier = Instant::now();

		let later = Instant::now();
		assert!(earlier <= later, "{:?}", earlier - later);

		sleep(DIFF).await;

		let later = Instant::now();
		assert!((later - earlier) >= DIFF, "{:?}", later - earlier);

		let later = Instant::now();
		assert!(earlier <= later, "{:?}", earlier - later);
	}
}
