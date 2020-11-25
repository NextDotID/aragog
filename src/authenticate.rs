#[cfg(feature = "password_hashing")]
use argonautica::{self, Hasher, Verifier};

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

    /// Returns a Argon2 encrypted password hash that can be safely stored in database.
    ///
    /// # Arguments
    ///
    /// * `password` - The actual password to encrypt, should be checked for length and complexity
    /// * `secret_key` - The key that Argon2 will use to hash the password, this key should be unique
    /// and set as an environment variable or retrieved from some vault.
    ///
    /// # Returns
    ///
    /// On success the hashed password is returned, if the hashing fails for some reason
    /// (password or secret key invalid format) a [`ServiceError`]::[`UnprocessableEntity`] will be returned.
    ///
    /// # Features
    ///
    /// This function requires the `password_hashing` feature to be enabled.
    ///
    /// [`ServiceError`]: enum.ServiceError.html
    /// [`UnprocessableEntity`]: enum.ServiceError.html#variant.UnprocessableEntity
    #[cfg(feature = "password_hashing")]
    fn hash_password(password: &str, secret_key: &str) -> Result<String, ServiceError> {
        let mut hasher = Hasher::new();

        let res = hasher
            .with_password(password)
            .with_secret_key(secret_key)
            .hash();
        match res {
            Ok(value) => Ok(value),
            Err(_error) => Err(ServiceError::UnprocessableEntity),
        }
    }

    /// Verifies if the given password matches the Argon2 encrypted password hash.
    ///
    /// # Arguments
    ///
    /// * `password` - The actual password to verify
    /// * `password_hash` - The Argon2 encrypted hash password that should match the `password`
    /// * `secret_key` - The key that Argon2 will use to hash the password, this key should be unique
    /// and set as an environment variable or retrieved from some vault.
    ///
    /// # Returns
    ///
    /// On failure a [`ServiceError`]::[`Unauthorized`] will be returned.
    ///
    /// # Features
    ///
    /// This function requires the `password_hashing` feature to be enabled.
    ///
    /// [`ServiceError`]: enum.ServiceError.html
    /// [`Unauthorized`]: enum.ServiceError.html#variant.Unauthorized
    #[cfg(feature = "password_hashing")]
    fn verify_password(
        password: &str,
        password_hash: &str,
        secret_key: &str,
    ) -> Result<(), ServiceError> {
        let mut verifier = Verifier::new();

        match verifier
            .with_hash(password_hash)
            .with_password(password)
            .with_secret_key(secret_key)
            .verify()
        {
            Ok(value) => {
                if value {
                    Ok(())
                } else {
                    Err(ServiceError::Unauthorized)
                }
            }
            Err(_error) => Err(ServiceError::UnprocessableEntity),
        }
    }
}
