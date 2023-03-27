use std::future::Future;
use std::pin::Pin;
use std::task::{ready, Context, Poll};
use std::time::Duration;

use js_sys::Promise;
use wasm_bindgen_futures::JsFuture;

pub(crate) struct Sleep(JsFuture);

impl Future for Sleep {
	type Output = ();

	fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
		ready!(Pin::new(&mut self.0).poll(cx)).unwrap();
		Poll::Ready(())
	}
}

/// Sleeps for the given [`Duration`].
pub(crate) fn sleep(duration: Duration) -> Sleep {
	let future = JsFuture::from(Promise::new(&mut |resolve, _| {
		let duration = duration.as_millis().try_into().unwrap();

		web_sys::window()
			.unwrap()
			.set_timeout_with_callback_and_timeout_and_arguments_0(&resolve, duration)
			.unwrap();
	}));

	Sleep(future)
}
