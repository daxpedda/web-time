# Contributing

Thank you for contributing!

## [`no_std`]

This crate has some code paths that depend on [`no_std`], which can be compiled with Cargo by using
`--no-default-features`. Additionally, its best to use the `wasm32v1-none` target to ensure the
standard library isn't included in any dependency.

Example usage:

```sh
cargo +nightly build --target wasm32v1-none --no-default-features
```

### Rust Analyzer

To get proper diagnostics for [`no_std`] it can be helpful to configure Rust Analyzer to support
that.

Here is an example configuration for Visual Studio Code:

```json
"rust-analyzer.cargo.target": "wasm32v1-none",
"rust-analyzer.cargo.noDefaultFeatures": true,
"rust-analyzer.cargo.extraEnv": {
    "RUSTUP_TOOLCHAIN": "nightly",
},
```

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
RUSTFLAGS=-Ctarget-feature=+atomics,+bulk-memory cargo +nightly build --target wasm32-unknown-unknown -Zbuild-std=panic_abort,std
# Example `no_std` `cargo build` usage:
RUSTFLAGS=-Ctarget-feature=+atomics,+bulk-memory cargo +nightly build --target wasm32v1-none -Zbuild-std=core,alloc --no-default-features
```

[`build-std`]: https://doc.rust-lang.org/1.73.0/cargo/reference/unstable.html#build-std

### Rust Analyzer

To get proper diagnostics for Rust Atomics it can be helpful to configure Rust Analyzer to support
that.

Here is an example configuration for Visual Studio Code:

```json
"rust-analyzer.cargo.target": "wasm32-unknown-unknown",
"rust-analyzer.cargo.extraArgs": [
    "-Zbuild-std=panic_abort,std"
],
"rust-analyzer.cargo.extraEnv": {
    "RUSTUP_TOOLCHAIN": "nightly",
    "RUSTFLAGS": "-Ctarget-feature=+atomics,+bulk-memory"
},
```

Or with [`no_std`]:

```json
"rust-analyzer.cargo.target": "wasm32v1-none",
"rust-analyzer.cargo.noDefaultFeatures": true,
"rust-analyzer.cargo.extraArgs": [
    "-Zbuild-std=core,alloc"
],
"rust-analyzer.cargo.extraEnv": {
    "RUSTUP_TOOLCHAIN": "nightly",
    "RUSTFLAGS": "-Ctarget-feature=+atomics,+bulk-memory"
},
```

## Testing

Tests are run as usual. But integration tests have a special setup to support [`no_std`].

### Run

To run integration tests just use `--workspace`:

```sh
# Run tests for native.
cargo test --workspace
# Run tests for Wasm.
cargo test --workspace --target wasm32-unknown-unknown
# Run tests for `no_std`.
cargo +nightly test --workspace --target wasm32v1-none --no-default-features
# Run tests for Wasm atomics.
RUSTFLAGS=-Ctarget-feature=+atomics,+bulk-memory cargo +nightly test --workspace --target wasm32-unknown-unknown -Zbuild-std=panic_abort,std
```

Make sure not to use `--all-features`.

### Implement

To implement integration tests, you have to understand the setup. [`no_std`] support requires the
[test harness] has to be disabled. However, to keep the [test harness] enabled for native tests, the
same tests are split into two [test targets]. These are defined in the `tests-native` and
`tests-web` crate for each target respectively. The [test targets] are then enabled, depending on
the target, via the `run` crate feature.

So to add a new integration test the following [test targets] have to be added:

`tests-web/Cargo.toml`:

```toml
[[test]]
harness = false
name = "web_new_test"
path = "../tests/new_test.rs"
required-features = ["run"]
```

`tests-native/Cargo.toml`:

```toml
[[test]]
name = "native_new_test"
path = "../tests/new_test.rs"
required-features = ["run"]
```

Additionally, keep in mind that usage of [`#[should_panic]`](`should_panic`) is known to cause
browsers to get stuck because of the lack of unwinding support.

The current workaround is to split tests using `await` into separate [test targets].

[`should_panic`]:
	https://doc.rust-lang.org/1.73.0/reference/attributes/testing.html#the-should_panic-attribute
[test harness]: https://doc.rust-lang.org/test
[test targets]: https://doc.rust-lang.org/1.82.0/cargo/reference/cargo-targets.html#tests

## Benchmark

The only benchmark is marked as an example target because of the lack of Wasm support. To run it you
can use the following command:

```sh
RUSTFLAGS=-Ctarget-feature=+nontrapping-fptoint cargo build --workspace --example benchmark --target wasm32-unknown-unknown --profile bench
wasm-bindgen --out-dir benches --target web --no-typescript target/wasm32-unknown-unknown/release/examples/benchmark.wasm
```

The `benches` folder then needs to be hosted by a HTTP server to run it in a browser.

Optionally `wasm-opt` could be added as well:

```sh
wasm-opt benches/benchmark_bg.wasm -o benches/benchmark_bg.wasm -O4
```

## Test Coverage

The process to generate WebAssembly test coverage is quite involved, see the [`wasm-bindgen` Guide
on the matter][1]. If you open a PR the "Coverage & Documentation" CI workflow will generate test
coverage data for you and will post a summary via a [Job Summary][2], but you can also download the
full test coverage data via an artifact called `test-coverage`.

[1]: https://rustwasm.github.io/docs/wasm-bindgen/wasm-bindgen-test/coverage.html
[2]:
	https://docs.github.com/actions/writing-workflows/choosing-what-your-workflow-does/workflow-commands-for-github-actions#adding-a-job-summary

If you want to generate test coverage locally, here is an example shell script that you can use:

```sh
# Single-threaded test run.
st () {
    RUSTFLAGS="-Cinstrument-coverage -Zcoverage-options=condition -Zno-profiler-runtime --emit=llvm-ir --cfg=wasm_bindgen_unstable_test_coverage" cargo +nightly test --workspace --features serde --target wasm32-unknown-unknown --tests $@
}

# Multi-threaded test run.
mt () {
    CFLAGS_wasm32_unknown_unknown="-matomics -mbulk-memory" RUSTFLAGS="-Cinstrument-coverage -Zcoverage-options=condition -Zno-profiler-runtime --emit=llvm-ir --cfg=wasm_bindgen_unstable_test_coverage -Ctarget-feature=+atomics,+bulk-memory" cargo +nightly test --workspace --features serde --target wasm32-unknown-unknown --tests $@
}

# To collect object files.
objects=()

# Run tests and adjust LLVM IR.
test () {
    local command=$1
    local path=$2

    # Run tests.
    mkdir -p coverage-input/$path
    WASM_BINDGEN_USE_BROWSER=1 CHROMEDRIVER=chromedriver WASM_BINDGEN_UNSTABLE_TEST_PROFRAW_OUT=$(realpath coverage-input/$path) $command ${@:3}

    local crate_name=web_time
    local IFS=$'\n'
    for file in $(
        # Extract path to artifacts.
        $command ${@:3} --no-run --message-format=json | \
        jq -r "select(.reason == \"compiler-artifact\") | (select(.target.kind == [\"test\"]) // select(.target.name == \"$crate_name\")) | .filenames[0]"
    )
    do
        # Get the path to the LLVM IR files instead of the rlib and Wasm artifacts.
        local base

        if [[ ${file##*.} == "rlib" ]]; then
            base=$(basename $file .rlib)
            file=$(dirname $file)/${base#"lib"}.ll
        else
            file=$(dirname $file)/$(basename $file .wasm).ll
        fi

        # Compile LLVM IR files to object files.
        local output=coverage-input/$path/$(basename $file .ll).o
        clang-19 $file -Wno-override-module -c -o $output
        objects+=(-object $output)
    done
}

test st 'st'
test st 'st-no_std' --no-default-features
test mt 'mt' -Zbuild-std=panic_abort,std
test mt 'mt-no_std' -Zbuild-std=core,alloc --no-default-features

# Merge all generated `*.profraw` files.
rust-profdata merge -sparse coverage-input/*/*.profraw -o coverage-input/coverage.profdata
# Finally generate coverage information.
rust-cov show -show-instantiations=false -output-dir coverage-output -format=html -instr-profile=coverage-input/coverage.profdata ${objects[@]} -sources src
```

[`no_std`]: https://doc.rust-lang.org/1.82.0/reference/names/preludes.html#the-no_std-attribute
