//! Traits for conversions between types.

/// A conversion error indicating that the value is outside of the range of the target type.
#[derive(Debug, thiserror::Error)]
pub enum RangeError {
    /// Exceeded the lower bound.
    #[error("Exceeded lower bound")]
    LowerBound,
    /// Exceeded the upper bound.
    #[error("Exceeded upper bound")]
    UpperBound,
}

/// A saturating version of [`From`] for conversions that fail with [`RangeError`].
pub trait SaturatingFrom<T> {
    /// Performs the conversion.
    fn saturating_from(value: T) -> Self;
}

/// A saturating version of [`Into`] for conversions that fail with [`RangeError`].
///
/// Library authors usually should not implement this trait, but instead prefer to implement
/// [`SaturatingFrom`].
pub trait SaturatingInto<T> {
    /// Performs the conversion.
    fn saturating_into(self) -> T;
}

impl<T, U> SaturatingInto<T> for U
where
    T: SaturatingFrom<U>,
{
    fn saturating_into(self) -> T {
        T::saturating_from(self)
    }
}
