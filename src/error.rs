use thiserror::Error;

/// Error enum used for the Arango ORM mapped as potential Http errors
#[derive(Error, Debug)]
pub enum AragogServiceError {
    /// Unhandled error
    /// Can be interpreted as a HTTP code `500` internal error
    #[error("Internal error")]
    InternalError,
    /// Validations failed (see model validation as implemented in [`Validate`].
    /// Can be interpreted as a HTTP code `400` bad request.
    ///
    /// [`Validate`]: trait.Validate.html
    #[error("Validations failed: `{0}`")]
    ValidationError(String),
    /// A query/request timed out.
    /// Can be interpreted as a HTTP code `408` Request timeout.
    #[error("Timeout")]
    Timeout,
    /// A record could not be found (see record query as implemented in [`Record`]).
    /// Can be interpreted as a HTTP code `404` not found.
    ///
    /// [`Record`]: trait.Record.html
    #[error("`{0}` not found")]
    NotFound(String),
    /// An operation on a document failed due to format or data issue.
    /// Can be interpreted as a HTTP code `422` unprocessable entity.
    #[error("Unprocessable entity")]
    UnprocessableEntity,
    /// The operation is refused due to lack of authentication.
    /// Can be interpreted as a HTTP code `401` unauthorized.
    #[error("Unauthorized")]
    Unauthorized,
    /// The operation is refused and authentication cannot resolve it.
    /// Can be interpreted as a HTTP code `403` forbidden.
    #[error("Forbidden")]
    Forbidden,
    /// The operation fails due to a conflict, for example a unique index was not respected.
    /// Can be interpreted as a HTTP code `409` conflict.
    #[error("Conflict")]
    Conflict,
}

impl AragogServiceError {
    /// Retrieves the matching HTTP code as a string.
    pub fn http_code(&self) -> String {
        match self {
            AragogServiceError::NotFound(_str) => "404".to_string(),
            AragogServiceError::ValidationError(_str) => "400".to_string(),
            AragogServiceError::UnprocessableEntity => "422".to_string(),
            AragogServiceError::Conflict => "409".to_string(),
            AragogServiceError::Unauthorized => "401".to_string(),
            AragogServiceError::Forbidden => "403".to_string(),
            AragogServiceError::Timeout => "408".to_string(),
            AragogServiceError::InternalError => "500".to_string()
        }
    }
}

impl Default for AragogServiceError {
    fn default() -> Self {
        AragogServiceError::InternalError
    }
}