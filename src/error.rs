use thiserror::Error;
use arangors::{ClientError};
#[cfg(feature = "actix_http_error")]
use actix_web::{error, http::StatusCode};

/// Error enum used for the Arango ORM mapped as potential Http errors
///
/// # Features
///
/// If the cargo feature `actix_http_error` is enabled, `ServiceError` will implement the actix-web error system.
/// Allowing `ServiceError` to be used in actix-web http endpoints.
#[derive(Error, Debug)]
pub enum ServiceError {
    /// Unhandled error.
    /// Can be interpreted as a HTTP code `500` internal error.
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

#[cfg(feature = "actix_http_error")]
/// If the feature `actix_http_error` is enabled, `ServiceError` will implement `actix_web` `ResponseError` trait.
///
/// The implementation allows `ServiceError` to be used as an error response on `actix_web` http endpoints.
impl error::ResponseError for ServiceError {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::ValidationError(_str) => StatusCode::BAD_REQUEST,
            Self::Timeout => StatusCode::REQUEST_TIMEOUT,
            Self::NotFound(_str) => StatusCode::NOT_FOUND,
            Self::UnprocessableEntity => StatusCode::UNPROCESSABLE_ENTITY,
            Self::Unauthorized => StatusCode::UNAUTHORIZED,
            Self::Forbidden => StatusCode::FORBIDDEN,
            Self::Conflict=> StatusCode::CONFLICT,
            _ => StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}

impl ServiceError {
    /// Retrieves the matching HTTP code as a string.
    pub fn http_code(&self) -> String {
        match self {
            ServiceError::NotFound(_str) => "404".to_string(),
            ServiceError::ValidationError(_str) => "400".to_string(),
            ServiceError::UnprocessableEntity => "422".to_string(),
            ServiceError::Conflict => "409".to_string(),
            ServiceError::Unauthorized => "401".to_string(),
            ServiceError::Forbidden => "403".to_string(),
            ServiceError::Timeout => "408".to_string(),
            _ => "500".to_string()
        }
    }
}

impl From<ClientError> for ServiceError {
    fn from(error: ClientError) -> Self {
        match error {
            ClientError::Arango(arango_error) => {
                match arango_error.code() {
                    404 => Self::NotFound(arango_error.message().to_string()),
                    409 => Self::Conflict,
                    403 => Self::Forbidden,
                    401 => Self::Unauthorized,
                    408 => Self::Timeout,
                    _ => Self::UnprocessableEntity
                }
            },
            ClientError::Serde(_serde_error) => Self::UnprocessableEntity,
            ClientError::InsufficientPermission { permission: _permission, operation: _operation } => Self::Unauthorized,
            ClientError::InvalidServer(_server) => Self::Unauthorized,
            ClientError::HttpClient(_client) => Self::UnprocessableEntity,
        }
    }
}

impl Default for ServiceError {
    fn default() -> Self {
        ServiceError::InternalError
    }
}
