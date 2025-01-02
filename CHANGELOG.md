# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

Dates in this file are in `YYYY-MM-DD (HH:MM)` format in [Europe/Warsaw] timezone[^1].

[Europe/Warsaw]: https://en.wikipedia.org/wiki/Europe/Warsaw
[^1]: Standard Time: CET/UTC+1, Daylight Saving Time: CEST/UTC+2

## [0.1.1] - 2025-01-02 09:02

### Changed

- Ratio conversion methods are now generic.

### Removed

- `Primant::from_ratio_unchecked` method [BREAKING].
  - Removed due to the generic `PrimInt` trait from `num-traits` not supporting
    unchecked operations (rust-num/num-traits#47).

## [0.1.0] - 2025-01-02 08:04

### Added

- First release.