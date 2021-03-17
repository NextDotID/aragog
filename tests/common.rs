use std::fmt::Debug;

use aragog::DatabaseConnectionPool;

pub const DEFAULT_DB_HOST: &str = "http://localhost:8529";
pub const DEFAULT_DB_NAME: &str = "aragog_test";
pub const DEFAULT_DB_USER: &str = "test";
pub const DEFAULT_DB_PWD: &str = "test";

#[maybe_async::maybe_async]
pub async fn setup_db() -> DatabaseConnectionPool {
    let pool = DatabaseConnectionPool::builder()
        .with_credentials(
            &std::env::var("DB_HOST").unwrap_or(DEFAULT_DB_HOST.to_string()),
            &std::env::var("DB_NAME").unwrap_or(DEFAULT_DB_NAME.to_string()),
            &std::env::var("DB_USER").unwrap_or(DEFAULT_DB_USER.to_string()),
            &std::env::var("DB_PWD").unwrap_or(DEFAULT_DB_PWD.to_string()),
        )
        .with_schema_path("./tests/schema.yaml")
        .apply_schema()
        .build()
        .await
        .unwrap();
    pool.truncate().await;
    pool
}

pub fn expect_assert(expr: bool) -> Result<(), String> {
    if !expr {
        Err(format!("Failed expectation"))
    } else {
        Ok(())
    }
}

pub fn expect_assert_eq<T: PartialEq>(expr1: T, expr2: T) -> Result<(), String>
where
    T: Debug,
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
