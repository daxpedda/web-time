//! Web specific utility.

#[cfg(all(target_feature = "atomics", feature = "std"))]
extern crate alloc;

#[cfg(all(target_feature = "atomics", feature = "std"))]
use alloc::sync::Arc;
use core::future::Future;
use core::pin::Pin;
#[cfg(all(target_feature = "atomics", feature = "std"))]
use core::sync::atomic::{AtomicBool, Ordering};
use core::task::{ready, Context, Poll};
use core::time::Duration;

#[cfg(all(target_feature = "atomics", feature = "std"))]
use futures_util::task::AtomicWaker;
use js_sys::{Function, Promise};
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen_futures::JsFuture;

/// Async version of [`std::thread::sleep()`].
pub(crate) struct Sleep(JsFuture);

impl Future for Sleep {
	type Output = ();

	fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
		ready!(Pin::new(&mut self.0).poll(cx)).unwrap();
		Poll::Ready(())
	}
}

/// Sleeps for the given [`Duration`].
#[allow(clippy::allow_attributes, dead_code, reason = "not used by all tests")]
pub(crate) fn sleep(duration: Duration) -> Sleep {
	#[wasm_bindgen]
	extern "C" {
		#[wasm_bindgen(js_name = setTimeout)]
		fn set_timeout(handler: &Function, timeout: i32);
	}

	let future = JsFuture::from(Promise::new(&mut |resolve, _| {
		let duration = duration.as_millis().try_into().unwrap();
		set_timeout(&resolve, duration);
	}));

	Sleep(future)
}

/// Can be awaited to wake up thread when signaled.
#[cfg(all(target_feature = "atomics", feature = "std"))]
#[derive(Clone)]
pub(crate) struct Flag(Arc<Inner>);

/// Shared data for [`Flag`].
#[cfg(all(target_feature = "atomics", feature = "std"))]
struct Inner {
	/// The registered thread to wake.
	waker: AtomicWaker,
	/// If the [`Flag`] was [`signal()`](Flag::signal)ed.
	set: AtomicBool,
}

#[cfg(all(target_feature = "atomics", feature = "std"))]
#[allow(clippy::allow_attributes, dead_code, reason = "not used by all tests")]
impl Flag {
	/// Creates a new [`Flag`].
	pub(crate) fn new() -> Self {
		Self(Arc::new(Inner {
			waker: AtomicWaker::new(),
			set: AtomicBool::new(false),
		}))
	}

	/// Will wake up any thread waiting on this [`Flag`].
	///
	/// Any thread awaiting this [`Flag`] will wake up immediately.
	pub(crate) fn signal(&self) {
		self.0.set.store(true, Ordering::Relaxed);
		self.0.waker.wake();
	}
}

#[cfg(all(target_feature = "atomics", feature = "std"))]
impl Future for Flag {
	type Output = ();

	fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
		// Short-circuit.
		if self.0.set.load(Ordering::Relaxed) {
			return Poll::Ready(());
		}

		self.0.waker.register(cx.waker());

		if self.0.set.load(Ordering::Relaxed) {
			Poll::Ready(())
		} else {
			Poll::Pending
		}
	}
}
