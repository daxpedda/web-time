//! A crate for running tests on Web without the default test harness.

#![cfg_attr(all(target_arch = "wasm32", not(feature = "std")), no_std)]
#![cfg_attr(all(test, target_arch = "wasm32"), no_main)]
#![cfg_attr(
	all(target_arch = "wasm32", target_feature = "atomics"),
	feature(thread_local)
)]

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
	use core::sync::atomic::{AtomicBool, Ordering};

	/// The allocator.
	static mut DLMALLOC: dlmalloc::Dlmalloc = dlmalloc::Dlmalloc::new();
	/// The lock flag.
	static LOCKED: AtomicBool = AtomicBool::new(false);
	/// Global allocator.
	#[global_allocator]
	static ALLOC: System = System;

	/// Implementing [`GlobalAlloc`].
	struct System;

	// SAFETY: we mostly rely on `dlmalloc` for safety.
	unsafe impl GlobalAlloc for System {
		#[inline]
		unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
			let _lock = lock();
			// SAFETY: `DLMALLOC` access is guaranteed to be safe because the lock gives us
			// unique and non-reentrant access. Calling `malloc()` is safe because
			// preconditions on this function match the trait method preconditions.
			unsafe { (*core::ptr::addr_of_mut!(DLMALLOC)).malloc(layout.size(), layout.align()) }
		}

		#[inline]
		unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
			let _lock = lock();
			// SAFETY: `DLMALLOC` access is guaranteed to be safe because the lock gives us
			// unique and non-reentrant access. Calling `calloc()` is safe because
			// preconditions on this function match the trait method preconditions.
			unsafe { (*core::ptr::addr_of_mut!(DLMALLOC)).calloc(layout.size(), layout.align()) }
		}

		#[inline]
		unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
			let _lock = lock();
			// SAFETY: `DLMALLOC` access is guaranteed to be safe because the lock gives us
			// unique and non-reentrant access. Calling `free()` is safe because
			// preconditions on this function match the trait method preconditions.
			unsafe { (*core::ptr::addr_of_mut!(DLMALLOC)).free(ptr, layout.size(), layout.align()) }
		}

		#[inline]
		unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
			let _lock = lock();
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
	struct DropLock;

	/// Create a [`DropLock`].
	fn lock() -> DropLock {
		while LOCKED
			.compare_exchange_weak(false, true, Ordering::Acquire, Ordering::Relaxed)
			.is_err()
		{}

		DropLock
	}

	impl Drop for DropLock {
		fn drop(&mut self) {
			LOCKED.swap(false, Ordering::Release);
		}
	}
}

#[cfg(all(target_arch = "wasm32", target_feature = "atomics"))]
#[expect(
	unsafe_code,
	reason = "no way to implement `critical_section` without unsafe"
)]
#[expect(
	clippy::no_mangle_with_rust_abi,
	reason = "from `critical_section::set_impl!` macro"
)]
mod critical_section {
	//! Implementing [`critical_section`].

	use core::cell::Cell;
	use core::sync::atomic::{AtomicBool, Ordering};

	use critical_section::Impl;

	/// The lock flag.
	static LOCKED: AtomicBool = AtomicBool::new(false);
	/// The thread local lock flag.
	#[thread_local]
	static LOCAL_LOCKED: Cell<bool> = Cell::new(false);

	critical_section::set_impl!(CriticalSection);

	/// Implementing [`critical_section::Impl`].
	struct CriticalSection;

	// SAFETY: the lock is safely implemented.
	unsafe impl Impl for CriticalSection {
		#[inline]
		unsafe fn acquire() -> bool {
			if LOCAL_LOCKED.get() {
				return true;
			}

			LOCAL_LOCKED.set(true);

			while LOCKED
				.compare_exchange_weak(false, true, Ordering::Acquire, Ordering::Relaxed)
				.is_err()
			{}

			false
		}

		#[inline]
		unsafe fn release(restore_state: bool) {
			if !restore_state {
				LOCKED.store(false, Ordering::Release);

				LOCAL_LOCKED.set(false);
			}
		}
	}
}
