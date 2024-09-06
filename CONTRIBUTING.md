# Contributing

Thank you for contributing!

## Wasm Atomics

This crate has some code paths that depend on Wasm Atomics, which has some prerequisites to compile:

- Rust nightly.
- The `rust-src` component.
- Cargo's [`build-std`].
- The `atomics` and `bulk-memory` target features.

These are set using [`rust-toolchain.toml`](./rust-toolchain.toml).

### Rust Analyzer

To get proper diagnostics for Rust Atomics it can be helpful to configure Rust Analyzer to support
that.

It takes the settings from `rust-toolchain.toml`, but we also need to specify a target, as seen for vscode in [.vscode/settings.json](./.vscode/settings.json).

## Testing

Tests are run as usual, but also rewuire an explicit target:

```sh
cargo test --target wasm32-unknown-unknown
```

Additionally, keep in mind that usage of [`#[should_panic]`](`should_panic`) is known to cause
browsers to get stuck because of the lack of unwinding support.

The current workaround is to split tests using `await` into separate test targets.

[`build-std`]: https://doc.rust-lang.org/1.73.0/cargo/reference/unstable.html#build-std
[`should_panic`]:
	https://doc.rust-lang.org/1.73.0/reference/attributes/testing.html#the-should_panic-attribute

## Benchmark

The only benchmark is marked as an example target because of the lack of Wasm support. To run it you
can use the following command:

```sh
cargo build --example benchmark --target wasm32-unknown-unknown --profile bench
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
    CARGO_HOST_RUSTFLAGS=--cfg=wasm_bindgen_unstable_test_coverage RUSTFLAGS="-Cinstrument-coverage -Zcoverage-options=condition -Zno-profiler-runtime --emit=llvm-ir --cfg=wasm_bindgen_unstable_test_coverage" cargo +nightly test --all-features --target wasm32-unknown-unknown -Ztarget-applies-to-host -Zhost-config --tests $@
}

# Multi-threaded test run.
mt () {
    CFLAGS_wasm32_unknown_unknown="-matomics -mbulk-memory" CARGO_HOST_RUSTFLAGS=--cfg=wasm_bindgen_unstable_test_coverage RUSTFLAGS="-Cinstrument-coverage -Zcoverage-options=condition -Zno-profiler-runtime --emit=llvm-ir --cfg=wasm_bindgen_unstable_test_coverage -Ctarget-feature=+atomics,+bulk-memory" cargo +nightly test --all-features --target wasm32-unknown-unknown -Ztarget-applies-to-host -Zhost-config -Zbuild-std=panic_abort,std --tests $@
}

# To collect object files.
objects=()

# Run tests and adjust LLVM IR.
test () {
    local command=$1
    local path=$2

    # Run tests.
    mkdir -p coverage-input/$path
    CHROMEDRIVER=chromedriver WASM_BINDGEN_UNSTABLE_TEST_PROFRAW_OUT=coverage-input/$path $command

    local crate_name=web_time
    local IFS=$'\n'
    for file in $(
        # Extract path to artifacts.
        $command --no-run --message-format=json | \
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

        # Copy LLVM IR files.
        local input=coverage-input/$path/$(basename $file)
        cp $file $input

        # Adjust LLVM IR files.
        perl -i -p0e 's/(^define.*?$).*?^}/$1\nstart:\n  unreachable\n}/gms' $input
        local counter=1
        while (( counter != 0 )); do
            counter=$(perl -i -p0e '$c+= s/(^(define|declare)(,? [^\n ]+)*),? range\(.*?\)/$1/gm; END{print "$c"}' $input)
        done

        # Compile LLVM IR files to object files.
        local output=coverage-input/$path/$(basename $file .ll).o
        clang-18 $input -Wno-override-module -c -o $output
        objects+=(-object $output)
    done
}

test st 'st'
test mt 'mt'

# Merge all generated `*.profraw` files.
llvm-profdata-18 merge -sparse coverage-input/*/*.profraw -o coverage-input/coverage.profdata
# Finally generate coverage information.
llvm-cov-18 show -show-instantiations=false -output-dir coverage-output -format=html -instr-profile=coverage-input/coverage.profdata ${objects[@]} -sources src
```
