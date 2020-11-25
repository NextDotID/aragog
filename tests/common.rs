use std::fmt::Debug;

use aragog::{AuthMode, DatabaseConnectionPool};

const DEFAULT_DB_HOST: &str = "http://localhost:8529";
const DEFAULT_DB_NAME: &str = "aragog_test";
const DEFAULT_DB_USER: &str = "test";
const DEFAULT_DB_PWD: &str = "test";

pub fn setup_db() -> DatabaseConnectionPool {
    std::env::set_var("SCHEMA_PATH", "./tests/schema.json");

    let pool = tokio_test::block_on(DatabaseConnectionPool::new(
        &std::env::var("DB_HOST").unwrap_or(DEFAULT_DB_HOST.to_string()),
        &std::env::var("DB_NAME").unwrap_or(DEFAULT_DB_NAME.to_string()),
        &std::env::var("DB_USER").unwrap_or(DEFAULT_DB_USER.to_string()),
        &std::env::var("DB_PWD").unwrap_or(DEFAULT_DB_PWD.to_string()),
        AuthMode::Basic,
    ));
    tokio_test::block_on(pool.truncate());
    pool
}

pub fn with_db<T>(test: T) -> Result<(), String>
where
    T: FnOnce(&DatabaseConnectionPool) -> Result<(), String>,
{
    let db_pool = setup_db();
    let res = test(&db_pool);
    tokio_test::block_on(db_pool.truncate());
    res
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
