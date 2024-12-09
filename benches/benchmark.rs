//! Benchmark to compare different conversion methods.

#![cfg_attr(not(feature = "std"), no_std, no_main)]
#![expect(
	clippy::as_conversions,
	clippy::cast_possible_truncation,
	clippy::cast_precision_loss,
	clippy::cast_sign_loss,
	clippy::unwrap_used,
	reason = "lots of math going on here"
)]

#[cfg(not(feature = "std"))]
extern crate alloc;

#[cfg(not(feature = "std"))]
use alloc::borrow::ToOwned;
#[cfg(not(feature = "std"))]
use alloc::{format, vec::Vec};
use core::time::Duration;
use core::{hint, iter};

use rand::distributions::Uniform;
use rand::rngs::{OsRng, StdRng};
use rand::{Rng, SeedableRng};
use tests_web as _;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use web_sys::{HtmlTableElement, HtmlTableRowElement};

/// Number of runs for the benchmark.
const RUNS: usize = 100_000_000;

/// Main function.
#[cfg_attr(not(feature = "std"), wasm_bindgen::prelude::wasm_bindgen(main))]
pub fn main() {
	let window = web_sys::window().unwrap();
	let performance = window.performance().unwrap();
	let document = window.document().unwrap();
	let body = document.body().unwrap();
	let table: HtmlTableElement = document.create_element("table").unwrap().unchecked_into();
	table.set_id("benchmark");
	body.append_child(&table).unwrap();

	let benchmark = |name: &str, run: fn(f64) -> Duration| {
		let performance = performance.clone();
		let table = table.clone();
		let name = name.to_owned();

		let closure = Closure::once_into_js(move || {
			// Range to maximum accurately representable integer.
			let mut random = StdRng::from_rng(OsRng)
				.unwrap()
				.sample_iter(Uniform::new_inclusive(
					0.,
					u64::pow(2, f64::MANTISSA_DIGITS) as f64,
				));

			let time_stamps: Vec<_> = iter::repeat_with(|| random.next().unwrap())
				.take(RUNS)
				.collect();

			let start = performance.now();

			for time_stamp in time_stamps {
				hint::black_box(run(time_stamp));
			}

			let time = performance.now() - start;
			let time = time / const { (RUNS / 1_000_000) as f64 };

			let row: HtmlTableRowElement = table.insert_row().unwrap().unchecked_into();
			let cell = row.insert_cell().unwrap();
			cell.set_text_content(Some(&name));
			cell.style().set_property("padding-right", "2em").unwrap();
			row.insert_cell()
				.unwrap()
				.set_text_content(Some(&format!("{time:.2}ns")));
		});

		window
			.set_timeout_with_callback(closure.unchecked_ref())
			.unwrap();
	};

	benchmark("custom `f64` conversion", adjusted_std);
	benchmark("`Duration::from_millis()`", |time_stamp| {
		Duration::from_millis(F64(time_stamp).trunc() as u64)
			+ Duration::from_nanos((F64(time_stamp).fract() * 1.0e6) as u64)
	});
	benchmark("`Duration::from_millis()` with rounding", |time_stamp| {
		Duration::from_millis(F64(time_stamp).trunc() as u64)
			+ Duration::from_nanos(F64(F64(time_stamp).fract() * 1.0e6).round() as u64)
	});
	benchmark("`Duration::from_secs_f64()`", |time_stamp| {
		Duration::from_secs_f64(time_stamp) / 1000
	});
	benchmark("`Duration::from_secs_f64()` with rounding", |time_stamp| {
		// Inspired by <https://github.com/rust-lang/rust/blob/1.83.0/library/core/src/time.rs#L822-L833>.
		/// Number of nanoseconds in a second.
		const NANOS_PER_SEC: u64 = 1_000_000_000;
		let rhs: u32 = 1000;
		let time_stamp = Duration::from_secs_f64(time_stamp);
		let (secs, extra_secs) = (
			time_stamp.as_secs() / u64::from(rhs),
			time_stamp.as_secs() % u64::from(rhs),
		);
		let (mut nanos, extra_nanos) = (
			time_stamp.subsec_nanos() / rhs,
			time_stamp.subsec_nanos() % rhs,
		);
		// CHANGED: Extracted part of the calculation into variable.
		let extra = extra_secs * NANOS_PER_SEC + u64::from(extra_nanos);
		nanos += u32::try_from(extra / u64::from(rhs)).unwrap();
		// CHANGED: Added rounding.
		nanos += u32::from(extra % u64::from(rhs) >= u64::from(rhs / 2));
		// CHANGED: Removed check that would fail because of the additional time added
		// by rounding.
		Duration::new(secs, nanos)
	});
}

/// [`f64`] `no_std` compatibility wrapper.
#[derive(Clone, Copy)]
struct F64(f64);

impl F64 {
	/// See [`f64::trunc()`].
	#[cfg(feature = "std")]
	#[expect(
		clippy::disallowed_methods,
		reason = "this is where the abstraction happens"
	)]
	fn trunc(self) -> f64 {
		self.0.trunc()
	}

	#[cfg(not(feature = "std"))]
	fn trunc(self) -> f64 {
		libm::trunc(self.0)
	}

	/// See [`f64::fract()`].
	#[cfg(feature = "std")]
	#[expect(
		clippy::disallowed_methods,
		reason = "this is where the abstraction happens"
	)]
	fn fract(self) -> f64 {
		self.0.fract()
	}

	#[cfg(not(feature = "std"))]
	fn fract(self) -> f64 {
		self.0 - self.trunc()
	}

	/// See [`f64::round()`].
	#[cfg(feature = "std")]
	#[expect(
		clippy::disallowed_methods,
		reason = "this is where the abstraction happens"
	)]
	#[cfg(feature = "std")]
	fn round(self) -> f64 {
		self.0.round()
	}

	#[cfg(not(feature = "std"))]
	fn round(self) -> f64 {
		libm::round(self.0)
	}
}

/// Adjusted [`Duration::from_secs_f64()`].
#[expect(warnings, reason = "code is copied")]
fn adjusted_std(time_stamp: f64) -> Duration {
	// CHANGED: 1G to 1M.
	const NANOS_PER_MILLI: u32 = 1_000_000;

	// See <https://github.com/rust-lang/rust/blob/1.83.0/library/core/src/time.rs#L1694-L1703>.
	const MANT_BITS: i16 = 52;
	const EXP_BITS: i16 = 11;
	const OFFSET: i16 = 44;

	// See <https://github.com/rust-lang/rust/blob/1.83.0/library/core/src/time.rs#L1480-L1558>.
	const MIN_EXP: i16 = 1 - (1_i16 << EXP_BITS) / 2;
	const MANT_MASK: u64 = (1 << MANT_BITS) - 1;
	const EXP_MASK: u64 = (1 << 1_i16 << EXP_BITS) - 1;

	assert!(
		time_stamp >= 0.0,
		"can not convert float seconds to Duration: value is negative"
	);

	let bits = time_stamp.to_bits();
	let mant = (bits & MANT_MASK) | (MANT_MASK + 1);
	let exp = ((bits >> MANT_BITS) & EXP_MASK) as i16 + MIN_EXP;

	// CHANGED: 31 to 21 bits, because the fractional part only handles
	// microseconds, not milliseconds.
	let (millis, mut nanos) = if exp < -21 {
		// the input represents less than 1ns and can not be rounded to it
		// CHANGED: Return early.
		return Duration::ZERO;
	} else if exp < 0 {
		// the input is less than 1 millisecond
		let t = <u128>::from(mant) << (OFFSET + exp);
		let nanos_offset = MANT_BITS + OFFSET;
		let nanos_tmp = u128::from(NANOS_PER_MILLI) * t;
		let nanos = (nanos_tmp >> nanos_offset) as u32;

		let rem_mask = (1 << nanos_offset) - 1;
		let rem_msb_mask = 1 << (nanos_offset - 1);
		let rem = nanos_tmp & rem_mask;
		let is_tie = rem == rem_msb_mask;
		let is_even = (nanos & 1) == 0;
		let rem_msb = nanos_tmp & rem_msb_mask == 0;
		let add_ns = !(rem_msb || (is_even && is_tie));

		let nanos = nanos + u32::from(add_ns);
		// CHANGED: Removed `f32` handling.
		// CHANGED: Return early.
		return if nanos != NANOS_PER_MILLI {
			Duration::new(0, nanos)
		} else {
			// CHANGED: Do second to millisecond conversion right here because of the early
			// return.
			Duration::from_millis(1)
		};
	} else if exp < MANT_BITS {
		let millis = mant >> (MANT_BITS - exp);
		let t = <u128>::from((mant << exp) & MANT_MASK);
		let nanos_offset = MANT_BITS;
		let nanos_tmp = <u128>::from(NANOS_PER_MILLI) * t;
		let nanos = (nanos_tmp >> nanos_offset) as u32;

		let rem_mask = (1 << nanos_offset) - 1;
		let rem_msb_mask = 1 << (nanos_offset - 1);
		let rem = nanos_tmp & rem_mask;
		let is_tie = rem == rem_msb_mask;
		let is_even = (nanos & 1) == 0;
		let rem_msb = nanos_tmp & rem_msb_mask == 0;
		let add_ns = !(rem_msb || (is_even && is_tie));

		let nanos = nanos + u32::from(add_ns);
		// CHANGED: Removed `f32` handling.
		if nanos != NANOS_PER_MILLI {
			(millis, nanos)
		} else {
			(millis + 1, 0)
		}
	}
	// NOTE: Theoretically milliseconds can be bigger then `u64` when trying to cover
	// `Duration::MAX`, but `Performance.now` is a monotonic clock, so we don't need to cover
	// that.
	else if exp < 64 {
		// the input has no fractional part
		let millis = mant << (exp - MANT_BITS);
		(millis, 0)
	} else {
		panic!("can not convert float seconds to Duration: value is either too big or NaN")
	};

	// Inspired by <https://github.com/rust-lang/rust/blob/1.83.0/library/core/src/time.rs#L822-L833>.
	/// Number of nanoseconds in a second.
	const NANOS_PER_SEC: u64 = 1_000_000_000;
	let rhs: u32 = 1000;

	let (secs, extra_secs) = (millis / u64::from(rhs), millis % u64::from(rhs));
	// CHANGED: Nanos were already calculated during the conversion.
	nanos += u32::try_from((extra_secs * NANOS_PER_SEC) / u64::from(rhs)).unwrap();

	debug_assert!(
		u64::from(nanos) < NANOS_PER_SEC,
		"impossible amount of nanoseconds found"
	);

	Duration::new(secs, nanos)
}
