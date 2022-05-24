use crate::Error;

/// The `New` trait of the Aragog library.
/// This trait provides the possibility to initialize a Type from an other one. Its main use
/// it to transform a Http form into a [`Record`] model instance.
///
/// [`Record`]: crate::Record
pub trait New<T>: Sized {
    /// Instantiate and returns a new `Self` instance from `T`.
    ///
    /// # Errors
    ///
    /// Can fail and return an error, the error is in most of the cases an [`Error`]::[`ValidationError`]
    /// on fields validation failure
    ///
    /// [`Error`]: crate::Error
    /// [`ValidationError`]: crate::Error::ValidationError
    fn new(form: T) -> Result<Self, Error>;
}
