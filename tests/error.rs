use aragog::ServiceError;

#[test]
fn error_default() {
    assert_eq!(
        ServiceError::default().http_code(),
        ServiceError::InternalError { message: None }.http_code()
    );
}

#[cfg(feature = "actix")]
mod actix_http_errors {
    use actix_web::ResponseError;

    use aragog::ServiceError;

    #[test]
    fn actix_web_status_codes() {
        assert_eq!(
            ServiceError::InternalError { message: None }
                .status_code()
                .as_str(),
            ServiceError::InternalError { message: None }.http_code()
        );
        assert_eq!(
            ServiceError::UnprocessableEntity {
                source: Box::new(ServiceError::default())
            }
            .status_code()
            .as_str(),
            ServiceError::UnprocessableEntity {
                source: Box::new(ServiceError::default())
            }
            .http_code()
        );
        assert_eq!(
            ServiceError::Unauthorized.status_code().as_str(),
            ServiceError::Unauthorized.http_code()
        );
        assert_eq!(
            ServiceError::Forbidden.status_code().as_str(),
            ServiceError::Forbidden.http_code()
        );
        let err = ServiceError::NotFound {
            id: "".to_string(),
            item: "".to_string(),
            source: None,
        };
        assert_eq!(err.status_code().as_str(), err.http_code());
        assert_eq!(
            ServiceError::ValidationError(String::default())
                .status_code()
                .as_str(),
            ServiceError::ValidationError(String::default()).http_code()
        );
    }
}
