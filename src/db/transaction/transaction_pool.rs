use std::collections::HashMap;

use arangors::client::reqwest::ReqwestClient;
use arangors::Database;

use crate::db::database_collection::DatabaseCollection;
use crate::DatabaseAccess;

/// Struct equivalent to [`DatabaseConnectionPool`] for transactional operations.
///
/// [`DatabaseConnectionPool`]: ../struct.DatabaseConnectionPool.html
#[derive(Debug, Clone)]
pub struct TransactionPool {
    pub(crate) collections: HashMap<String, DatabaseCollection>,
    pub(crate) database: Database<ReqwestClient>,
}

impl DatabaseAccess for TransactionPool {
    fn collection(&self, collection: &str) -> Option<&DatabaseCollection> {
        self.collections.get(collection)
    }

    fn database(&self) -> &Database<ReqwestClient> {
        &self.database
    }
}
