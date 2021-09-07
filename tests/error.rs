use aragog::error::{ArangoError, ArangoHttpError, DatabaseError};
use aragog::Error;
use std::error::Error as StdError;

#[test]
fn error_default() {
    assert_eq!(
        Error::default().http_code(),
        Error::InternalError { message: None }.http_code()
    );
}

#[test]
fn error_sources() {
    let db_error = DatabaseError {
        http_error: ArangoHttpError::BadParameter,
        arango_error: ArangoError::ArangoIllegalState,
        message: "".to_string(),
    };

    assert!(Error::ValidationError(String::new()).source().is_none());
    assert!(Error::NotFound {
        item: "".to_string(),
        id: "".to_string(),
        source: None
    }
    .source()
    .is_none());
    assert!(Error::NotFound {
        item: "".to_string(),
        id: "".to_string(),
        source: Some(db_error.clone())
    }
    .source()
    .is_some());
    assert!(Error::ArangoError(db_error.clone()).source().is_some());
    assert!(Error::Conflict(db_error.clone()).source().is_some());
    assert!(Error::Forbidden(Some(db_error.clone())).source().is_some());
    assert!(Error::Unauthorized(Some(db_error.clone()))
        .source()
        .is_some());
    assert!(Error::Forbidden(None).source().is_none());
    assert!(Error::Unauthorized(None).source().is_none());
    assert!(Error::UnprocessableEntity {
        source: Box::new(db_error)
    }
    .source()
    .is_some());
    assert!(Error::InternalError { message: None }.source().is_none());
    assert!(Error::InitError {
        item: "".to_string(),
        message: "".to_string()
    }
    .source()
    .is_none());
}
