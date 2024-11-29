//! [`Instant`] tests.

#![cfg(test)]
#![cfg_attr(target_family = "wasm", no_main)]
#![cfg_attr(all(target_family = "wasm", not(feature = "std")), no_std)]

mod util;

use wasm_bindgen_test::wasm_bindgen_test;
use web_time::{Duration, Instant};

use self::util::{sleep, DIFF, MAX_DIFF};

/// [`Instant::duration_since()`] success.
#[wasm_bindgen_test(unsupported = pollster::test)]
async fn duration_success() {
	let instant = Instant::now();
	sleep(DIFF).await;
	let duration = Instant::now().duration_since(instant);
	assert!(duration >= DIFF);
	assert!(duration <= MAX_DIFF);
}

/// [`Instant::duration_since()`] failure.
#[wasm_bindgen_test(unsupported = pollster::test)]
async fn duration_failure() {
	let instant = Instant::now();
	sleep(DIFF).await;
	assert_eq!(instant.duration_since(Instant::now()), Duration::ZERO);
}

/// [`Instant::checked_duration_since()`] success.
#[wasm_bindgen_test(unsupported = pollster::test)]
async fn checked_duration_success() {
	let instant = Instant::now();
	sleep(DIFF).await;
	let duration = Instant::now().checked_duration_since(instant);
	assert!(duration >= Some(DIFF));
	assert!(duration <= Some(MAX_DIFF));
}

/// [`Instant::checked_duration_since()`] failure.
#[wasm_bindgen_test(unsupported = pollster::test)]
async fn checked_duration_failure() {
	let instant = Instant::now();
	sleep(DIFF).await;
	assert_eq!(instant.checked_duration_since(Instant::now()), None);
}

/// [`Instant::saturating_duration_since()`] success.
#[wasm_bindgen_test(unsupported = pollster::test)]
async fn saturating_duration_success() {
	let instant = Instant::now();
	sleep(DIFF).await;
	let duration = Instant::now().saturating_duration_since(instant);
	assert!(duration >= DIFF);
	assert!(duration <= MAX_DIFF);
}

/// [`Instant::saturating_duration_fail()`] success.
#[wasm_bindgen_test(unsupported = pollster::test)]
async fn saturating_duration_failure() {
	let instant = Instant::now();
	sleep(DIFF).await;
	assert_eq!(
		instant.saturating_duration_since(Instant::now()),
		Duration::ZERO
	);
}

/// [`Instant::elapsed()`].
#[wasm_bindgen_test(unsupported = pollster::test)]
async fn elapsed() {
	let instant = Instant::now();
	sleep(DIFF).await;
	let duration = instant.elapsed();
	assert!(duration >= DIFF);
	assert!(duration <= MAX_DIFF);
}

/// [`Instant::checked_add()`] success.
#[wasm_bindgen_test(unsupported = pollster::test)]
async fn checked_add_success() {
	let instant = Instant::now();
	sleep(DIFF).await;
	let now = Instant::now();
	assert!(instant.checked_add(DIFF).unwrap() <= now);
	assert!(instant.checked_add(MAX_DIFF).unwrap() >= now);
}

/// [`Instant::checked_add()`] failure.
#[wasm_bindgen_test(unsupported = pollster::test)]
async fn checked_add_failure() {
	sleep(DIFF).await;
	assert_eq!(Instant::now().checked_add(Duration::MAX), None);
}

/// [`Instant::checked_sub()`] success.
#[wasm_bindgen_test(unsupported = pollster::test)]
async fn checked_sub_success() {
	let instant = Instant::now();
	sleep(DIFF).await;
	let now = Instant::now();
	assert!(now.checked_sub(DIFF).unwrap() >= instant);
	assert!(now.duration_since(instant) <= MAX_DIFF);
}

/// [`Instant::checked_sub()`] failure.
#[wasm_bindgen_test(unsupported = test)]
fn checked_sub_failure() {
	assert_eq!(Instant::now().checked_sub(Duration::MAX), None);
}

/// [`Instant::add()`] success.
#[wasm_bindgen_test(unsupported = pollster::test)]
async fn add_success() {
	let instant = Instant::now();
	sleep(DIFF).await;
	let now = Instant::now();
	assert!(instant + DIFF <= now);
	assert!(instant + MAX_DIFF >= now);
}

/// [`Instant::add_assign()`] success.
#[wasm_bindgen_test(unsupported = pollster::test)]
async fn add_assign_success() {
	let mut instant = Instant::now();
	sleep(DIFF).await;
	let now = Instant::now();
	instant += DIFF;
	assert!(instant <= now);
	instant += DIFF;
	assert!(instant >= now);
}

/// [`Instant::sub()`] success.
#[wasm_bindgen_test(unsupported = pollster::test)]
#[allow(
	clippy::allow_attributes,
	clippy::unchecked_duration_subtraction,
	reason = "this is what we are testing"
)]
async fn sub_success() {
	let instant = Instant::now();
	sleep(DIFF).await;
	let now = Instant::now();
	assert!(now - DIFF >= instant);
	assert!(now.duration_since(instant) <= MAX_DIFF);
}

/// [`Instant::sub_assign()`] success.
#[wasm_bindgen_test(unsupported = pollster::test)]
async fn sub_assign_success() {
	let earlier = Instant::now();
	sleep(DIFF).await;
	let mut later = Instant::now();
	later -= DIFF;
	assert!(later >= earlier);
	assert!(later.duration_since(earlier) <= MAX_DIFF);
}

/// [`Self`] comparisons.
#[wasm_bindgen_test(unsupported = pollster::test)]
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
