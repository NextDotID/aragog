use std::collections::HashMap;

use arangors::client::reqwest::ReqwestClient;
use arangors::{Collection, Database};

use crate::DatabaseAccess;

/// Struct equivalent to [`DatabaseConnectionPool`] for transactional operations.
///
/// [`DatabaseConnectionPool`]: ../struct.DatabaseConnectionPool.html
#[derive(Debug, Clone)]
pub struct TransactionPool {
    pub(crate) collections: HashMap<String, Collection<ReqwestClient>>,
    pub(crate) database: Database<ReqwestClient>,
}

impl DatabaseAccess for TransactionPool {
    fn get_collection(&self, collection: &str) -> &Collection<ReqwestClient> {
        if !self.collections.contains_key(collection) {
            panic!(
                "Undefined collection {}, check your schema.yaml file",
                collection
            )
        }
        &self.collections[collection]
    }

    fn database(&self) -> &Database<ReqwestClient> {
        &self.database
    }
}
