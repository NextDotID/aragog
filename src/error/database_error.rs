use crate::error::{ArangoError, ArangoHttpError};
use arangors::ArangoError as DriverError;
use std::fmt::{self, Display, Formatter};
use thiserror::Error;

/// Mapped Arango db error response
#[derive(Debug, Clone, Error)]
pub struct DatabaseError {
    /// The mapped Arango HTTP Error
    pub http_error: ArangoHttpError,
    /// The mapped Arango Error
    pub arango_error: ArangoError,
    /// The error message
    pub message: String,
}

impl Display for DatabaseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "code: {}\n\
                   error_num: {}\n\
                   message: {}",
            self.http_error, self.arango_error, self.message
        )
    }
}

impl From<DriverError> for DatabaseError {
    fn from(error: DriverError) -> Self {
        Self {
            http_error: ArangoHttpError::from_code(error.code()),
            arango_error: ArangoError::from_error_num(error.error_num()),
            message: error.message().to_string(),
        }
    }
}
