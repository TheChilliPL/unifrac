use core::fmt::{Debug, Display};
use num_traits::float::FloatCore;

/// A fraction between 0 and 1 (exclusive).
///
/// This type is useful for representing a cyclic value like an angle or a phase.
///
/// # Naming
/// 
/// _Phase_ was chosen because it is a common term for cyclic values.
/// 
/// # Representation
/// A `Phase` is represented as a 32-bit unsigned integer.
/// The value `0` represents `0.0`, and the maximum value represents `0.9…`.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Phase(u32);

/// Raw conversion functions.
///
/// These functions should never panic, as every [`Phase`] is a valid value.
impl Phase {
    pub const MIN: Phase = Phase(0);
    pub const MAX: Phase = Phase(u32::MAX);

    /// Creates a new [`Phase`] from a raw representation.
    pub fn from_raw(value: u32) -> Self {
        Phase(value)
    }

    /// Returns the raw representation of the [`Phase`].
    pub fn to_raw(self) -> u32 {
        self.0
    }
}

impl TryFrom<f32> for Phase {
    type Error = ();

    fn try_from(value: f32) -> Result<Self, Self::Error> {
        if !(0.0..1.0).contains(&value) {
            Err(())
        } else {
            Ok(Phase((value * u32::MAX as f32) as u32))
        }
    }
}

impl TryFrom<f64> for Phase {
    type Error = ();

    fn try_from(value: f64) -> Result<Self, Self::Error> {
        if !(0.0..1.0).contains(&value) {
            Err(())
        } else {
            Ok(Phase((value * u32::MAX as f64) as u32))
        }
    }
}

impl From<Phase> for f32 {
    fn from(value: Phase) -> Self {
        value.0 as f32 / u32::MAX as f32
    }
}

impl From<Phase> for f64 {
    fn from(value: Phase) -> Self {
        value.0 as f64 / u32::MAX as f64
    }
}

/// Generic conversion functions to and from floating-point numbers.
impl Phase {
    /// Creates a new [`Phase`] from a floating-point value.
    /// 
    /// # Panics
    /// 
    /// Panics if the value is not in the range `0.0..1.0`.
    pub fn from_float<T: FloatCore>(value: T) -> Self {
        assert!(value >= T::zero() && value < T::one(), "value must be in the range 0.0..1.0");
        Phase((value * T::from(u32::MAX).unwrap()).to_u32().unwrap())
    }
    
    /// Creates a new [`Phase`] from a floating-point value.
    /// 
    /// Returns `None` if the value is not in the range `0.0..1.0`.
    pub fn try_from_float<T: FloatCore>(value: T) -> Option<Self> {
        if value < T::zero() || value >= T::one() { return None; }
        let value = (value * T::from(u32::MAX)?).to_u32()?;
        Some(Phase(value))
    }
    
    /// Creates a new [`Phase`] from a floating-point value.
    /// 
    /// If the value is not in the range `0.0..1.0`, it saturates to the closest
    /// representable value.
    pub fn from_float_saturating<T: FloatCore>(value: T) -> Self {
        Phase((value.clamp(T::zero(), T::one()) * T::from(u32::MAX).unwrap()).to_u32().unwrap())
    }
}

impl Debug for Phase {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Phase({})", f32::from(*self))
    }
}

impl Display for Phase {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:.4}", f32::from(*self))
    }
}