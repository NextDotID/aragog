use thiserror::Error;

/// Error enum used for the Arango ORM mapped as potential Http errors
#[derive(Error, Debug)]
pub enum AragogServiceError {
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

impl AragogServiceError {
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