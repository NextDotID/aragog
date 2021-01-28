use crate::schema::SchemaDatabaseOperation;
use arangors::client::reqwest::ReqwestClient;
use arangors::index::{Index, IndexSettings};
use arangors::{ClientError, Database};
use serde::{Deserialize, Serialize};

/// Aragog schema representation of an ArangoDB Index.
/// This struct is meant to load/generate the schema file.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct IndexSchema {
    /// Index name (must be unique)
    pub name: String,
    /// Collection name
    pub collection: String,
    /// Index fields
    pub fields: Vec<String>,
    /// Index settings
    pub settings: IndexSettings,
}

impl Into<Index> for IndexSchema {
    fn into(self) -> Index {
        Index::builder()
            .name(self.name)
            .fields(self.fields)
            .settings(self.settings)
            .build()
    }
}

impl IndexSchema {
    /// Retrieve the index id
    pub fn id(&self) -> String {
        format!("{}/{}", &self.collection, &self.name)
    }
}

#[maybe_async::maybe_async]
impl SchemaDatabaseOperation for IndexSchema {
    type PoolType = Index;

    async fn apply_to_database(
        &self,
        database: &Database<ReqwestClient>,
        silent: bool,
    ) -> Result<Option<Self::PoolType>, ClientError> {
        log::debug!("Creating index {}", &self.name);
        let duplicate = serde_json::to_string(self).unwrap();
        let duplicate: Self = serde_json::from_str(&duplicate).unwrap();
        let index = duplicate.into();
        Self::handle_pool_result(
            database.create_index(&self.collection, &index).await,
            silent,
        )
    }

    async fn drop(&self, database: &Database<ReqwestClient>) -> Result<(), ClientError> {
        log::debug!("Deleting index {}", &self.name);
        database.delete_index(&self.id()).await?;
        Ok(())
    }

    async fn get(&self, database: &Database<ReqwestClient>) -> Result<Self::PoolType, ClientError> {
        database.index(&self.name).await
    }
}
