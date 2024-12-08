//! Run tests with the atomics target feature.

#![cfg(test)]
#![cfg(target_feature = "atomics")]

mod util;

use futures_util::future;
use futures_util::future::Either;
use wasm_bindgen_test::wasm_bindgen_test;
use web_sys::{console, OfflineAudioContext};
use web_thread::web::audio_worklet::BaseAudioContextExt;
use web_time::Instant;

use self::util::{Flag, MAX_DIFF};

/// Testing failure of [`Instant::now()`] in audio worklet.
#[wasm_bindgen_test]
async fn test() {
	if web_sys::window().is_none() {
		console::error_1(&"found ourselves not in a `Window`".into());
		return;
	}

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
