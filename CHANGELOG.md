# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to
[Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.4.0] - 2025-01-21

### Added

- `Progress<D>` wrapper for tracking progress with percentage calculations
- `Clone` implementation for `Counter<D>` and `Progress<D>` when `D: Clone`
- `Default` implementation for `Counter<D>` and `Progress<D>` when `D: Default`
- `Debug` implementation for `Progress<D>` when `D: Debug`
- `reset()` method for both `Counter` and `Progress` to reset byte counters
- `std` feature flag (enabled by default) for `std::io` trait implementations
- CONTRIBUTING.md and CHANGELOG.md
- rustfmt.toml configuration

### Changed

- Reworked CI pipeline
- Bumped MSRV to 1.85 and updated to Rust 2024 edition
- Updated README to accurately document available features

## [0.3.0] - 2025-01-01

- Initial release with `Counter<D>` wrapper for byte counting
- Support for `std::io`, `futures_io`, and `tokio::io` traits

[Unreleased]: https://github.com/spire-rs/countio/compare/v0.4.0...HEAD
[0.4.0]: https://github.com/spire-rs/countio/compare/v0.3.0...v0.4.0
[0.3.0]: https://github.com/spire-rs/countio/releases/tag/v0.3.0
