use arangors::client::reqwest::ReqwestClient;
use arangors::Database;
use serde_json::Value;

use crate::db::database_collection::DatabaseCollection;
use crate::query::JsonQueryResult;
use crate::{OperationOptions, ServiceError};

/// The `DatabaseAccess` trait of the `Aragog` library.
///
/// It defines the abstract level for database access requirements.
///
/// # Usage
///
/// Instead of directly calling [`DatabaseConnectionPool`],
/// which is the main database accessor, this allow for a more abstract and modular system.
/// This way, the `Transaction` system can work with all the current methods.
///
/// # Note:
/// this trait is meant for development purposes, for a classic use of the library you don't need this trait.
///
/// [`DatabaseConnectionPool`]: struct.DatabaseConnectionPool.html
#[maybe_async::maybe_async]
pub trait DatabaseAccess: Sync {
    /// Defines the default operation options to use on `write` operations.
    ///
    /// This method will be used on:
    /// * [`DatabaseRecord`]::[`create`] ,
    /// * [`DatabaseRecord`]::[`save`] ,
    /// * [`DatabaseRecord`]::[`delete`] ,
    ///
    /// [`DatabaseRecord`]: struct.DatabaseRecord.html
    /// [`create`]: struct.DatabaseRecord.html#method.create
    /// [`save`]: struct.DatabaseRecord.html#method.save
    /// [`delete`]: struct.DatabaseRecord.html#method.delete
    fn operation_options(&self) -> OperationOptions {
        OperationOptions::default()
    }

    /// Retrieves a Collection from the database accessor.
    fn collection(&self, collection: &str) -> Option<&DatabaseCollection>;

    /// Retrieves a Collection from the database accessor.
    fn get_collection(&self, collection: &str) -> Result<&DatabaseCollection, ServiceError> {
        self.collection(collection).ok_or(ServiceError::NotFound {
            item: "Collection".to_string(),
            id: collection.to_string(),
            source: None,
        })
    }

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
