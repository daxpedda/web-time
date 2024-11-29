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
use js_sys::Promise;
#[cfg(all(target_feature = "atomics", feature = "std"))]
use wasm_bindgen::prelude::wasm_bindgen;
#[cfg(all(target_feature = "atomics", feature = "std"))]
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
#[cfg(all(target_feature = "atomics", feature = "std"))]
use web_sys::{DedicatedWorkerGlobalScope, Window};

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
	/// Holding the [global object](https://developer.mozilla.org/en-US/docs/Glossary/Global_object).
	#[cfg(all(target_feature = "atomics", feature = "std"))]
	enum Global {
		/// The window.
		Window(Window),
		/// A dedicated worker.
		DedicatedWorker(DedicatedWorkerGlobalScope),
	}

	#[cfg(all(target_feature = "atomics", feature = "std"))]
	thread_local! {
		/// Cached [`Global`].
		static GLOBAL: Global = {
			#[wasm_bindgen]
			extern "C" {
				type SleepGlobal;

				#[wasm_bindgen(method, getter, js_name = Window)]
				fn window(this: &SleepGlobal) -> JsValue;

				#[wasm_bindgen(method, getter, js_name = DedicatedWorkerGlobalScope)]
				fn worker(this: &SleepGlobal) -> JsValue;
			}

			let global: SleepGlobal = js_sys::global().unchecked_into();

			if !global.window().is_undefined() {
				Global::Window(global.unchecked_into())
			} else if !global.worker().is_undefined() {
				Global::DedicatedWorker(global.unchecked_into())
			} else {
				unreachable!("only supported in a browser or web worker")
			}
		};
	}

	let future =
		JsFuture::from(Promise::new(&mut |resolve, _| {
			let duration = duration.as_millis().try_into().unwrap();

			#[cfg(not(all(target_feature = "atomics", feature = "std")))]
			web_sys::window()
				.unwrap()
				.set_timeout_with_callback_and_timeout_and_arguments_0(&resolve, duration)
				.unwrap();

			#[cfg(all(target_feature = "atomics", feature = "std"))]
			GLOBAL
				.with(|global| match global {
					Global::Window(window) => window
						.set_timeout_with_callback_and_timeout_and_arguments_0(&resolve, duration),
					Global::DedicatedWorker(worker) => worker
						.set_timeout_with_callback_and_timeout_and_arguments_0(&resolve, duration),
				})
				.unwrap();
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
