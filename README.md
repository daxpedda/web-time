# web-time

[![Crates.io Version](https://img.shields.io/crates/v/web-time.svg)](https://crates.io/crates/web-time)
[![Live Build Status](https://img.shields.io/github/checks-status/daxpedda/web-time/main?label=CI)](https://github.com/daxpedda/web-time/actions?query=branch%3Amain)
[![Docs.rs Documentation](https://img.shields.io/docsrs/web-time)](https://docs.rs/crate/web-time)
[![Main Documentation](https://img.shields.io/github/actions/workflow/status/daxpedda/web-time/documentation.yml?branch=main)](https://daxpedda.github.io/web-time/web_time/index.html)

## Description

Complete drop-in replacement for [`std::time`] that works in the browser.

Currently [`Instant`] and [`SystemTime`] will simply panic when using the
`wasm32-unknown-unknown` target. This implementation uses
[`Performance.now()`] to offer a drop-in replacement that works in the
browser.

At the same time the library will simply re-export [`std::time`] when not
using the `wasm32-unknown-unknown` target and will also not pull in any
dependencies.

Additionally, if compiled with `target-feature = "atomics"` it will
synchronize the timestamps to account for different context's, like web
workers. See [`Performance.timeOrigin`] for more information.

## Target

This library specifically targets browsers, that support
[`Performance.now()`], with the `wasm32-unknown-unknown` target. Emscripten
is not supported. WASI doesn't require support as it has it's own native API
to deal with [`std::time`].

Furthermore it depends on [`wasm-bindgen`], which is required. This library
will continue to depend on it until a viable alternative presents itself, in
which case multiple ecosystems could be supported.

## Note

Currently a known bug is affecting browsers on operating system other then
Windows. This bug prevents [`Instant`] from continuing to tick when the
context is asleep. This doesn't necessarily conflict with Rusts requirements
of [`Instant`], but might still be unexpected.

See [the MDN documentation on this](https://developer.mozilla.org/en-US/docs/Web/API/Performance/now#ticking_during_sleep) for more information.

## Usage

You can simply import the types you need:
```rust
use web_time::{Instant, SystemTime};

let now = Instant::now();
let time = SystemTime::now();
```

## MSRV

As this library heavily relies on [`wasm-bindgen`] the MSRV depends on it.
At the point of time this was written the MSRV is 1.56.

## Alternatives

[instant](https://crates.io/crates/instant) [![Crates.io](https://img.shields.io/crates/v/instant.svg)](https://crates.io/crates/instant) is a popular alternative! However the API it implements doesn't match [`std::time`] exactly.

## Changelog

See the [CHANGELOG] file for details.

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE] or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT] or <http://opensource.org/licenses/MIT>)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.

[CHANGELOG]: https://github.com/daxpedda/web-time/blob/v0.1.0/CHANGELOG.md
[LICENSE-MIT]: https://github.com/daxpedda/web-time/blob/v0.1.0/LICENSE-MIT
[LICENSE-APACHE]: https://github.com/daxpedda/web-time/blob/v0.1.0/LICENSE-APACHE
[`Instant`]: https://doc.rust-lang.org/std/time/struct.Instant.html
[`SystemTime`]: https://doc.rust-lang.org/std/time/struct.SystemTime.html
[`std::time`]: https://doc.rust-lang.org/stable/std/time/
[`performance.now()`]: https://developer.mozilla.org/en-US/docs/Web/API/Performance/now
[`Performance.timeOrigin`]: https://developer.mozilla.org/en-US/docs/Web/API/Performance/timeOrigin
[`wasm-bindgen`]: https://crates.io/crates/wasm-bindgen
