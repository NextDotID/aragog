use aragorn::DatabaseConnectionPool;
use std::fmt::Debug;

pub fn setup_db() -> DatabaseConnectionPool {
    std::env::set_var("SCHEMA_PATH", "./tests/schema.json");

    let pool = tokio_test::block_on(DatabaseConnectionPool::new(
        "http://localhost:8529",
        "aragorn_test",
        "test",
        "test"
    ));
    truncate_db(&pool);
    pool
}

pub fn with_db<T>(test: T) -> Result<(), String> where T: FnOnce(&DatabaseConnectionPool) -> Result<(), String>
{
    let db_pool = setup_db();
    let res = test(&db_pool);
    truncate_db(&db_pool);
    res
}

pub fn truncate_db(pool: &DatabaseConnectionPool) {
    for collection in pool.collections.iter() {
        tokio_test::block_on(collection.1.collection.truncate()).unwrap();
    }
}

pub fn expect_assert(expr: bool) -> Result<(), String> {
    if !expr {
        Err(format!("Failed expectation"))
    } else {
        Ok(())
    }
}

pub fn expect_assert_eq<T: PartialEq>(expr1: T, expr2: T) -> Result<(), String> where T: Debug {
    if expr1 != expr2 {
        Err(format!("Failed expectation, {:?} and {:?} are not eqal", expr1, expr2))
    } else {
        Ok(())
    }
}
