mod util;

use futures_channel::oneshot;
use wasm_bindgen_test::wasm_bindgen_test;
use web_time::Instant;

use self::util::{sleep, DIFF};

wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
async fn test() {
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
