use arangors::client::reqwest::ReqwestClient;
use arangors::{ClientError, Collection, Database};
use serde::{Deserialize, Serialize};

use crate::schema::SchemaDatabaseOperation;

/// Aragog schema representation of an ArangoDB Collection.
/// This struct is meant to load/generate the schema file.
#[derive(Debug, Serialize, Deserialize)]
pub struct CollectionSchema {
    /// Collection name
    pub name: String,
    /// Is the collection a edge collection
    pub is_edge_collection: bool,
}

impl CollectionSchema {
    /// Initializes a new collection schema with the collection name and a flag to define if
    /// it's an edge collection or not.
    pub fn new(name: &str, is_edge_collection: bool) -> Self {
        Self {
            name: name.to_string(),
            is_edge_collection,
        }
    }
}

#[maybe_async::maybe_async]
impl SchemaDatabaseOperation for CollectionSchema {
    type PoolType = Collection<ReqwestClient>;

    async fn apply_to_database(
        &mut self,
        database: &Database<ReqwestClient>,
        silent: bool,
    ) -> Result<(), ClientError> {
        log::debug!("Creating Collection {}", &self.name);
        if self.is_edge_collection {
            Self::handle_error(database.create_edge_collection(&self.name).await, silent)?;
        } else {
            Self::handle_error(database.create_collection(self.name.as_str()).await, silent)?;
        }
        Ok(())
    }

    async fn drop(&self, database: &Database<ReqwestClient>) -> Result<(), ClientError> {
        log::debug!("Deleting Collection {}", &self.name);
        database.drop_collection(&self.name).await?;
        Ok(())
    }

    async fn get(&self, database: &Database<ReqwestClient>) -> Result<Self::PoolType, ClientError> {
        database.collection(&self.name).await
    }
}
