#![cfg(test)]
#![cfg(all(target_family = "wasm", target_feature = "atomics"))]

mod util;

use futures_util::future;
use futures_util::future::Either;
use util::MAX_DIFF;
use wasm_bindgen_test::wasm_bindgen_test;
use web_sys::OfflineAudioContext;
use web_thread::web::audio_worklet::BaseAudioContextExt;
use web_time::Instant;

use self::util::Flag;

wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
async fn test() {
	let context =
		OfflineAudioContext::new_with_number_of_channels_and_length_and_sample_rate(1, 1, 8000.)
			.unwrap();

	let flag = Flag::new();

	context
		.clone()
		.register_thread(None, {
			let flag = flag.clone();
			move || {
				let _ = Instant::now();
				flag.signal();
			}
		})
		.await
		.unwrap();

	assert!(matches!(
		future::select(flag, util::sleep(MAX_DIFF)).await,
		Either::Right(_)
	));
}
