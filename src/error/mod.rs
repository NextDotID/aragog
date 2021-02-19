use std::error::Error;

#[cfg(feature = "actix")]
use actix_web::{error, http::StatusCode};
use arangors::ClientError;
#[cfg(feature = "open-api")]
use paperclip::actix::api_v2_errors;
use thiserror::Error as ErrorDerive;

pub use {
    arango_error::ArangoError, arango_http_error::ArangoHttpError, database_error::DatabaseError,
};

mod arango_error;
mod arango_http_error;
mod database_error;

/// Error enum used for the Arango ORM mapped as potential Http errors
///
/// # Features
///
/// If the cargo feature `actix` is enabled, `ServiceError` will implement the actix-web error system.
/// Allowing `ServiceError` to be used in actix-web http endpoints.
#[cfg_attr(feature = "open-api", api_v2_errors())]
#[derive(ErrorDerive, Debug)]
pub enum ServiceError {
    /// Unhandled error.
    /// Can be interpreted as a HTTP code `500` internal error.
    #[error("Internal error")]
    InternalError {
        /// Optional message (will not be displayed)
        message: Option<String>,
    },
    /// Validations failed (see model validation as implemented in [`Validate`].
    /// Can be interpreted as a HTTP code `400` bad request.
    ///
    /// [`Validate`]: trait.Validate.html
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
        source: Box<dyn Error>,
    },
    /// The ArangoDb Error as returned by the database host
    ///
    /// Can be interpreted as a HTTP code `500` Internal Error.
    #[error("Internal Error")]
    ArangoError(#[source] DatabaseError),
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
    Unauthorized,
    /// The operation is refused and authentication cannot resolve it.
    /// Can be interpreted as a HTTP code `403` forbidden.
    #[error("Forbidden")]
    Forbidden,
}

#[cfg(feature = "actix")]
/// If the feature `actix` is enabled, `ServiceError` will implement `actix_web` `ResponseError` trait.
///
/// The implementation allows `ServiceError` to be used as an error response on `actix_web` http endpoints.
impl error::ResponseError for ServiceError {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::ValidationError(_str) => StatusCode::BAD_REQUEST,
            Self::UnprocessableEntity { .. } => StatusCode::UNPROCESSABLE_ENTITY,
            Self::NotFound { .. } => StatusCode::NOT_FOUND,
            Self::ArangoError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl ServiceError {
    /// get the matching http code
    #[allow(dead_code)]
    pub fn http_code(&self) -> &str {
        match self {
            Self::ValidationError(_str) => "400",
            Self::UnprocessableEntity { .. } => "422",
            Self::NotFound { .. } => "404",
            Self::ArangoError(_) => "500",
            _ => "500",
        }
    }
}

impl From<ClientError> for ServiceError {
    fn from(error: ClientError) -> Self {
        log::debug!("Client Error: {}", error);
        match error {
            ClientError::Arango(arango_error) => {
                Self::ArangoError(DatabaseError::from(arango_error))
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
