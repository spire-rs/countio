# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to
[Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.3.0] - 2025-01-21

### Added

- `Progress<D>` wrapper for tracking progress with percentage calculations
- Separate `expected_reader_bytes` and `expected_writer_bytes` tracking in `Progress`
- `reader_percentage()` and `writer_percentage()` methods for `Progress`
- `with_expected_reader_bytes()`, `with_expected_writer_bytes()`, and `with_expected_bytes()` constructors
- `Clone` implementation for `Counter<D>` and `Progress<D>` when `D: Clone`
- `Default` implementation for `Counter<D>` and `Progress<D>` when `D: Default`
- `Debug` implementation for `Progress<D>` when `D: Debug`
- `reset()` method for both `Counter` and `Progress` to reset byte counters
- `std` feature flag (enabled by default) for `std::io` trait implementations

### Changed

- **Breaking:** Renamed `bytes_read()` to `reader_bytes()` for consistency
- **Breaking:** Renamed `bytes_written()` to `writer_bytes()` for consistency
- **Breaking:** Renamed `bytes_processed()` to `total_bytes()`
- **Breaking:** Changed `with_bytes()` parameter order to `(inner, reader_bytes, writer_bytes)`
- Reworked CI pipeline
- Bumped MSRV to 1.85 and updated to Rust 2024 edition
- Updated README to accurately document available features

### Removed

- **Breaking:** Removed `counter()` and `counter_mut()` from `Progress` (use delegated methods instead)

## [0.2.0] - 2024-01-01

- Initial release with `Counter<D>` wrapper for byte counting
- Support for `std::io`, `futures_io`, and `tokio::io` traits

[Unreleased]: https://github.com/spire-rs/countio/compare/v0.3.0...HEAD
[0.3.0]: https://github.com/spire-rs/countio/compare/v0.2.0...v0.3.0
[0.2.0]: https://github.com/spire-rs/countio/releases/tag/v0.2.0
