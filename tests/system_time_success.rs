//! [`SystemTime`] tests.

#![cfg(test)]
#![cfg_attr(target_arch = "wasm32", no_main)]
#![cfg_attr(all(target_arch = "wasm32", not(feature = "std")), no_std)]

extern crate alloc;

mod util;

use alloc::string::ToString;

use wasm_bindgen_test::wasm_bindgen_test;
use web_time::{Duration, SystemTime};

use self::util::{sleep, DIFF, MAX_DIFF};

/// [`SystemTime::UNIX_EPOCH`].
#[allow(
	clippy::allow_attributes,
	clippy::eq_op,
	reason = "thats what we are testing"
)]
#[wasm_bindgen_test(unsupported = test)]
fn unix_epoch() {
	let time = SystemTime::UNIX_EPOCH.elapsed().unwrap();
	assert_eq!(time - time, Duration::ZERO);
}

/// [`SystemTime::duration_since()`] success.
#[wasm_bindgen_test(unsupported = pollster::test)]
async fn duration_since_success() {
	let time = SystemTime::now();
	sleep(DIFF).await;
	let duration = SystemTime::now().duration_since(time).unwrap();
	assert!(duration >= DIFF);
	assert!(duration <= MAX_DIFF);
}

/// [`SystemTime::duration_since()`] failure.
#[wasm_bindgen_test(unsupported = pollster::test)]
async fn duration_since_failure() {
	let time = SystemTime::now();
	sleep(DIFF).await;
	let error = time.duration_since(SystemTime::now()).unwrap_err();
	let duration = error.duration();
	assert!(duration >= DIFF);
	assert!(duration <= MAX_DIFF);
}

/// [`SystemTime::elapsed()`] success.
#[wasm_bindgen_test(unsupported = pollster::test)]
async fn elapsed_success() {
	let time = SystemTime::now();
	sleep(DIFF).await;
	let duration = time.elapsed().unwrap();
	assert!(duration >= DIFF);
	assert!(duration <= MAX_DIFF);
}

/// [`SystemTime::elapsed()`] failure.
#[wasm_bindgen_test(unsupported = test)]
fn elapsed_failure() {
	let time = SystemTime::now() + DIFF;
	let error = time.elapsed().unwrap_err();
	assert!(error.duration() <= DIFF);
}

/// [`SystemTime::checked_add()`] success.
#[wasm_bindgen_test(unsupported = pollster::test)]
async fn checked_add_success() {
	let time = SystemTime::now();
	sleep(DIFF).await;
	let now = SystemTime::now();
	assert!(time.checked_add(DIFF).unwrap() <= now);
	assert!(time.checked_add(MAX_DIFF).unwrap() >= now);
}

/// [`SystemTime::checked_add()`] failure.
#[wasm_bindgen_test(unsupported = pollster::test)]
async fn checked_add_failure() {
	sleep(DIFF).await;
	assert_eq!(SystemTime::now().checked_add(Duration::MAX), None);
}

/// [`SystemTime::checked_sub()`] success.
#[wasm_bindgen_test(unsupported = pollster::test)]
async fn checked_sub_success() {
	let time = SystemTime::now();
	sleep(DIFF).await;
	let now = SystemTime::now();
	assert!(now.checked_sub(DIFF).unwrap() >= time);
	assert!(now.checked_sub(MAX_DIFF).unwrap_or(SystemTime::UNIX_EPOCH) <= time);
}

/// [`SystemTime::checked_sub()`] failure.
#[wasm_bindgen_test(unsupported = test)]
fn checked_sub_failure() {
	assert_eq!(SystemTime::now().checked_sub(Duration::MAX), None);
}

/// [`SystemTime::add()`] success.
#[wasm_bindgen_test(unsupported = pollster::test)]
async fn add_success() {
	let time = SystemTime::now();
	sleep(DIFF).await;
	assert!(time + DIFF <= SystemTime::now());
	assert!(time + MAX_DIFF >= SystemTime::now());
}

/// [`SystemTime::add_assign()`] success.
#[wasm_bindgen_test(unsupported = pollster::test)]
async fn add_assign_success() {
	let mut time_1 = SystemTime::now();
	let mut time_2 = time_1;
	sleep(DIFF).await;
	let now = SystemTime::now();
	time_1 += DIFF;
	assert!(time_1 <= now);
	time_2 += MAX_DIFF;
	assert!(time_2 >= now);
}

/// [`SystemTime::sub()`] success.
#[wasm_bindgen_test(unsupported = pollster::test)]
async fn sub_success() {
	let time = SystemTime::now();
	sleep(DIFF).await;
	let now = SystemTime::now();
	assert!(now - DIFF >= time);
	assert!(now.duration_since(time).unwrap() <= MAX_DIFF);
}

/// [`SystemTime::sub_assign()`] success.
#[wasm_bindgen_test(unsupported = pollster::test)]
async fn sub_assign_success() {
	let earlier = SystemTime::now();
	sleep(DIFF).await;
	let mut later = SystemTime::now();
	later -= DIFF;
	assert!(later >= earlier);
	assert!(later.duration_since(earlier).unwrap() <= MAX_DIFF);
}

/// [`SystemTime::elapsed()`] failure.
#[wasm_bindgen_test(unsupported = test)]
fn error() {
	let time = SystemTime::now() + DIFF;
	let error = time.elapsed().unwrap_err();
	assert_eq!(
		error.to_string(),
		"second time provided was later than self"
	);
}
