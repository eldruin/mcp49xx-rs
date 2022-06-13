# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](http://semver.org/spec/v2.0.0.html).

## [Unreleased]

...

## [0.3.0] - 2022-06-13

### Changed
- [breaking-change] A reference to the SPI device is now passed in every call, which eases bus sharing.

## [0.2.0] - 2021-09-16

### Added
- `PartialEq` trait implementation for the `Error` type.

### Changed
- [breaking-change] SPI mode constants have been replaced with reexports of the constants from `embedded_hal`.
  `MODE0` (0,0) corresponds now to `MODE_0` and `MODE1` (1,1) to `MODE_3`.
- [breaking-change] Use fallible output pins. `Error` type now contains `Pin` variant.
- [breaking-change] Removed `Default` trait implementation for `Mcp49xx`.
- `interface` module is now public to ease usage.

## 0.1.0 - 2019-02-17

This is the initial release to crates.io of the feature-complete driver. There
may be some API changes in the future, in case I decide that something can be
further improved. All changes will be documented in this CHANGELOG.

[Unreleased]: https://github.com/eldruin/mcp49xx-rs/compare/v0.3.0...HEAD
[0.3.0]: https://github.com/eldruin/mcp49xx-rs/compare/v0.2.0...v0.3.0
[0.2.0]: https://github.com/eldruin/mcp49xx-rs/compare/v0.1.0...v0.2.0
