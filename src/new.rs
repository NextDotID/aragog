use crate::AragornServiceError;

/// The `New` trait of the Aragorn library.
/// This trait provides the possibility to initialize a Type from an other one. Its main use
/// it to transform a Http form into a [`Record`] model instance.
///
/// [`Record`]: ../record/trait.Record.html
pub trait New<T> :Sized {
    /// Instantiate and returns a new `Self` instance from `T`.
    /// Can fail and return an error, the error is in most of the cases a [`AragornServiceError`]::[`ValidationError`]
    /// on fields validation failure
    ///
    /// [`AragornServiceError`]: ../error/enum.AragornServiceError.html
    /// [`ValidationError`]: ../error/enum.AragornServiceError.html#variant.ValidationError
    fn new(form :T) -> Result<Self, AragornServiceError>;
}