use arangors::client::reqwest::ReqwestClient;
use arangors::{Collection, Database};
use serde_json::Value;

use crate::query::JsonQueryResult;
use crate::ServiceError;

/// The `DatabaseAccess` trait of the `Aragog` library.
///
/// It defines the abstract level for database access requirements.
///
/// # Usage
///
/// Instead of direcly calling [`DatabaseConnectionPool`],
/// which is the main database accessor, this allow for a more abstract and modular system.
/// This way, the `Transaction` system can work with all the current methods.
///
/// # Note:
/// this trait is meant for development purposes, for a classic use of the library you don't need this trait.
///
/// [`DatabaseConnectionPool`]: struct.DatabaseConnectionPool.html
#[maybe_async::maybe_async]
pub trait DatabaseAccess: Sync {
    /// Retrieves a Collection from the database accessor.
    fn get_collection(&self, collection: &str) -> &Collection<ReqwestClient>;

    /// Retrieves the database object
    fn database(&self) -> &Database<ReqwestClient>;

    /// Runs an AQL query and returns the found documents
    async fn aql_get(&self, aql: &str) -> Result<JsonQueryResult, ServiceError> {
        log::debug!("Executing AQL: {}", aql);
        let query_result: Vec<Value> = match self.database().aql_str(aql).await {
            Ok(value) => value,
            Err(error) => {
                log::error!("{}", error);
                return Err(ServiceError::from(error));
            }
        };
        Ok(JsonQueryResult::new(query_result))
    }
}
