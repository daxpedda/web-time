use std::future::Future;
use std::pin::Pin;
#[cfg(target_feature = "atomics")]
use std::sync::atomic::{AtomicBool, Ordering};
#[cfg(target_feature = "atomics")]
use std::sync::Arc;
use std::task::{ready, Context, Poll};
use std::time::Duration;

#[cfg(target_feature = "atomics")]
use futures_util::task::AtomicWaker;
use js_sys::Promise;
#[cfg(target_feature = "atomics")]
use wasm_bindgen::prelude::wasm_bindgen;
#[cfg(target_feature = "atomics")]
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
#[cfg(target_feature = "atomics")]
use web_sys::{DedicatedWorkerGlobalScope, Window};

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
	#[cfg(target_feature = "atomics")]
	enum Global {
		Window(Window),
		DedicatedWorker(DedicatedWorkerGlobalScope),
	}

	#[cfg(target_feature = "atomics")]
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

			#[cfg(not(target_feature = "atomics"))]
			web_sys::window()
				.unwrap()
				.set_timeout_with_callback_and_timeout_and_arguments_0(&resolve, duration)
				.unwrap();

			#[cfg(target_feature = "atomics")]
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
#[cfg(target_feature = "atomics")]
#[derive(Clone)]
pub(crate) struct Flag(Arc<Inner>);

#[cfg(target_feature = "atomics")]
struct Inner {
	waker: AtomicWaker,
	set: AtomicBool,
}

#[cfg(target_feature = "atomics")]
#[allow(unused)]
impl Flag {
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

#[cfg(target_feature = "atomics")]
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
