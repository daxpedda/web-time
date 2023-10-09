# Contributing

Thank you for contributing!

## Wasm Atomics

This crate has some code paths that depend on Wasm Atomics, which has some prerequisites to compile:
- Rust nightly.
- The `rust-src` component.
- Cargo's [`build-std`].
- The `atomics` and `bulk-memory` target features.

Example usage:
```sh
# Installing Rust nightly and necessary components:
rustup toolchain install nightly --target wasm32-unknown-unknown --component rust-src
# Example `cargo build` usage:
RUSTFLAGS=-Ctarget-feature=+atomics,+bulk-memory cargo +nightly build -Zbuild-std=panic_abort,std --target wasm32-unknown-unknown
```

### Rust Analyzer

To get proper diagnostics for Rust Atomics it can be helpful to configure Rust Analyzer to support that.

Here is an example configuration for Visual Studio Code:
```json
"rust-analyzer.cargo.target": "wasm32-unknown-unknown",
"rust-analyzer.cargo.extraArgs": [
    "-Zbuild-std=panic_abort,std"
],
"rust-analyzer.cargo.extraEnv": {
    "RUSTUP_TOOLCHAIN": "nightly",
    // Don't overwrite `RUSTFLAGS` from `.cargo/config.toml`.
    "CARGO_TARGET_WASM32_UNKNOWN_UNKNOWN_RUSTFLAGS": "-Ctarget-feature=+atomics,+bulk-memory"
},
```

## Testing

Tests are run as usual, but tests that require Wasm Atomics can be run like this:
```sh
RUSTFLAGS=-Ctarget-feature=+atomics,+bulk-memory cargo +nightly test -Zbuild-std=panic_abort,std --target wasm32-unknown-unknown
```

Additionally, keep in mind that usage of [`#[should_panic]`](`should_panic`) is known to cause browsers to get stuck because of the lack of unwinding support.

The current workaround is to split tests using `await` into separate test targets.

[`build-std`]: https://doc.rust-lang.org/1.73.0/cargo/reference/unstable.html#build-std
[`should_panic`]: https://doc.rust-lang.org/1.73.0/reference/attributes/testing.html#the-should_panic-attribute
