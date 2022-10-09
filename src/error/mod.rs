use arangors_lite::ClientError;
use thiserror::Error;
pub use {
    arango_error::ArangoError, arango_http_error::ArangoHttpError, database_error::DatabaseError,
};

mod arango_error;
mod arango_http_error;
mod database_error;

/// Error enum used for the Arango ORM mapped as potential Http errors
#[derive(Debug, Error)]
pub enum Error {
    /// Unhandled error.
    /// Can be interpreted as a HTTP code `500` internal error.
    #[error("Internal Error")]
    InternalError {
        /// Optional message (will not be displayed)
        message: Option<String>,
    },
    /// Validations failed (see model validation as implemented in [`Validate`].
    /// Can be interpreted as a HTTP code `400` bad request.
    ///
    /// [`Validate`]: crate::Validate
    #[error("Validations failed: `{0}`")]
    ValidationError(String),
    /// An Item (document or collection) could not be found.
    /// Can be interpreted as a HTTP code `404` not found.
    #[error("{item} {id} not found")]
    NotFound {
        /// The missing item
        item: String,
        /// The missing item identifier
        id: String,
        /// Optional database source error
        #[source]
        source: Option<DatabaseError>,
    },
    /// An operation failed due to format or data issue.
    ///
    /// Can be interpreted as a HTTP code `422` Unprocessable Entity.
    #[error("Unprocessable Entity")]
    UnprocessableEntity {
        /// The source error
        #[source]
        source: Box<dyn std::error::Error + Send + Sync>,
    },
    /// The ArangoDb Error as returned by the database host
    ///
    /// Can be interpreted as a HTTP code `500` Internal Error.
    #[error("ArangoDB Error")]
    ArangoError(#[source] DatabaseError),
    /// A database conflict occured
    ///
    /// Can be interpreted as a HTTP code `409` Conflict.
    #[error("Conflict")]
    Conflict(#[source] DatabaseError),
    /// Failed to load config or initialize the app.
    ///
    /// Can be interpreted as a HTTP code `500` Internal Error.
    #[error("Failed to initialize `{item}`: `{message}`")]
    InitError {
        /// Item that failed to init
        item: String,
        /// Error message
        message: String,
    },
    /// The operation is refused due to lack of authentication.
    /// Can be interpreted as a HTTP code `401` unauthorized.
    #[error("Unauthorized")]
    Unauthorized(#[source] Option<DatabaseError>),
    /// The operation is refused and authentication cannot resolve it.
    /// Can be interpreted as a HTTP code `403` forbidden.
    #[error("Forbidden")]
    Forbidden(#[source] Option<DatabaseError>),
}

impl Error {
    /// get the matching http code
    #[allow(dead_code)]
    #[must_use]
    #[inline]
    pub const fn http_code(&self) -> u16 {
        match self {
            Self::ValidationError(_str) => 400,
            Self::UnprocessableEntity { .. } => 422,
            Self::NotFound { .. } => 404,
            Self::Forbidden(_) => 403,
            Self::Unauthorized(_) => 401,
            Self::ArangoError(_) | Self::InitError { .. } | Self::InternalError { .. } => 500,
            Self::Conflict(_) => 409,
        }
    }
}

impl From<ClientError> for Error {
    fn from(error: ClientError) -> Self {
        log::debug!("Client Error: {}", error);
        match error {
            ClientError::Arango(arango_error) => {
                let arango_error = DatabaseError::from(arango_error);
                match arango_error.http_error {
                    ArangoHttpError::Unauthorized => Self::Unauthorized(Some(arango_error)),
                    ArangoHttpError::Forbidden => Self::Forbidden(Some(arango_error)),
                    ArangoHttpError::Conflict => Self::Conflict(arango_error),
                    _ => Self::ArangoError(arango_error),
                }
            }
            ClientError::Serde(serde_error) => Self::UnprocessableEntity {
                source: Box::new(serde_error),
            },
            ClientError::InvalidServer(server) => Self::InitError {
                item: server,
                message: String::from("Is not ArangoDB"),
            },
            ClientError::InsufficientPermission {
                permission,
                operation,
            } => Self::InitError {
                item: operation.clone(),
                message: format!(
                    "Insufficent permission for {} : {:?}",
                    operation, permission
                ),
            },
            ClientError::HttpClient(error) => Self::InitError {
                item: "Http Client".to_string(),
                message: error.to_string(),
            },
        }
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Self::UnprocessableEntity {
            source: Box::new(err),
        }
    }
}

impl Default for Error {
    fn default() -> Self {
        Self::InternalError { message: None }
    }
}
