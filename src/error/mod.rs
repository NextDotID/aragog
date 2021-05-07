use std::error::Error;
use std::fmt::{self, Display, Formatter};

use arangors::ClientError;

use thiserror::private::AsDynError;
pub use {
    arango_error::ArangoError, arango_http_error::ArangoHttpError, database_error::DatabaseError,
};

mod arango_error;
mod arango_http_error;
mod database_error;

/// Error enum used for the Arango ORM mapped as potential Http errors
#[derive(Debug)]
pub enum ServiceError {
    /// Unhandled error.
    /// Can be interpreted as a HTTP code `500` internal error.
    InternalError {
        /// Optional message (will not be displayed)
        message: Option<String>,
    },
    /// Validations failed (see model validation as implemented in [`Validate`].
    /// Can be interpreted as a HTTP code `400` bad request.
    ///
    /// [`Validate`]: trait.Validate.html
    ValidationError(String),
    /// An Item (document or collection) could not be found.
    /// Can be interpreted as a HTTP code `404` not found.
    NotFound {
        /// The missing item
        item: String,
        /// The missing item identifier
        id: String,
        /// Optional database source error
        source: Option<DatabaseError>,
    },
    /// An operation failed due to format or data issue.
    ///
    /// Can be interpreted as a HTTP code `422` Unprocessable Entity.
    UnprocessableEntity {
        /// The source error
        source: Box<dyn Error>,
    },
    /// The ArangoDb Error as returned by the database host
    ///
    /// Can be interpreted as a HTTP code `500` Internal Error.
    ArangoError(DatabaseError),
    /// A database conflict occured
    ///
    /// Can be interpreted as a HTTP code `409` Conflict.
    Conflict(DatabaseError),
    /// Failed to load config or initialize the app.
    ///
    /// Can be interpreted as a HTTP code `500` Internal Error.
    InitError {
        /// Item that failed to init
        item: String,
        /// Error message
        message: String,
    },
    /// The operation is refused due to lack of authentication.
    /// Can be interpreted as a HTTP code `401` unauthorized.
    Unauthorized(Option<DatabaseError>),
    /// The operation is refused and authentication cannot resolve it.
    /// Can be interpreted as a HTTP code `403` forbidden.
    Forbidden(Option<DatabaseError>),
}

impl Display for ServiceError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ServiceError::InternalError { .. } => "Internal Error".to_string(),
                ServiceError::ValidationError(str) => format!("Validations failed: `{}`", str),
                ServiceError::NotFound { item, id, .. } => format!("{} {} not found", item, id),
                ServiceError::UnprocessableEntity { .. } => "Unprocessable Entity".to_string(),
                ServiceError::ArangoError(_) => "ArangoDB Error".to_string(),
                ServiceError::Conflict(_) => "Conflict".to_string(),
                ServiceError::InitError { item, message, .. } =>
                    format!("Failed to initialize `{}`: `{}`", item, message),
                ServiceError::Unauthorized(_) => "Unauthorized".to_string(),
                ServiceError::Forbidden(_) => "Forbidden".to_string(),
            }
        )
    }
}

impl Error for ServiceError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            ServiceError::InternalError { .. } => None,
            ServiceError::ValidationError(_) => None,
            ServiceError::NotFound { source, .. } => source.as_ref().map(AsDynError::as_dyn_error),
            ServiceError::UnprocessableEntity { source } => Some(source.as_ref()),
            ServiceError::ArangoError(e) => Some(e),
            ServiceError::Conflict(e) => Some(e),
            ServiceError::InitError { .. } => None,
            ServiceError::Unauthorized(source) => source.as_ref().map(AsDynError::as_dyn_error),
            ServiceError::Forbidden(source) => source.as_ref().map(AsDynError::as_dyn_error),
        }
    }
}

impl ServiceError {
    /// get the matching http code
    #[allow(dead_code)]
    pub fn http_code(&self) -> u16 {
        match self {
            Self::ValidationError(_str) => 400,
            Self::UnprocessableEntity { .. } => 422,
            Self::NotFound { .. } => 404,
            Self::Forbidden(_) => 403,
            Self::Unauthorized(_) => 401,
            Self::ArangoError(_db_error) => 500,
            Self::InitError { .. } => 500,
            Self::InternalError { .. } => 500,
            Self::Conflict(_) => 409,
        }
    }
}

impl From<ClientError> for ServiceError {
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
                message: error,
            },
        }
    }
}

impl Default for ServiceError {
    fn default() -> Self {
        Self::InternalError { message: None }
    }
}
