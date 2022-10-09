#![allow(dead_code)]

use std::fmt::Debug;

use aragog::DatabaseConnection;

pub const DEFAULT_DB_HOST: &str = "http://localhost:8529";
pub const DEFAULT_DB_NAME: &str = "aragog_test";
pub const DEFAULT_DB_USER: &str = "test";
pub const DEFAULT_DB_PASSWORD: &str = "test";

#[maybe_async::maybe_async]
pub async fn setup_db() -> DatabaseConnection {
    let connection = DatabaseConnection::builder()
        .with_credentials(
            &std::env::var("DB_HOST").unwrap_or_else(|_| DEFAULT_DB_HOST.to_string()),
            &std::env::var("DB_NAME").unwrap_or_else(|_| DEFAULT_DB_NAME.to_string()),
            &std::env::var("DB_USER").unwrap_or_else(|_| DEFAULT_DB_USER.to_string()),
            &std::env::var("DB_PASSWORD").unwrap_or_else(|_| DEFAULT_DB_PASSWORD.to_string()),
        )
        .with_schema_path("./tests/schema.yaml")
        .apply_schema()
        .build()
        .await
        .unwrap();
    connection.truncate().await;
    connection
}

pub fn expect_assert(expr: bool) -> Result<(), String> {
    if !expr {
        Err("Failed expectation".to_string())
    } else {
        Ok(())
    }
}

pub fn expect_assert_eq<T>(expr1: T, expr2: T) -> Result<(), String>
where
    T: Debug + PartialEq,
{
    if expr1 != expr2 {
        Err(format!(
            "Failed expectation, {:?} and {:?} are not equal",
            expr1, expr2
        ))
    } else {
        Ok(())
    }
}
