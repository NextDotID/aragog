use crate::AragogServiceError;

/// The `New` trait of the Aragog library.
/// This trait provides the possibility to initialize a Type from an other one. Its main use
/// it to transform a Http form into a [`Record`] model instance.
///
/// [`Record`]: trait.Record.html
pub trait New<T> :Sized {
    /// Instantiate and returns a new `Self` instance from `T`.
    /// Can fail and return an error, the error is in most of the cases a [`AragogServiceError`]::[`ValidationError`]
    /// on fields validation failure
    ///
    /// [`AragogServiceError`]: eum.AragogServiceError.html
    /// [`ValidationError`]: enum.AragogServiceError.html#variant.ValidationError
    fn new(form :T) -> Result<Self, AragogServiceError>;
}