//! Run tests with the atomics target feature.

#![cfg(test)]
#![cfg(target_feature = "atomics")]

mod util;

use futures_channel::oneshot;
use wasm_bindgen_test::wasm_bindgen_test;
use web_sys::console;
use web_thread::web::{self, has_spawn_support};
use web_time::{Duration, Instant};

use self::util::{sleep, Flag, DIFF, WAIT};

#[wasm_bindgen_test]
async fn basic() {
	if !has_spawn_support() {
		console::error_1(&"can't spawn threads".into());
		return;
	}

	let earlier = Instant::now();

	let flag = Flag::new();

	web::spawn_async({
		let flag = flag.clone();
		move || async move {
			let later = Instant::now();
			assert!(earlier <= later, "{:?}", earlier - later);

			sleep(WAIT).await;

			let later = Instant::now();
			assert!((later - earlier) >= DIFF, "{:?}", later - earlier);

			let later = Instant::now();
			assert!(earlier <= later, "{:?}", earlier - later);

			flag.signal();
		}
	});

	flag.await;
}

#[wasm_bindgen_test]
async fn delay() {
	if !has_spawn_support() {
		console::error_1(&"can't spawn threads".into());
		return;
	}

	sleep(Duration::from_secs(2)).await;

	let earlier = Instant::now();

	let flag = Flag::new();

	web::spawn_async({
		let flag = flag.clone();
		move || async move {
			let later = Instant::now();
			assert!(earlier <= later, "{:?}", earlier - later);

			sleep(WAIT).await;

			let later = Instant::now();
			assert!((later - earlier) >= DIFF, "{:?}", later - earlier);

			let later = Instant::now();
			assert!(earlier <= later, "{:?}", earlier - later);

			flag.signal();
		}
	});

	flag.await;
}

#[wasm_bindgen_test]
async fn worker() {
	if !has_spawn_support() {
		console::error_1(&"can't spawn threads".into());
		return;
	}

	let (sender, receiver) = oneshot::channel();
	web::spawn_async(move || async move { sender.send(Instant::now()).unwrap() });

	let earlier = receiver.await.unwrap();
	let later = Instant::now();
	assert!(earlier <= later, "{:?}", earlier - later);

	sleep(WAIT).await;

	let later = Instant::now();
	assert!((later - earlier) >= DIFF, "{:?}", later - earlier);

	let later = Instant::now();
	assert!(earlier <= later, "{:?}", earlier - later);
}
