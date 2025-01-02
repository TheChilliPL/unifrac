use core::fmt::{Debug, Display};
use num_traits::float::FloatCore;

/// A fraction between 0 and 1 (inclusive).
///
/// This type is useful for representing a fraction of a whole, such as a percentage.
///
/// # Naming
///
/// _Primantissa_, or _primant_ for short, is a neologism derived from the term
/// _mantissa_, which is the fractional part of a logarithm. _Mantissa_, however,
/// is already used in the context of floating-point numbers, so _primant_ was
/// chosen to avoid confusion.
///
/// [Source: “A word for a value between 0 and 1 (inclusive)”, English Language & Usage Stack Exchange](https://english.stackexchange.com/a/286524).
///
/// _Proportion_ was not chosen because this type is not implemented as a ratio
/// of two integers.
///
/// # Representation
///
/// A [`Primant`] is represented as a 32-bit unsigned integer.
/// The value `0` represents `0.0`, and the maximum value represents `1.0`.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Primant(u32);

/// Raw conversion functions.
///
/// These functions should never panic, as every [`Primant`] is a valid value.
impl Primant {
    pub const MIN: Primant = Primant(0);
    pub const ZERO: Primant = Primant(0);
    pub const MAX: Primant = Primant(u32::MAX);

    /// Creates a new [`Primant`] from a raw representation.
    pub fn from_raw(value: u32) -> Self {
        Primant(value)
    }

    /// Returns the raw representation of the [`Primant`].
    pub fn to_raw(self) -> u32 {
        self.0
    }
}

impl TryFrom<f32> for Primant {
    type Error = ();

    fn try_from(value: f32) -> Result<Self, Self::Error> {
        if !(0.0..=1.0).contains(&value) {
            Err(())
        } else {
            Ok(Primant((value * u32::MAX as f32) as u32))
        }
    }
}

impl TryFrom<f64> for Primant {
    type Error = ();

    fn try_from(value: f64) -> Result<Self, Self::Error> {
        if !(0.0..=1.0).contains(&value) {
            Err(())
        } else {
            Ok(Primant((value * u32::MAX as f64) as u32))
        }
    }
}

impl From<Primant> for f32 {
    fn from(value: Primant) -> Self {
        value.0 as f32 / u32::MAX as f32
    }
}

impl From<Primant> for f64 {
    fn from(value: Primant) -> Self {
        value.0 as f64 / u32::MAX as f64
    }
}

/// Generic conversion functions to and from floating-point numbers.
impl Primant {
    /// Creates a new [`Primant`] from a floating-point value.
    ///
    /// # Panics
    ///
    /// Panics if the value is not in the range `0.0..=1.0`.
    pub fn from_float<T: FloatCore>(value: T) -> Self {
        assert!(value >= T::zero() && value <= T::one(), "value must be in the range 0.0..=1.0");
        Primant((value * T::from(u32::MAX).unwrap()).to_u32().unwrap())
    }

    /// Creates a new [`Primant`] from a floating-point value.
    ///
    /// Returns `None` if the value is not in the range `0.0..=1.0`.
    pub fn try_from_float<T: FloatCore>(value: T) -> Option<Self> {
        if value < T::zero() || value > T::one() { return None; }
        let value = (value * T::from(u32::MAX)?).to_u32()?;
        Some(Primant(value))
    }

    /// Creates a new [`Primant`] from a floating-point value.
    ///
    /// If the value is not in the range `0.0..=1.0`, it saturates to the closest
    /// representable value.
    pub fn from_float_saturating<T: FloatCore>(value: T) -> Self {
        Primant((value.clamp(T::zero(), T::one()) * T::from(u32::MAX).unwrap()).to_u32().unwrap())
    }

    /// Returns the value as a floating-point number.
    pub fn into_float<T: FloatCore>(self) -> T {
        T::from(self.0).unwrap() / T::from(u32::MAX).unwrap()
    }
}

/// Conversion functions to and from integer ratios.
impl Primant {
    /// Creates a new [`Primant`] from a numerator and a denominator.
    ///
    /// # Panics
    ///
    /// Panics if the denominator is zero or if the result would not fit in a `Primant`.
    pub fn from_ratio(numerator: u32, denominator: u32) -> Self {
        assert_ne!(denominator, 0, "denominator must not be zero");
        assert!(numerator <= denominator, "numerator must not be greater than the denominator");
        unsafe { Self::from_ratio_unchecked(numerator, denominator) }
    }

    /// Creates a new [`Primant`] from a numerator and a denominator.
    ///
    /// Returns `None` if the denominator is zero or if the result would not fit in a `Primant`.
    pub fn try_from_ratio(numerator: u32, denominator: u32) -> Option<Self> {
        if denominator == 0 || numerator > denominator { return None; }
        Some(unsafe { Self::from_ratio_unchecked(numerator, denominator) })
    }

    /// Creates a new [`Primant`] from a numerator and a denominator.
    ///
    /// If the result would not fit in a `Primant`, it saturates to the closest representable
    /// value.
    ///
    /// # Panics
    ///
    /// Panics if the denominator is zero.
    pub fn from_ratio_saturating(numerator: u32, denominator: u32) -> Self {
        assert_ne!(denominator, 0, "denominator must not be zero");

        Primant(numerator.saturating_mul(u32::MAX / denominator))
    }

    /// Creates a new [`Primant`] from a numerator and a denominator.
    ///
    /// # Safety
    ///
    /// This function doesn't perform any checks.
    /// If called with invalid arguments, it produces undefined behavior.
    /// Prefer using [`Primant::from_ratio`] or [`Primant::try_from_ratio`] instead.
    pub unsafe fn from_ratio_unchecked(numerator: u32, denominator: u32) -> Self {
        Primant(numerator.unchecked_mul(u32::MAX / denominator))
    }
}

/// Conversion functions to and from percentages.
impl Primant {
    /// Returns the value as a percentage.
    pub fn to_percentage<T: FloatCore>(self) -> T {
        self.into_float::<T>() * T::from(100).unwrap()
    }

    /// Creates a new [`Primant`] from a percentage.
    ///
    /// # Panics
    ///
    /// Panics if the percentage is not in the range `0.0..=100.0`.
    pub fn from_percentage<T: FloatCore>(percentage: T) -> Self {
        Self::from_float(percentage / T::from(100).unwrap())
    }

    /// Creates a new [`Primant`] from a percentage.
    ///
    /// Returns `None` if the percentage is not in the range `0.0..=100.0`.
    pub fn try_from_percentage<T: FloatCore>(percentage: T) -> Option<Self> {
        Self::try_from_float(percentage / T::from(100)?)
    }

    /// Creates a new [`Primant`] from a percentage.
    ///
    /// If the percentage is not in the range `0.0..=100.0`, it saturates to
    /// the closest representable value.
    pub fn from_percentage_saturating<T: FloatCore>(percentage: T) -> Self {
        let value = percentage / T::from(100).unwrap();
        Self::from_float(value.clamp(T::zero(), T::one()))
    }
}

impl Debug for Primant {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Primant({})", f64::from(*self))
    }
}

impl Display for Primant {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:.2}%", f64::from(*self) * 100.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    extern crate std;
    extern crate alloc;
    use alloc::format;
    use core::str::FromStr;
    use std::println;

    const EPSILON: f64 = 1e-6;

    fn assert_approx_eq<T: Into<f64>>(a: T, b: T) {
        assert!((a.into() - b.into()).abs() < EPSILON);
    }

    #[test]
    fn test_f64_conversion() {
        let fraction = Primant::try_from(0.5f64).unwrap();
        let f: f64 = fraction.into();
        assert_approx_eq(f, 0.5);
    }

    #[test]
    fn test_f32_conversion() {
        let fraction = Primant::try_from(0.5f32).unwrap();
        let f: f32 = fraction.into();
        assert_approx_eq(f, 0.5);
    }

    #[test]
    fn test_into_percent() {
        let fraction = Primant::try_from(0.5f64).unwrap();
        let percent = fraction.to_percentage();
        assert_approx_eq(percent, 50.0);
    }

    #[test]
    fn test_from_percent() {
        let fraction = Primant::try_from_percentage(50.0).unwrap();
        assert_approx_eq(fraction.into_float(), 0.5);
    }

    #[test]
    fn test_from_ratio() {
        let fraction = Primant::try_from_ratio(1u32, 2u32).unwrap();
        assert_approx_eq(fraction.into_float(), 0.5);
    }

    #[test]
    fn test_debug() {
        let fraction = Primant::try_from(0.5f64).unwrap();
        let output = format!("{:?}", fraction);
        println!("{}", output);
        assert_eq!(&output[0..8], "Primant(");
        assert_eq!(&output[output.len() - 1..], ")");
        let value_str = &output[8..output.len() - 1];
        assert_approx_eq(f64::from_str(value_str).unwrap(), 0.5);
    }

    #[test]
    fn test_display() {
        let fraction = Primant::try_from(0.5f64).unwrap();
        let output = format!("{}", fraction);
        println!("{}", output);
        assert_eq!(format!("{}", fraction), "50.00%");
    }
}
