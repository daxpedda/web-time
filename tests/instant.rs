mod util;

use web_time::{Duration, Instant};

use self::util::{sleep, DIFF};

#[cfg(all(
	target_family = "wasm",
	not(any(target_os = "emscripten", target_os = "wasi"))
))]
wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

#[cfg_attr(
	not(all(
		target_family = "wasm",
		not(any(target_os = "emscripten", target_os = "wasi"))
	)),
	pollster::test
)]
#[cfg_attr(
	all(
		target_family = "wasm",
		not(any(target_os = "emscripten", target_os = "wasi"))
	),
	wasm_bindgen_test::wasm_bindgen_test
)]
async fn test() {
	// `Instant::duration_since()` success.
	let instant = Instant::now();
	sleep(DIFF).await;
	assert!(Instant::now().duration_since(instant) >= DIFF);

	// `Instant::duration_since()` fail.
	let instant = Instant::now();
	sleep(DIFF).await;
	assert_eq!(instant.duration_since(Instant::now()), Duration::ZERO);

	// `Instant::checked_duration_since()` success.
	let instant = Instant::now();
	sleep(DIFF).await;
	assert!(Instant::now().checked_duration_since(instant) >= Some(DIFF));

	// `Instant::checked_duration_since()` fail.
	let instant = Instant::now();
	sleep(DIFF).await;
	assert_eq!(instant.checked_duration_since(Instant::now()), None);

	// `Instant::saturating_duration_since()` success.
	let instant = Instant::now();
	sleep(DIFF).await;
	assert!(Instant::now().saturating_duration_since(instant) >= DIFF);

	// `Instant::saturating_duration_fail()` success.
	let instant = Instant::now();
	sleep(DIFF).await;
	assert_eq!(
		instant.saturating_duration_since(Instant::now()),
		Duration::ZERO
	);

	// `Instant::elapsed()`.
	let instant = Instant::now();
	sleep(DIFF).await;
	assert!(instant.elapsed() >= DIFF);

	// `Instant::checked_add()` success.
	let instant = Instant::now();
	sleep(DIFF).await;
	assert!(instant.checked_add(DIFF).unwrap() <= Instant::now());

	// `Instant::checked_add()` fail.
	sleep(DIFF).await;
	assert_eq!(Instant::now().checked_add(Duration::MAX), None);

	// `Instant::checked_sub()` success.
	let instant = Instant::now();
	sleep(DIFF).await;
	assert!(Instant::now().checked_sub(DIFF).unwrap() >= instant);

	// `Instant::checked_sub()` fail.
	assert_eq!(Instant::now().checked_sub(Duration::MAX), None);

	// `Instant::add()` success.
	let instant = Instant::now();
	sleep(DIFF).await;
	assert!(instant + DIFF <= Instant::now());

	// `Instant::add_assign()` success.
	let mut instant = Instant::now();
	sleep(DIFF).await;
	instant += DIFF;
	assert!(instant <= Instant::now());

	// `Instant::sub()` success.
	let isntant = Instant::now();
	sleep(DIFF).await;
	assert!(Instant::now() - DIFF >= isntant);

	// `Instant::sub_assign()` success.
	let earlier = Instant::now();
	sleep(DIFF).await;
	let mut later = Instant::now();
	later -= DIFF;
	assert!(later >= earlier);

	// `Self` comparisons.
	let earlier = Instant::now();

	let later = Instant::now();
	assert!(earlier <= later, "{:?}", earlier - later);

	sleep(DIFF).await;

	let later = Instant::now();
	assert!((later - earlier) >= DIFF, "{:?}", later - earlier);

	let later = Instant::now();
	assert!(earlier <= later, "{:?}", earlier - later);
}

// Waiting for <https://github.com/rustwasm/wasm-bindgen/pull/3293>.
/*#[wasm_bindgen_test]
#[should_panic]
async fn add_fail() {
	sleep(DIFF).await;
	let _ = Instant::now() + Duration::MAX;
}*/

// Waiting for <https://github.com/rustwasm/wasm-bindgen/pull/3293>.
/*#[wasm_bindgen_test]
#[should_panic]
async fn add_assign_fail() {
	sleep(DIFF).await;
	let mut instant = Instant::now();
	instant += Duration::MAX;
}*/

// Waiting for <https://github.com/rustwasm/wasm-bindgen/pull/3293>.
/*#[wasm_bindgen_test]
#[should_panic]
async fn sub_fail() {
	let _ = Instant::now() - Duration::MAX;
}*/

// Waiting for <https://github.com/rustwasm/wasm-bindgen/pull/3293>.
/*#[wasm_bindgen_test]
#[should_panic]
async fn sub_assign_fail() {
	let instant = Instant::now();
	let _ = instant - Duration::MAX;
}*/
