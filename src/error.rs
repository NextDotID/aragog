use thiserror::Error;

/// Error enum used for the Arango ORM mapped as potential Http errors
#[derive(Error, Debug)]
pub enum AragornServiceError {
    /// code 500 internal error
    #[error("Internal error")]
    InternalError,
    /// code 400 bad request
    #[error("Validations failed: `{0}`")]
    ValidationError(String),
    /// code 408 Request timeout
    #[error("Timeout")]
    Timeout,
    /// Code 404 not found
    #[error("`{0}` not found")]
    NotFound(String),
    /// code 422 unprocessable entity
    #[error("Unprocessable entity")]
    UnprocessableEntity,
    /// code 401 unauthorized
    #[error("Unauthorized")]
    Unauthorized,
    /// code 403 forbidden
    #[error("Forbidden")]
    Forbidden,
    /// code 409 conflict
    #[error("Conflict")]
    Conflict,
}

impl AragornServiceError {
    pub fn http_code(&self) -> String {
        match self {
            AragornServiceError::NotFound(_str) => "404".to_string(),
            AragornServiceError::ValidationError(_str) => "400".to_string(),
            AragornServiceError::UnprocessableEntity => "422".to_string(),
            AragornServiceError::Conflict => "409".to_string(),
            AragornServiceError::Unauthorized => "401".to_string(),
            AragornServiceError::Forbidden => "403".to_string(),
            AragornServiceError::Timeout => "408".to_string(),
            AragornServiceError::InternalError => "500".to_string()
        }
    }
}

impl Default for AragornServiceError {
    fn default() -> Self {
        AragornServiceError::InternalError
    }
}