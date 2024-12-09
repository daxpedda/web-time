# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/), and this project
adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- [`no_std`] support through a `std` crate feature.
- Support for the [`wasm32v1-none`] target.
- MSRV policy.
- A `msrv` crate feature that allows `web-time` to make use of features only available in higher
  MSRVs. This offers compile-time detection and does not break compilation when enabled with the
  crates MSRV.

  - Rust v1.77 + `std`: Enables the use of the [`f64.nearest`] instruction. Which will significantly
    reduce the instruction count for `Instant::now()`.
  - Rust Nightly: Enables the use of the [`f64.trunc`] and [`f64.nearest`] instruction. Which will
    significantly reduce the instruction count for `Instant::now()`.

### Changed

- Improve performance of `Instant::now()` by using `f64::round_ties_even()` instead of
  `f64::round()` internally.
- Removed `js-sys` dependency in favor of custom bindings.

### Fixed

- As `wasm64-*` is not supported, `web-time` now falls back to `std` instead of unnecessarily
  pulling in dependencies on these targets.

[`no_std`]: https://doc.rust-lang.org/1.82.0/reference/names/preludes.html#the-no_std-attribute
[`wasm32v1-none`]: https://doc.rust-lang.org/nightly/rustc/platform-support/wasm32v1-none.html
[`f64.nearest`]:
	https://webassembly.github.io/spec/core/syntax/instructions.html#syntax-instr-numeric
[`f64.trunc`]: https://webassembly.github.io/spec/core/syntax/instructions.html#syntax-instr-numeric

## [1.1.0] - 2024-03-01

### Added

- Serde de/serialization support for `SystemTime` through a `serde` crate feature.

## [1.0.0] - 2024-01-14

### Added

- V1 release.

## [0.2.4] - 2023-12-24

### Added

- `web` module containing a platform-specific extension trait to `SystemTime`, allowing conversion
  from and to `std::time::SystemTime`.

### Changed

- Improve performance of `SystemTime` by using `Duration` internally.

## [0.2.3] - 2023-10-23

### Changed

- Improve accuracy of `Instant::now()`.

## [0.2.2] - 2023-10-08

### Fixed

- Time conversion for `Instant`.

## [0.2.1] - 2023-10-07 [YANKED]

### Changed

- Bump MSRV to v1.60.

### Removed

- Unnecessary `once_cell` dependency.

## [0.2.0] - 2023-03-28

### Added

- Export [`TryFromFloatSecsError`] without breaking MSRV.

[`TryFromFloatSecsError`]: https://doc.rust-lang.org/std/time/struct.TryFromFloatSecsError.html

## [0.1.0] - 2023-03-27

### Added

- Initial release.

[Unreleased]: https://github.com/daxpedda/web-time/compare/v1.1.0...HEAD
[1.1.0]: https://github.com/daxpedda/web-time/compare/v1.0.0...v1.1.0
[1.0.0]: https://github.com/daxpedda/web-time/compare/v0.2.4...v1.0.0
[0.2.4]: https://github.com/daxpedda/web-time/compare/v0.2.3...v0.2.4
[0.2.3]: https://github.com/daxpedda/web-time/compare/v0.2.2...v0.2.3
[0.2.2]: https://github.com/daxpedda/web-time/compare/v0.2.1...v0.2.2
[0.2.1]: https://github.com/daxpedda/web-time/compare/v0.2.0...v0.2.1
[0.2.0]: https://github.com/daxpedda/web-time/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/daxpedda/web-time/releases/tag/v0.1.0
