//! Benchmark to compare different conversion methods.

#[cfg(not(all(
	target_family = "wasm",
	not(any(target_os = "emscripten", target_os = "wasi"))
)))]
fn main() {
	panic!("made to run under `wasm32-unknown-unknown`")
}

#[cfg(all(
	target_family = "wasm",
	not(any(target_os = "emscripten", target_os = "wasi"))
))]
#[allow(
	clippy::too_many_lines,
	clippy::as_conversions,
	clippy::cast_possible_truncation,
	clippy::cast_sign_loss,
	clippy::min_ident_chars,
	clippy::unwrap_used
)]
fn main() {
	use std::time::Duration;
	use std::{array, hint};

	use rand::distributions::Uniform;
	use rand::Rng;
	use wasm_bindgen::closure::Closure;
	use wasm_bindgen::JsCast;
	use web_sys::{HtmlTableElement, HtmlTableRowElement, Performance};

	/// Copy from `src/web/instant.rs`. This should be kept in sync.
	#[allow(clippy::missing_docs_in_private_items)]
	fn custom_conversion(time_stamp: f64) -> Duration {
		// CHANGED: 1G to 1M.
		const NANOS_PER_MILLI: u32 = 1_000_000;

		// See <https://doc.rust-lang.org/1.73.0/src/core/time.rs.html#1477-1484>.
		const MANT_BITS: i16 = 52;
		const EXP_BITS: i16 = 11;
		const OFFSET: i16 = 44;

		// See <https://doc.rust-lang.org/1.73.0/src/core/time.rs.html#1262-1340>.
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
			#[allow(clippy::if_not_else)]
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
			#[allow(clippy::if_not_else)]
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

		let secs = millis / 1000;
		let carry = millis - secs * 1000;
		let extra_nanos = carry * u64::from(NANOS_PER_MILLI);
		nanos += extra_nanos as u32;

		debug_assert!(
			nanos < 1_000_000_000,
			"impossible amount of nanoseconds found"
		);

		Duration::new(secs, nanos)
	}

	thread_local! {
		static PERFORMANCE: Performance = web_sys::window().unwrap().performance().unwrap();
	}

	let window = web_sys::window().unwrap();
	let document = window.document().unwrap();
	let body = document.body().unwrap();
	let table: HtmlTableElement = document.create_element("table").unwrap().unchecked_into();
	table.set_id("benchmark");
	body.append_child(&table).unwrap();

	let benchmark = |name: &str, f: fn(f64) -> Duration| {
		let table = table.clone();
		let name = name.to_owned();

		let closure = Closure::once_into_js(move || {
			let mut random = rand::thread_rng().sample_iter(Uniform::new_inclusive(
				0.,
				285_616. * 365. * 24. * 60. * 60. * 1000.,
			));

			let mut time = 0.;

			for _ in 0..10_000 {
				let time_stamps: [f64; 10_000] = array::from_fn(|_| random.next().unwrap());

				let start = PERFORMANCE.with(Performance::now);

				for time_stamp in time_stamps {
					hint::black_box(f(hint::black_box(time_stamp)));
				}

				time += PERFORMANCE.with(Performance::now) - start;
			}

			let time = time / 100.;

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

	benchmark("custom `f64` conversion", custom_conversion);
	benchmark("`Duration::from_millis()`", |time_stamp| {
		Duration::from_millis(time_stamp.trunc() as u64)
			+ Duration::from_nanos((time_stamp.fract() * 1.0e6) as u64)
	});
	benchmark("`Duration::from_millis()` with rounding", |time_stamp| {
		Duration::from_millis(time_stamp.trunc() as u64)
			+ Duration::from_nanos((time_stamp.fract() * 1.0e6).round() as u64)
	});
	benchmark("`Duration::from_secs_f64()`", |time_stamp| {
		Duration::from_secs_f64(time_stamp) / 1000
	});
	benchmark("`Duration::from_secs_f64()` with rounding", |time_stamp| {
		let time_stamp = Duration::from_secs_f64(time_stamp);
		let secs = time_stamp.as_secs() / 1000;
		let carry = time_stamp.as_secs() - secs * 1000;
		#[allow(clippy::as_conversions, clippy::cast_possible_truncation)]
		let extra_nanos = (carry * 1_000_000_000 / 1000) as u32;
		let nanos = time_stamp.subsec_micros()
			+ u32::from(time_stamp.subsec_nanos() % 1000 > 499)
			+ extra_nanos;
		Duration::new(secs, nanos)
	});
}
