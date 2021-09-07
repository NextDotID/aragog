use arangors::Database;
use uclient::reqwest::ReqwestClient;

use crate::db::database_collection::DatabaseCollection;
use crate::db::database_service::{query_records, query_records_in_batches};
use crate::query::{Query, QueryCursor, QueryResult};
use crate::undefined_record::UndefinedRecord;
use crate::{OperationOptions, ServiceError};

/// The `DatabaseAccess` trait of the `Aragog` library.
///
/// It defines the abstract level for database access requirements.
///
/// # Usage
///
/// Instead of directly calling [`DatabaseConnection`],
/// which is the main database accessor, this allow for a more abstract and modular system.
/// This way, the `Transaction` system can work with all the current methods.
///
/// # Note:
/// this trait is meant for development purposes, for a classic use of the library you don't need this trait.
///
/// [`DatabaseConnection`]: struct.DatabaseConnection.html
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

    /// Runs an AQL query and returns the found documents as undefined records.
    ///
    /// # Note
    ///
    /// The returned documents are simple wrappers for `serde_json`::`Value` values.
    /// Typed `Record` can be dynamically retrieved afterwards.
    ///
    /// If you want a specific [`Record`] type use [`DatabaseRecord`]::[`get`] directly.
    ///
    /// [`Record`]: trait.Record.html
    /// [`DatabaseRecord`]: struct.DatabaseRecord.html
    /// [`get`]: struct.DatabaseRecord.html#method.get
    async fn query(&self, query: Query) -> Result<QueryResult<UndefinedRecord>, ServiceError> {
        query_records(self, query.to_aql().as_str()).await
    }

    /// Runs an AQL query using batches and returns a cursor on the found documents as undefined records.
    ///
    /// # Note
    ///
    /// The returned documents are simple wrappers for `serde_json`::`Value` values.
    /// Typed `Record` can be dynamically retrieved afterwards.
    ///
    /// If you want a specific [`Record`] type use [`DatabaseRecord`]::[`get_in_batches`] directly.
    ///
    /// [`Record`]: trait.Record.html
    /// [`DatabaseRecord`]: struct.DatabaseRecord.html
    /// [`get_in_batches`]: struct.DatabaseRecord.html#method.get_in_batches
    async fn query_in_batches(
        &self,
        query: Query,
        batch_size: u32,
    ) -> Result<QueryCursor<UndefinedRecord>, ServiceError> {
        query_records_in_batches(self, query.to_aql().as_str(), batch_size).await
    }
}
