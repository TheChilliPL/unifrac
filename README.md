![Maintenance](https://img.shields.io/badge/maintenance-experimental-blue.svg?style=for-the-badge)
[![Crates.io version](https://img.shields.io/crates/v/unifrac?style=for-the-badge)][crates]
[![Docs.rs](https://img.shields.io/docsrs/unifrac?style=for-the-badge)][docs]
![Size](https://img.shields.io/crates/size/unifrac?style=for-the-badge)
[![License](https://img.shields.io/github/license/TheChilliPL/unifrac?style=for-the-badge)][license]

# unifrac

A crate for working with normalized fractional values.

It provides several fractional types for Rust:
- `Primant` — a type representing a fraction between 0 and 1 (inclusive).
- `Phase` — a type representing a fraction between 0 and 1 (exclusive).

It does not depend on the standard library, so it can be used in `no_std` contexts.

For more information, see the [crate documentation][docs].

[crates]: https://crates.io/crates/unifrac
[docs]: https://docs.rs/unifrac
[license]: https://github.com/TheChilliPL/unifrac/blob/main/LICENSE