use std::collections::HashMap;

use arangors_lite::Database;

use crate::db::database_collection::DatabaseCollection;
use crate::{DatabaseAccess, OperationOptions};

/// Struct equivalent to [`DatabaseConnection`] for transactional operations.
///
/// [`DatabaseConnection`]: ../struct.DatabaseConnection.html
#[derive(Debug, Clone)]
pub struct TransactionDatabaseConnection {
    pub(crate) collections: HashMap<String, DatabaseCollection>,
    pub(crate) database: Database,
    pub(crate) operation_options: OperationOptions,
}

impl DatabaseAccess for TransactionDatabaseConnection {
    fn operation_options(&self) -> OperationOptions {
        self.operation_options.clone()
    }

    fn collection(&self, collection: &str) -> Option<&DatabaseCollection> {
        self.collections.get(collection)
    }

    fn database(&self) -> &Database {
        &self.database
    }
}
