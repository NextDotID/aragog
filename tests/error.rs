use aragog::error::{ArangoError, ArangoHttpError, DatabaseError};
use aragog::ServiceError;
use std::error::Error;

#[test]
fn error_default() {
    assert_eq!(
        ServiceError::default().http_code(),
        ServiceError::InternalError { message: None }.http_code()
    );
}

#[test]
fn error_sources() {
    let db_error = DatabaseError {
        http_error: ArangoHttpError::BadParameter,
        arango_error: ArangoError::ArangoIllegalState,
        message: "".to_string(),
    };

    assert!(ServiceError::ValidationError(String::new())
        .source()
        .is_none());
    assert!(ServiceError::NotFound {
        item: "".to_string(),
        id: "".to_string(),
        source: None
    }
    .source()
    .is_none());
    assert!(ServiceError::NotFound {
        item: "".to_string(),
        id: "".to_string(),
        source: Some(db_error.clone())
    }
    .source()
    .is_some());
    assert!(ServiceError::ArangoError(db_error.clone())
        .source()
        .is_some());
    assert!(ServiceError::Conflict(db_error.clone()).source().is_some());
    assert!(ServiceError::Forbidden(Some(db_error.clone()))
        .source()
        .is_some());
    assert!(ServiceError::Unauthorized(Some(db_error.clone()))
        .source()
        .is_some());
    assert!(ServiceError::Forbidden(None).source().is_none());
    assert!(ServiceError::Unauthorized(None).source().is_none());
    assert!(ServiceError::UnprocessableEntity {
        source: Box::new(db_error.clone())
    }
    .source()
    .is_some());
    assert!(ServiceError::InternalError { message: None }
        .source()
        .is_none());
    assert!(ServiceError::InitError {
        item: "".to_string(),
        message: "".to_string()
    }
    .source()
    .is_none());
}
