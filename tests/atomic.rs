#![cfg(test)]
#![cfg(all(target_family = "wasm", target_feature = "atomics"))]

mod util;

use futures_channel::oneshot;
use wasm_bindgen_test::wasm_bindgen_test;
use web_time::{Duration, Instant};

use self::util::{sleep, Flag, DIFF};

wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
async fn basic() {
	let earlier = Instant::now();

	let flag = Flag::new();

	wasm_worker::spawn_async({
		let flag = flag.clone();
		move |context| async move {
			let later = Instant::now();
			assert!(earlier <= later, "{:?}", earlier - later);

			sleep(DIFF).await;

			let later = Instant::now();
			assert!((later - earlier) >= DIFF, "{:?}", later - earlier);

			let later = Instant::now();
			assert!(earlier <= later, "{:?}", earlier - later);

			flag.signal();
			context.close();
		}
	});

	flag.await;
}

#[wasm_bindgen_test]
async fn delay() {
	sleep(Duration::from_secs(2)).await;

	let earlier = Instant::now();

	let flag = Flag::new();

	wasm_worker::spawn_async({
		let flag = flag.clone();
		move |context| async move {
			let later = Instant::now();
			assert!(earlier <= later, "{:?}", earlier - later);

			sleep(DIFF).await;

			let later = Instant::now();
			assert!((later - earlier) >= DIFF, "{:?}", later - earlier);

			let later = Instant::now();
			assert!(earlier <= later, "{:?}", earlier - later);

			flag.signal();
			context.close();
		}
	});

	flag.await;
}

#[wasm_bindgen_test]
async fn worker() {
	let (sender, receiver) = oneshot::channel();
	wasm_worker::spawn_async(move |context| async move {
		sender.send(Instant::now()).unwrap();
		context.close();
	});

	let earlier = receiver.await.unwrap();
	let later = Instant::now();
	assert!(earlier <= later, "{:?}", earlier - later);

	sleep(DIFF).await;

	let later = Instant::now();
	assert!((later - earlier) >= DIFF, "{:?}", later - earlier);

	let later = Instant::now();
	assert!(earlier <= later, "{:?}", earlier - later);
}
