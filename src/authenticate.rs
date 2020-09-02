use crate::AragornServiceError;

/// The `Authenticate` trait of the Aragorn library.
/// This trait provides the possibility to authenticate a Type with some secret. Its main use
/// it to authenticate a user or client [`Record`] model instance.
///
/// [`Record`]: ../record/trait.Record.html
pub trait Authenticate {
    /// Authenticates the instance with a secret.
    ///
    /// # Arguments
    ///
    /// * `secret` - the value supposed to validate authentication like a password
    ///
    /// # Returns
    ///
    /// On success `()` is returned, on failure it will return a [`AragornServiceError`] according to
    /// the Authenticate implementation
    ///
    /// [`AragornServiceError`]: ../../../error/enum.AragornServiceError.html
    fn authenticate(&self, secret: &str) -> Result<(), AragornServiceError>;
}