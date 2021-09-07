extern crate aragog;

use serde::{Deserialize, Serialize};

use aragog::{
    AuthMode, DatabaseAccess, DatabaseConnection, DatabaseRecord, Error, OperationOptions, Record,
};
use common::*;

pub mod common;

#[derive(Serialize, Deserialize, Record, Clone)]
#[before_create(func = "before_create")]
struct Dish {
    pub name: String,
    pub price: u16,
}

impl Dish {
    fn before_create(&self) -> Result<(), Error> {
        Err(Error::InternalError {
            message: String::from("Hook forbids creation").into(),
        })
    }
}

#[maybe_async::test(
    feature = "blocking",
    async(all(not(feature = "blocking")), tokio::test)
)]
async fn works_with_correct_parameters() {
    DatabaseConnection::builder()
        .with_credentials(
            &std::env::var("DB_HOST").unwrap_or_else(|_| DEFAULT_DB_HOST.to_string()),
            &std::env::var("DB_NAME").unwrap_or_else(|_| DEFAULT_DB_NAME.to_string()),
            &std::env::var("DB_USER").unwrap_or_else(|_| DEFAULT_DB_USER.to_string()),
            &std::env::var("DB_PWD").unwrap_or_else(|_| DEFAULT_DB_PWD.to_string()),
        )
        .with_schema_path("./tests/schema.yaml")
        .apply_schema()
        .build()
        .await
        .unwrap();
}

#[maybe_async::test(
    feature = "blocking",
    async(all(not(feature = "blocking")), tokio::test)
)]
async fn fails_with_wrong_parameters() {
    match DatabaseConnection::builder()
        .with_credentials(
            &std::env::var("DB_HOST").unwrap_or_else(|_| DEFAULT_DB_HOST.to_string()),
            &std::env::var("DB_NAME").unwrap_or_else(|_| DEFAULT_DB_NAME.to_string()),
            "fake_user",
            "fake_password",
        )
        .with_schema_path("./tests/schema.yaml")
        .with_auth_mode(AuthMode::Basic)
        .apply_schema()
        .build()
        .await
    {
        Ok(_) => panic!("should have failed"),
        Err(e) => match e {
            Error::Unauthorized(db_error) => {
                assert_eq!(db_error.unwrap().http_error.http_code(), 401)
            }
            _ => panic!("wrong error"),
        },
    }

    match DatabaseConnection::builder()
        .with_credentials(
            &std::env::var("DB_HOST").unwrap_or_else(|_| DEFAULT_DB_HOST.to_string()),
            &std::env::var("DB_NAME").unwrap_or_else(|_| DEFAULT_DB_NAME.to_string()),
            "fake_user",
            "fake_password",
        )
        .with_schema_path("./tests/schema.yaml")
        .with_auth_mode(AuthMode::Jwt)
        .apply_schema()
        .build()
        .await
    {
        Ok(_) => panic!("should have failed"),
        Err(e) => match e {
            Error::Unauthorized(db_error) => {
                assert_eq!(db_error.unwrap().http_error.http_code(), 401)
            }
            _ => panic!("wrong error"),
        },
    }
}

#[maybe_async::test(
    feature = "blocking",
    async(all(not(feature = "blocking")), tokio::test)
)]
async fn operation_options() {
    let connection = DatabaseConnection::builder()
        .with_credentials(
            &std::env::var("DB_HOST").unwrap_or_else(|_| DEFAULT_DB_HOST.to_string()),
            &std::env::var("DB_NAME").unwrap_or_else(|_| DEFAULT_DB_NAME.to_string()),
            &std::env::var("DB_USER").unwrap_or_else(|_| DEFAULT_DB_USER.to_string()),
            &std::env::var("DB_PWD").unwrap_or_else(|_| DEFAULT_DB_PWD.to_string()),
        )
        .with_schema_path("./tests/schema.yaml")
        .apply_schema()
        .with_operation_options(
            OperationOptions::default()
                .wait_for_sync(true)
                .ignore_hooks(true),
        )
        .build()
        .await
        .unwrap();
    let options = connection.operation_options();
    assert_eq!(options.wait_for_sync, Some(true));
    assert!(options.ignore_revs);
    assert!(options.ignore_hooks);
    // the hook is not launched
    DatabaseRecord::create(
        Dish {
            name: "Cordon Bleu".to_string(),
            price: 7,
        },
        &connection,
    )
    .await
    .unwrap();
    // The hook is launched manually
    match DatabaseRecord::create_with_options(
        Dish {
            name: "Cordon Bleu".to_string(),
            price: 7,
        },
        &connection,
        OperationOptions::default(),
    )
    .await
    {
        Ok(_) => panic!("Hook should have launched failure"),
        Err(e) => match e {
            Error::InternalError { message } => {
                assert_eq!(message.unwrap(), "Hook forbids creation".to_string())
            }
            _ => panic!("Wrong error"),
        },
    }
}
