//! Bindings to the JS API.

#[cfg(not(feature = "std"))]
use once_cell::unsync::Lazy;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::{JsCast, JsValue};

#[wasm_bindgen]
extern "C" {
	/// Type for the [global object](https://developer.mozilla.org/en-US/docs/Glossary/Global_object).
	type Global;

	/// Returns the [`Performance`](https://developer.mozilla.org/en-US/docs/Web/API/Performance) object.
	#[wasm_bindgen(method, getter)]
	fn performance(this: &Global) -> JsValue;

	/// Type for the [`Performance` object](https://developer.mozilla.org/en-US/docs/Web/API/Performance).
	type JsPerformance;

	/// Binding to [`Performance.now()`](https://developer.mozilla.org/en-US/docs/Web/API/Performance/now).
	#[wasm_bindgen(method)]
	fn now(this: &JsPerformance) -> f64;

	/// Binding to [`Performance.timeOrigin`](https://developer.mozilla.org/en-US/docs/Web/API/Performance/timeOrigin).
	#[cfg(target_feature = "atomics")]
	#[wasm_bindgen(method, getter, js_name = timeOrigin)]
	fn time_origin(this: &JsPerformance) -> f64;
}

/// Cached [`Performance` object](https://developer.mozilla.org/en-US/docs/Web/API/Performance).
pub(super) struct Performance;

impl Performance {
	/// Create the [`Performance` object](https://developer.mozilla.org/en-US/docs/Web/API/Performance).
	fn init() -> JsPerformance {
		let global: Global = js_sys::global().unchecked_into();
		let performance = global.performance();

		if performance.is_undefined() {
			panic!("`Performance` object not found")
		} else {
			performance.unchecked_into()
		}
	}

	/// Access to the underlying [`Performance` object](https://developer.mozilla.org/en-US/docs/Web/API/Performance).
	#[cfg(feature = "std")]
	fn with<R>(fun: impl FnOnce(&JsPerformance) -> R) -> R {
		thread_local! {
			static PERFORMANCE: JsPerformance = Performance::init();
		}

		PERFORMANCE.with(fun)
	}

	/// Access to the underlying [`Performance` object](https://developer.mozilla.org/en-US/docs/Web/API/Performance).
	#[cfg(not(feature = "std"))]
	fn with<R>(fun: impl FnOnce(&JsPerformance) -> R) -> R {
		/// `Send + Sync` wrapper for [`Lazy`].
		struct Wrapper<T>(T);

		#[cfg(not(target_feature = "atomics"))]
		#[allow(clippy::non_send_fields_in_send_ty, unsafe_code)]
		// SAFETY: only when no threads are supported.
		unsafe impl<T> Send for Wrapper<T> {}
		#[cfg(not(target_feature = "atomics"))]
		#[allow(clippy::non_send_fields_in_send_ty, unsafe_code)]
		// SAFETY: only when no threads are supported.
		unsafe impl<T> Sync for Wrapper<T> {}

		/// Cached [`Performance` object](https://developer.mozilla.org/en-US/docs/Web/API/Performance).
		#[cfg_attr(target_feature = "atomics", thread_local)]
		static PERFORMANCE: Wrapper<Lazy<JsPerformance>> = Wrapper(Lazy::new(Performance::init));

		#[allow(unsafe_code)]
		fun(&PERFORMANCE.0)
	}

	/// Calls [`Performance.now()`](https://developer.mozilla.org/en-US/docs/Web/API/Performance/now).
	pub(super) fn now() -> f64 {
		Self::with(JsPerformance::now)
	}
}

/// Cached [`Performance.timeOrigin`](https://developer.mozilla.org/en-US/docs/Web/API/Performance/timeOrigin).
#[cfg(target_feature = "atomics")]
pub(super) struct Origin;

#[cfg(target_feature = "atomics")]
impl Origin {
	/// Get [`Performance.timeOrigin`](https://developer.mozilla.org/en-US/docs/Web/API/Performance/timeOrigin).
	fn init() -> f64 {
		Performance::with(JsPerformance::time_origin)
	}

	/// Returns [`Performance.timeOrigin`](https://developer.mozilla.org/en-US/docs/Web/API/Performance/timeOrigin).
	#[cfg(feature = "std")]
	pub(super) fn get() -> f64 {
		thread_local! {
			static ORIGIN: f64 = Origin::init();
		}

		ORIGIN.with(|origin| *origin)
	}

	/// Returns [`Performance.timeOrigin`](https://developer.mozilla.org/en-US/docs/Web/API/Performance/timeOrigin).
	#[cfg(not(feature = "std"))]
	pub(super) fn get() -> f64 {
		#[thread_local]
		static ORIGIN: Lazy<f64> = Lazy::new(Origin::init);

		*ORIGIN
	}
}
