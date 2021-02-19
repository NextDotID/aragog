use std::collections::HashMap;

use arangors::client::reqwest::ReqwestClient;
use arangors::{Collection, Database};

use crate::{DatabaseAccess, ServiceError};

/// Struct equivalent to [`DatabaseConnectionPool`] for transactional operations.
///
/// [`DatabaseConnectionPool`]: ../struct.DatabaseConnectionPool.html
#[derive(Debug, Clone)]
pub struct TransactionPool {
    pub(crate) collections: HashMap<String, Collection<ReqwestClient>>,
    pub(crate) database: Database<ReqwestClient>,
}

impl DatabaseAccess for TransactionPool {
    fn get_collection(&self, collection: &str) -> Result<&Collection<ReqwestClient>, ServiceError> {
        match self.collections.get(collection) {
            Some(c) => Ok(c),
            None => Err(ServiceError::NotFound {
                item: "Collection".to_string(),
                id: collection.to_string(),
                source: None,
            }),
        }
    }

    fn database(&self) -> &Database<ReqwestClient> {
        &self.database
    }
}
