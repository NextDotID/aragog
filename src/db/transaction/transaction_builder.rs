use crate::db::database_collection::DatabaseCollection;
use crate::transaction::{Transaction, TransactionPool};
use crate::{DatabaseAccess, DatabaseConnectionPool, OperationOptions, ServiceError};
use arangors::transaction::{TransactionCollections, TransactionSettings};
use std::collections::HashMap;

const LOCK_TIMEOUT: usize = 60000;

/// Builder for Aragog [`Transaction`]
///
/// [`Transaction`]: struct.Transaction.html
pub struct TransactionBuilder {
    collections: Option<Vec<String>>,
    wait_for_sync: Option<bool>,
    lock_timeout: Option<usize>,
    operation_options: Option<OperationOptions>,
}

impl TransactionBuilder {
    /// Instantiates a new builder for a [`Transaction`]
    ///
    /// [`Transaction`]: struct.Transaction.html
    pub fn new() -> Self {
        Self::default()
    }

    /// The built transaction will be restricted to the specified collection names
    pub fn collections(mut self, collections: Vec<String>) -> Self {
        self.collections = Some(collections);
        self
    }

    /// The built transaction will wait for Database synchronization
    pub fn wait_for_sync(mut self) -> Self {
        self.wait_for_sync = Some(true);
        self
    }

    /// Defines the transaction lock timeout (default value: 60 000)
    pub fn lock_timeout(mut self, lock_timeout: usize) -> Self {
        self.lock_timeout = Some(lock_timeout);
        self
    }

    /// Defines custom `write` operation options for this transaction.
    /// By default the options set in the [`DatabaseConnectionPool`] are used.
    ///
    /// [`DatabaseConnectionPool`]: struct.DatabaseConnectionPool.html
    pub fn operation_options(mut self, options: OperationOptions) -> Self {
        self.operation_options = Some(options);
        self
    }

    /// Builds the transaction with the database connection pool
    #[maybe_async::maybe_async]
    pub async fn build(
        self,
        db_pool: &DatabaseConnectionPool,
    ) -> Result<Transaction, ServiceError> {
        let collection_names = self.collections.unwrap_or(db_pool.collections_names());
        let accessor = db_pool
            .database()
            .begin_transaction(
                TransactionSettings::builder()
                    .lock_timeout(self.lock_timeout.unwrap_or(LOCK_TIMEOUT))
                    .wait_for_sync(self.wait_for_sync.unwrap_or(false))
                    .collections(
                        TransactionCollections::builder()
                            .write(collection_names.clone())
                            .build(),
                    )
                    .build(),
            )
            .await?;
        log::trace!("Initialized ArangoDB transaction {}", accessor.id());
        // TODO: Change this for direct Collection<> transition when `arangors` supports it
        let mut collections = HashMap::new();
        for collections_name in db_pool.collections_names().iter() {
            let collection = accessor.collection(collections_name).await?;
            collections.insert(
                collections_name.clone(),
                DatabaseCollection::from(collection),
            );
        }
        //
        log::trace!("Initialized Aragog transaction pool");
        let database = db_pool.database().clone();
        let operation_options = self
            .operation_options
            .unwrap_or(db_pool.operation_options());
        Ok(Transaction {
            accessor,
            pool: TransactionPool {
                collections,
                database,
                operation_options,
            },
        })
    }
}

impl Default for TransactionBuilder {
    fn default() -> Self {
        Self {
            collections: None,
            wait_for_sync: None,
            lock_timeout: None,
            operation_options: None,
        }
    }
}
