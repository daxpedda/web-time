//! A crate for running tests on Web without the default test harness.

#![cfg_attr(all(target_arch = "wasm32", not(feature = "std")), no_std)]
#![cfg_attr(all(test, target_arch = "wasm32"), no_main)]

#[cfg(all(target_arch = "wasm32", not(feature = "std")))]
use wasm_bindgen_test as _;

#[cfg(all(test, not(target_arch = "wasm32")))]
fn main() {}

#[cfg(all(target_arch = "wasm32", not(feature = "std")))]
#[expect(
	unsafe_code,
	reason = "no way to implement `GlobalAlloc` without unsafe"
)]
mod allocator {
	//! Implementing [`GlobalAlloc`].
	//!
	//! See <https://github.com/rust-lang/rust/blob/1.82.0/library/std/src/sys/alloc/wasm.rs>.

	use core::alloc::{GlobalAlloc, Layout};

	use dlmalloc::Dlmalloc;

	/// The allocator.
	static mut DLMALLOC: Dlmalloc = Dlmalloc::new();
	/// Global allocator.
	#[global_allocator]
	static ALLOC: System = System;

	/// Implementing [`GlobalAlloc`].
	struct System;

	// SAFETY: we mostly rely on `dlmalloc` for safety.
	unsafe impl GlobalAlloc for System {
		#[inline]
		unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
			let _lock = lock::lock();
			// SAFETY: `DLMALLOC` access is guaranteed to be safe because the lock gives us
			// unique and non-reentrant access. Calling `malloc()` is safe because
			// preconditions on this function match the trait method preconditions.
			unsafe { (*core::ptr::addr_of_mut!(DLMALLOC)).malloc(layout.size(), layout.align()) }
		}

		#[inline]
		unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
			let _lock = lock::lock();
			// SAFETY: `DLMALLOC` access is guaranteed to be safe because the lock gives us
			// unique and non-reentrant access. Calling `calloc()` is safe because
			// preconditions on this function match the trait method preconditions.
			unsafe { (*core::ptr::addr_of_mut!(DLMALLOC)).calloc(layout.size(), layout.align()) }
		}

		#[inline]
		unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
			let _lock = lock::lock();
			// SAFETY: `DLMALLOC` access is guaranteed to be safe because the lock gives us
			// unique and non-reentrant access. Calling `free()` is safe because
			// preconditions on this function match the trait method preconditions.
			unsafe { (*core::ptr::addr_of_mut!(DLMALLOC)).free(ptr, layout.size(), layout.align()) }
		}

		#[inline]
		unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
			let _lock = lock::lock();
			// SAFETY: `DLMALLOC` access is guaranteed to be safe because the lock gives us
			// unique and non-reentrant access. Calling `realloc()` is safe because
			// preconditions on this function match the trait method preconditions.
			unsafe {
				(*core::ptr::addr_of_mut!(DLMALLOC)).realloc(
					ptr,
					layout.size(),
					layout.align(),
					new_size,
				)
			}
		}
	}

	/// The lock guard.
	// CHANGED: Add `#[must_used]` for safety.
	#[must_use = "if unused it will immediately unlock"]
	struct DropLock;

	/// Lock implementation.
	#[cfg(target_feature = "atomics")]
	mod lock {
		use core::sync::atomic::{AtomicBool, Ordering};

		use super::DropLock;

		/// The lock flag.
		// CHANGED: using an `AtomicBool` instead of an `AtomicU32`.
		static LOCKED: AtomicBool = AtomicBool::new(false);

		/// Locks the thread until available.
		pub(super) fn lock() -> DropLock {
			loop {
				if !LOCKED.swap(true, Ordering::Acquire) {
					return DropLock;
				}
			}
		}

		impl Drop for DropLock {
			fn drop(&mut self) {
				LOCKED.swap(false, Ordering::Release);
			}
		}
	}

	/// Empty lock implementation when threads are not available.
	#[cfg(not(target_feature = "atomics"))]
	mod lock {
		use super::DropLock;

		/// Locks the thread until available.
		#[expect(
			clippy::missing_const_for_fn,
			reason = "compatibility with non-atomic lock"
		)]
		pub(super) fn lock() -> DropLock {
			DropLock
		}
	}
}
