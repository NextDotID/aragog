use crate::ServiceError;

/// The `Authenticate` trait of the Aragog library.
/// This trait provides the possibility to authenticate a Type with some secret. Its main use
/// it to authenticate a user or client [`Record`] model instance.
///
/// [`Record`]: trait.Record.html
pub trait Authenticate {
    /// Authenticates the instance with a secret.
    ///
    /// # Arguments
    ///
    /// * `secret` - the value supposed to validate authentication like a password
    ///
    /// # Returns
    ///
    /// On success `()` is returned, on failure it will return a [`ServiceError`] according to
    /// the Authenticate implementation
    ///
    /// [`ServiceError`]: enum.ServiceError.html
    fn authenticate(&self, secret: &str) -> Result<(), ServiceError>;
}