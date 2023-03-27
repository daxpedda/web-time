#![allow(clippy::cognitive_complexity, clippy::semicolon_if_nothing_returned)]

mod util;

use web_time::{Duration, SystemTime};

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
	// `SystemTime::UNIX_EPOCH`.
	#[allow(clippy::eq_op)]
	{
		let time = SystemTime::UNIX_EPOCH.elapsed().unwrap();
		assert_eq!(time - time, Duration::ZERO);
	}

	// `SystemTime::duration_since()` success.
	let time = SystemTime::now();
	sleep(DIFF).await;
	assert!(SystemTime::now().duration_since(time).unwrap() >= DIFF);

	// `SystemTime::duration_since()` fail.
	let time = SystemTime::now();
	sleep(DIFF).await;
	let error = time.duration_since(SystemTime::now()).unwrap_err();
	assert!(error.duration() >= DIFF);

	// `SystemTime::elapsed()` success.
	let time = SystemTime::now();
	sleep(DIFF).await;
	assert!(time.elapsed().unwrap() >= DIFF);

	// `SystemTime::elapsed()` fail.
	let time = SystemTime::now() + DIFF;
	let error = time.elapsed().unwrap_err();
	assert!(error.duration() <= DIFF);

	// `SystemTime::checked_add()` success.
	let time = SystemTime::now();
	sleep(DIFF).await;
	assert!(time.checked_add(DIFF).unwrap() <= SystemTime::now());

	// `SystemTime::checked_add()` fail.
	sleep(DIFF).await;
	assert_eq!(SystemTime::now().checked_add(Duration::MAX), None);

	// `SystemTime::checked_sub()` success.
	let time = SystemTime::now();
	sleep(DIFF).await;
	assert!(SystemTime::now().checked_sub(DIFF).unwrap() >= time);

	// `SystemTime::checked_sub()` fail.
	assert_eq!(SystemTime::now().checked_sub(Duration::MAX), None);

	// `SystemTime::add()` success.
	let time = SystemTime::now();
	sleep(DIFF).await;
	assert!(time + DIFF <= SystemTime::now());

	// `SystemTime::add_assign()` success.
	let mut time = SystemTime::now();
	sleep(DIFF).await;
	time += DIFF;
	assert!(time <= SystemTime::now());

	// `SystemTime::sub()` success.
	let time = SystemTime::now();
	sleep(DIFF).await;
	assert!(SystemTime::now() - DIFF >= time);

	// `SystemTime::sub_assign()` success.
	let earlier = SystemTime::now();
	sleep(DIFF).await;
	let mut later = SystemTime::now();
	later -= DIFF;
	assert!(later >= earlier);
}

// Waiting for <https://github.com/rustwasm/wasm-bindgen/pull/3293>.
/*#[wasm_bindgen_test]
#[should_panic]
async fn add_fail() {
	sleep(DIFF).await;
	let _ = SystemTime::now() + Duration::MAX;
}*/

// Waiting for <https://github.com/rustwasm/wasm-bindgen/pull/3293>.
/*#[wasm_bindgen_test]
#[should_panic]
async fn add_assign_fail() {
	sleep(DIFF).await;
	let mut time = SystemTime::now();
	time += Duration::MAX;
}*/

// Waiting for <https://github.com/rustwasm/wasm-bindgen/pull/3293>.
/*#[wasm_bindgen_test]
#[should_panic]
async fn sub_fail() {
	let _ = SystemTime::now() - Duration::MAX;
}*/

// Waiting for <https://github.com/rustwasm/wasm-bindgen/pull/3293>.
/*#[wasm_bindgen_test]
#[should_panic]
async fn sub_assign_fail() {
	let time = SystemTime::now();
	let _ = time - Duration::MAX;
}*/
