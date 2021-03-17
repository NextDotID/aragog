#[cfg(feature = "async")]
use std::future::Future;

use arangors::client::reqwest::ReqwestClient;
use arangors::transaction::Transaction as TransactionLayer;

pub use {
    transaction_builder::TransactionBuilder, transaction_output::TransactionOutput,
    transaction_pool::TransactionPool,
};

use crate::{DatabaseConnectionPool, ServiceError};

mod transaction_builder;
mod transaction_output;
mod transaction_pool;

/// Struct representing a ArangoDB transaction.
///
/// Its `pool` is equivalent to a [`DatabaseConnectionPool`] but for transactional operations.
/// Use it instead of the classic pool to use the streaming transaction.
///
/// # Example
///
/// ```rust
/// # use aragog::{DatabaseConnectionPool, transaction::Transaction, Record, Validate, DatabaseRecord};
/// # use aragog::query::{Comparison, Filter};
/// # use serde::{Serialize, Deserialize};
///
/// #[derive(Debug, Clone, Record, Validate, Serialize, Deserialize)]
/// pub struct User {
///     pub field1: String,
///     pub field2: String
/// }
///
/// # #[tokio::main]
/// # async fn main() {
/// let db_pool = DatabaseConnectionPool::builder()
///     # .with_schema_path("tests/schema.yaml")
///     # .apply_schema()
///     .build()
///     .await
///     .unwrap();
/// # db_pool.truncate().await;
/// // Build a transaction pool from the main database pool
/// let transaction = Transaction::new(&db_pool).await.unwrap();
/// // Safely execute document operations in the transaction, the transaction will be closed afterwards
/// let result = transaction.safe_execute(|pool| async move {
///     // All operations here will be transactional, if an error is raised, the transaction will be aborted.
///     let doc = User {
///         field1: String::from("foo"),
///         field2: String::from("bar"),
///     };
///     // The closure safely checks for errors, use the `?` operator and avoid `unwrap()`
///     let mut db_doc = DatabaseRecord::create(doc, &pool).await?;
///     db_doc.field1 = String::from("not foo");
///     db_doc.save(&pool).await?;
///     Ok(db_doc)
/// }).await.unwrap();
///
/// // We make sure everything was committed
/// assert!(result.is_committed());
/// // We retrieve our document from the classic pool to check if it worked
/// let result = User::get(
///     User::query().filter(Comparison::field("field1").equals_str("not foo").into()),
///     &db_pool
/// ).await.unwrap();
/// assert_eq!(result.len(), 1);
/// # }
/// ```
///
/// # Note
///
/// The `WRITE` transaction operations muse be document related: `create`, `save`, `delete`, etc. The AQL operations may not work.
/// On the other hand all `READ` operations as `find`, `get`, etc should all work even with `AQL` queries.
///
/// [`DatabaseConnectionPool`]: ../struct.DatabaseConnectionPool.html
// TODO: Add #[derive(Debug)] when arangors supports it
pub struct Transaction {
    accessor: TransactionLayer<ReqwestClient>,
    pool: TransactionPool,
}

impl Transaction {
    /// Transaction unique identifier
    pub fn id(&self) -> &str {
        self.accessor.id()
    }

    /// Instantiates a new `Transaction` from a [`DatabaseConnectionPool`] on all collections
    ///
    /// # Arguments
    ///
    /// * `db_pool` - The current database pool
    ///
    /// The transaction will be initialized with default settings:
    /// - No disk writing wait (waitForSync)
    /// - A lock timeout of 60 000
    /// - No collection restriction
    ///
    /// For more options use [`TransactionBuilder`]
    ///
    /// [`DatabaseConnectionPool`]: ../struct.DatabaseConnectionPool.html
    /// [`TransactionBuilder`]: struct.TransactionBuilder.html
    #[maybe_async::maybe_async]
    pub async fn new(db_pool: &DatabaseConnectionPool) -> Result<Self, ServiceError> {
        TransactionBuilder::new().build(db_pool).await
    }

    /// Tries to commit all operations from the transaction
    ///
    /// A `Transaction` instance can be committed multiple times.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use aragog::{DatabaseConnectionPool, transaction::Transaction, Record, Validate, DatabaseRecord};
    /// # use serde::{Serialize, Deserialize};
    ///
    /// #[derive(Debug, Clone, Record, Validate, Serialize, Deserialize)]
    /// pub struct User {
    ///     pub field1: String,
    ///     pub field2: String
    /// }
    ///
    /// # #[tokio::main]
    /// # async fn main() {
    /// # let db_pool = DatabaseConnectionPool::builder()
    ///     # .with_schema_path("tests/schema.yaml")
    ///     # .apply_schema()
    ///     # .build()
    ///     # .await
    ///     # .unwrap();
    /// // Build a transaction pool from the main database pool
    /// let transaction = Transaction::new(&db_pool).await.unwrap();
    /// let doc = User {
    ///     field1: String::from("foo"),
    ///     field2: String::from("bar"),
    /// };
    /// // Use the transaction pool instead of the standard `DatabaseConnectionPool`
    /// match DatabaseRecord::create(doc, transaction.pool()).await {
    ///     /// On the operation success we commit the complete pool of operations
    ///     Ok(_) => transaction.commit().await.unwrap(),
    ///     /// On the operation success we abort the complete pool of operations
    ///     Err(_) => transaction.abort().await.unwrap()
    /// }
    /// # }
    /// ```
    ///
    /// # Note
    ///
    /// For a more practical and safer use, use the `safe_execute` method which allows multiple operations
    #[maybe_async::maybe_async]
    pub async fn commit(&self) -> Result<(), ServiceError> {
        let status = self.accessor.commit().await?;
        log::debug!("Transaction committed with status: {:?}", status);
        // TODO: Create a critical error for wrong status, which should not happen
        Ok(())
    }

    /// Tries to abort all operations from the transaction.
    ///
    /// If the operation succeeds, the ArangoDB transaction will be deleted and the current
    /// `Transaction` instance can't be used anymore.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use aragog::{DatabaseConnectionPool, transaction::Transaction, Record, Validate, DatabaseRecord};
    /// # use serde::{Serialize, Deserialize};
    ///
    /// #[derive(Debug, Clone, Record, Validate, Serialize, Deserialize)]
    /// pub struct User {
    ///     pub field1: String,
    ///     pub field2: String
    /// }
    ///
    /// # #[tokio::main]
    /// # async fn main() {
    /// # let db_pool = DatabaseConnectionPool::builder()
    ///     # .with_schema_path("tests/schema.yaml")
    ///     # .apply_schema()
    ///     # .build()
    ///     # .await
    ///     # .unwrap();
    /// // Build a transaction pool from the main database pool
    /// let transaction = Transaction::new(&db_pool).await.unwrap();
    /// let doc = User {
    ///     field1: String::from("foo"),
    ///     field2: String::from("bar"),
    /// };
    /// // Use the transaction pool instead of the standard `DatabaseConnectionPool`
    /// match DatabaseRecord::create(doc, transaction.pool()).await {
    ///     /// On the operation success we commit the complete pool of operations
    ///     Ok(_) => transaction.commit().await.unwrap(),
    ///     /// On the operation failure we abort the complete pool of operations
    ///     Err(_) => transaction.abort().await.unwrap()
    /// }
    /// # }
    /// ```
    ///
    /// # Note
    ///
    /// For a more practical and safer use, use the `safe_execute` method which allows multiple operations
    #[maybe_async::maybe_async]
    pub async fn abort(&self) -> Result<(), ServiceError> {
        let status = self.accessor.abort().await?;
        log::debug!("Transaction aborted with status: {:?}", status);
        // TODO: Create a critical error for wrong status, which should not happen
        Ok(())
    }

    /// Allows to run multiple operations using the transaction pool. If an operation fails or an `Err`
    /// is returned by the closure, all operations will be aborted
    ///
    /// # Example
    ///
    /// ```rust
    /// # use aragog::{DatabaseConnectionPool, transaction::Transaction, Record, Validate, DatabaseRecord};
    /// # use serde::{Serialize, Deserialize};
    ///
    /// #[derive(Debug, Clone, Record, Validate, Serialize, Deserialize)]
    /// pub struct User {
    ///     pub field1: String,
    ///     pub field2: String
    /// }
    ///
    /// # #[tokio::main]
    /// # async fn main() {
    /// # let db_pool = DatabaseConnectionPool::builder()
    ///     # .with_schema_path("tests/schema.yaml")
    ///     # .apply_schema()
    ///     # .build()
    ///     # .await
    ///     # .unwrap();
    /// # db_pool.truncate().await;
    /// // Build a transaction pool from the main database pool
    /// let transaction = Transaction::new(&db_pool).await.unwrap();
    /// // Safely execute document operations in the transaction
    /// transaction.safe_execute(|pool| async move {
    ///     // All operations here will be transactional, if an error is raised, the transaction will be aborted.
    ///     let doc = User {
    ///         field1: String::from("foo"),
    ///         field2: String::from("bar"),
    ///     };
    ///     // The closure safely checks for errors, use the `?` operator and avoid `unwrap()`
    ///     let mut db_doc = DatabaseRecord::create(doc, &pool).await?;
    ///     db_doc.field1 = String::from("not foo");
    ///     db_doc.save(&pool).await?;
    ///     Ok(db_doc)
    /// }).await.unwrap();
    /// # }
    /// ```
    ///
    /// # Note
    ///
    /// Don't use `unwrap()` in the closure, as if the code panics the transaction won't be aborted nor commited.
    #[cfg(feature = "async")]
    pub async fn safe_execute<T, O, F>(
        &self,
        operations: O,
    ) -> Result<TransactionOutput<T>, ServiceError>
    where
        O: FnOnce(TransactionPool) -> F,
        F: Future<Output = Result<T, ServiceError>>,
    {
        log::trace!("Safely executing transactional operations..");
        let res = operations(self.pool.clone()).await;
        log::trace!(
            "Safely executing transactional operations.. Done. Success: {}",
            res.is_ok()
        );
        self.handle_safe_execute(res).await
    }

    /// Allows to run multiple operations using the transaction pool. If an operation fails or an `Err`
    /// is returned by the closure, all operations will be aborted
    ///
    /// # Example
    ///
    /// ```rust
    /// # use aragog::{DatabaseConnectionPool, transaction::Transaction, Record, Validate, DatabaseRecord};
    /// # use serde::{Serialize, Deserialize};
    ///
    /// #[derive(Debug, Clone, Record, Validate, Serialize, Deserialize)]
    /// pub struct User {
    ///     pub field1: String,
    ///     pub field2: String
    /// }
    ///
    /// # #[tokio::main]
    /// # async fn main() {
    /// # let db_pool = DatabaseConnectionPool::builder()
    ///     # .with_schema_path("tests/schema.yaml")
    ///     # .apply_schema()
    ///     # .build()
    ///     # .await
    ///     # .unwrap();
    /// # db_pool.truncate().await;
    /// // Build a transaction pool from the main database pool
    /// let transaction = Transaction::new(&db_pool).await.unwrap();
    /// // Safely execute document operations in the transaction
    /// transaction.safe_execute(|pool| async move {
    ///     // All operations here will be transactional, if an error is raised, the transaction will be aborted.
    ///     let doc = User {
    ///         field1: String::from("foo"),
    ///         field2: String::from("bar"),
    ///     };
    ///     // The closure safely checks for errors, use the `?` operator and avoid `unwrap()`
    ///     let mut db_doc = DatabaseRecord::create(doc, &pool).await?;
    ///     db_doc.field1 = String::from("not foo");
    ///     db_doc.save(&pool).await?;
    ///     Ok(db_doc)
    /// }).await.unwrap();
    /// # }
    /// ```
    ///
    /// # Note
    ///
    /// Don't use `unwrap()` in the closure, as if the code panics the transaction won't be aborted nor commited.
    #[cfg(not(feature = "async"))]
    pub fn safe_execute<T, O>(&self, operations: O) -> Result<TransactionOutput<T>, ServiceError>
    where
        O: FnOnce(TransactionPool) -> Result<T, ServiceError>,
    {
        log::trace!("Safely executing transactional operations..");
        let res = operations(self.pool.clone());
        log::trace!(
            "Safely executing transactional operations.. Done. Success: {}",
            res.is_ok()
        );
        self.handle_safe_execute(res)
    }

    #[maybe_async::maybe_async]
    async fn handle_safe_execute<T>(
        &self,
        result: Result<T, ServiceError>,
    ) -> Result<TransactionOutput<T>, ServiceError> {
        match result {
            Ok(value) => {
                log::debug!("Transaction succeeded. Committing..");
                self.commit().await?;
                Ok(TransactionOutput::Committed(value))
            }
            Err(err) => {
                log::debug!("Transaction failed with: {}. Aborting..", err);
                self.abort().await?;
                Ok(TransactionOutput::Aborted(err))
            }
        }
    }

    /// Retrieves the pool of the transaction which implements [`DatabaseAccess`].
    /// This pool can be used exactly the same way was the classic database pool.
    ///
    /// [`DatabaseAccess`]: ../trait.DatabaseAccess.html
    pub fn pool(&self) -> &TransactionPool {
        &self.pool
    }
}
