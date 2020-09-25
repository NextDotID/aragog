use aragog::ServiceError;

#[test]
fn error_default() {
    assert_eq!(ServiceError::default().http_code(), ServiceError::InternalError.http_code());
}

#[cfg(feature = "actix")]
mod actix_http_errors {
    use aragog::ServiceError;
    use actix_web::ResponseError;

    #[test]
    fn actix_web_status_codes() {
        assert_eq!(ServiceError::InternalError.status_code().as_str(), &ServiceError::InternalError.http_code());
        assert_eq!(ServiceError::UnprocessableEntity.status_code().as_str(), &ServiceError::UnprocessableEntity.http_code());
        assert_eq!(ServiceError::Unauthorized.status_code().as_str(), &ServiceError::Unauthorized.http_code());
        assert_eq!(ServiceError::Forbidden.status_code().as_str(), &ServiceError::Forbidden.http_code());
        assert_eq!(ServiceError::NotFound(String::default()).status_code().as_str(), &ServiceError::NotFound(String::default()).http_code());
        assert_eq!(ServiceError::ValidationError(String::default()).status_code().as_str(), &ServiceError::ValidationError(String::default()).http_code());
        assert_eq!(ServiceError::Timeout.status_code().as_str(), &ServiceError::Timeout.http_code());
        assert_eq!(ServiceError::Conflict.status_code().as_str(), &ServiceError::Conflict.http_code());
    }
}