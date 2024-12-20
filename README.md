# web-time

[![Crates.io Version](https://img.shields.io/crates/v/web-time.svg)](https://crates.io/crates/web-time)
[![Live Build Status](https://img.shields.io/github/check-runs/daxpedda/web-time/main?label=CI)](https://github.com/daxpedda/web-time/actions?query=branch%3Amain)
[![Docs.rs Documentation](https://img.shields.io/docsrs/web-time?label=docs.rs)](https://docs.rs/web-time/1.1.0)
[![Main Documentation](https://img.shields.io/github/actions/workflow/status/daxpedda/web-time/coverage-documentation.yaml?branch=main&label=main%20docs)](https://daxpedda.github.io/web-time/doc/web_time)
[![Test Coverage](https://img.shields.io/badge/dynamic/json?url=https%3A%2F%2Fdaxpedda.github.io%2Fweb-time%2Fcoverage%2Fcoverage.json&query=%24.coverage&label=Test%20Coverage)](https://daxpedda.github.io/web-time/coverage)

## Description

Complete drop-in replacement for [`std::time`] that works in browsers.

Currently [`Instant::now()`] and [`SystemTime::now()`] will simply panic when using the
`wasm32-unknown-unknown` target. This implementation uses [`Performance.now()`] for [`Instant`] and
[`Date.now()`] for [`SystemTime`] to offer a drop-in replacement that works in browsers.

At the same time the library will simply re-export [`std::time`] when not using the
`wasm32-unknown-unknown` or `wasm32v1-none` target and will not pull in any dependencies.

Additionally, if compiled with `target-feature = "atomics"` it will synchronize the timestamps to
account for different context's, like web workers. See [`Performance.timeOrigin`] for more
information.

## Target

This library specifically targets browsers, that support [`Performance.now()`], with the
`wasm32-unknown-unknown` or `wasm32v1-none` target. Emscripten is not supported. WASI doesn't
require support as it has it's own native API to deal with [`std::time`].

Furthermore it depends on [`wasm-bindgen`], which is required. This library will continue to depend
on it until a viable alternative presents itself, in which case multiple ecosystems could be
supported.

## Note

### Ticking during sleep

Currently a known bug is affecting browsers on operating system other then Windows. This bug
prevents [`Instant`] from continuing to tick when the context is asleep. While this doesn't conflict
with Rusts requirements of [`Instant`], by chance Rust's Std
[has the same problem](https://github.com/rust-lang/rust/issues/79462).

See
[the MDN documentation on this](https://developer.mozilla.org/en-US/docs/Web/API/Performance/now#ticking_during_sleep)
for more information.

### Context support

The implementation of [`Instant::now()`] relies on the availability of the [`Performance` object], a
lack thereof will cause a panic. This can happen if called from a [worklet].

## Usage

You can simply import the types you need:

```rust
use web_time::{Instant, SystemTime};

let now = Instant::now();
let time = SystemTime::now();
```

Using `-Ctarget-feature=+nontrapping-fptoint` will improve the performance of [`Instant::now()`] and
[`SystemTime::now()`], but the vast majority of the time is still spent going through JS.

## Features

### `std` (enabled by default)

Enables the corresponding crate feature in all dependencies and allows for some optimized
instruction output.

Without this crate feature compilation the standard library is not included. Has no effect on
targets other then `wasm32-unknown-unknown` or `wasm32v1-none`.

### `msrv` (enabled by default)

Allows `web-time` to make use of features only available in higher MSRVs. This offers compile-time
detection and does not break compilation when enabled with the crates MSRV.

- Rust v1.77 + `std`: Enables the use of the [`f64.nearest`] instruction. Which will significantly
  reduce the instruction count for [`Instant::now()`].
- Rust Nightly: Enables the use of the [`f64.trunc`] and [`f64.nearest`] instruction. Which will
  significantly reduce the instruction count for [`Instant::now()`].

### `serde`

Implements [`serde::Deserialize`] and [`serde::Serialize`] for [`SystemTime`].

## Conditional Configurations

### `docsrs`

This requires Rust nightly and enhances the documentation. It must only be used with `RUSTDOCFLAGS`,
not with `RUSTFLAGS`.

## MSRV Policy

The MSRV is v1.60. Changes to the MSRV will be accompanied by a minor version bump.

## Changelog

See the [CHANGELOG] file for details.

## Contributing

See the [CONTRIBUTING] file for details.

## Attribution

Inspiration was taken from the [instant](https://github.com/sebcrozet/instant/tree/v0.1.12) project.

Additional insight was taken from the [time](https://github.com/time-rs/time/tree/v0.3.20) project.

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE] or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT] or <http://opensource.org/licenses/MIT>)

at your option.

### Copyright

A majority of the code and documentation was taken from [`std::time`]. For license information see
[#License](https://github.com/rust-lang/rust/tree/1.68.1#license).

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the
work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.

[CHANGELOG]: https://github.com/daxpedda/web-time/blob/v1.1.0/CHANGELOG.md
[CONTRIBUTING]: https://github.com/daxpedda/web-time/blob/v1.1.0/CONTRIBUTING.md
[LICENSE-MIT]: https://github.com/daxpedda/web-time/blob/v1.1.0/LICENSE-MIT
[LICENSE-APACHE]: https://github.com/daxpedda/web-time/blob/v1.1.0/LICENSE-APACHE
[worklet]: https://developer.mozilla.org/en-US/docs/Web/API/Worklet
[`Date.now()`]:
	https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/now
[`Instant`]: https://doc.rust-lang.org/std/time/struct.Instant.html
[`Instant::now()`]: https://doc.rust-lang.org/std/time/struct.Instant.html#method.now
[`SystemTime`]: https://doc.rust-lang.org/std/time/struct.SystemTime.html
[`SystemTime::now()`]: https://doc.rust-lang.org/std/time/struct.SystemTime.html#method.now
[`std::time`]: https://doc.rust-lang.org/std/time/
[`performance.now()`]: https://developer.mozilla.org/en-US/docs/Web/API/Performance/now
[`Performance.timeOrigin`]: https://developer.mozilla.org/en-US/docs/Web/API/Performance/timeOrigin
[`Performance` object]: https://developer.mozilla.org/en-US/docs/Web/API/performance_property
[`serde::Deserialize`]: https://docs.rs/serde/1/serde/trait.Deserialize.html
[`serde::Serialize`]: https://docs.rs/serde/1/serde/trait.Serialize.html
[`wasm-bindgen`]: https://crates.io/crates/wasm-bindgen
[`f64.nearest`]:
	https://webassembly.github.io/spec/core/syntax/instructions.html#syntax-instr-numeric
[`f64.trunc`]: https://webassembly.github.io/spec/core/syntax/instructions.html#syntax-instr-numeric
