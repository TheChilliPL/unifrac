//! This crate provides several fractional types for Rust:
//! - [`Primant`] — a type representing a fraction between 0 and 1 (inclusive).
//! - [`Phase`] — a type representing a fraction between 0 and 1 (exclusive).
//!
//! It does not depend on the standard library, so it can be used in `no_std` contexts.
//! Be aware that tests do require the standard library, at least for now.
#![no_std]
mod primant;
mod phase;

pub use primant::Primant;
pub use phase::Phase;