use crate::schema::SchemaDatabaseOperation;
use arangors::client::reqwest::ReqwestClient;
use arangors::index::{Index, IndexSettings};
use arangors::{ClientError, Database};
use serde::{Deserialize, Serialize};

/// Aragog schema representation of an ArangoDB Index.
/// This struct is meant to load/generate the schema file.
#[derive(Debug, Serialize, Deserialize)]
pub struct IndexSchema {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The id of the index, can be `None` on creation but id required to drop the index.
    pub id: Option<String>,
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

#[maybe_async::maybe_async]
impl SchemaDatabaseOperation for IndexSchema {
    type PoolType = Index;

    async fn apply_to_database(
        &mut self,
        database: &Database<ReqwestClient>,
        silent: bool,
    ) -> Result<(), ClientError> {
        log::debug!("Creating index {}", &self.name);
        let duplicate = serde_json::to_string(self).unwrap();
        let duplicate: Self = serde_json::from_str(&duplicate).unwrap();
        let index = duplicate.into();
        match database.create_index(&self.collection, &index).await {
            Ok(index) => self.id = Some(index.id),
            Err(error) => Self::handle_error(Err(error) as Result<Index, ClientError>, silent)?,
        };
        Ok(())
    }

    async fn drop(&self, database: &Database<ReqwestClient>) -> Result<(), ClientError> {
        log::debug!("Deleting index {}", &self.name);
        database.delete_index(&self.id.as_ref().unwrap()).await?;
        Ok(())
    }

    async fn get(&self, database: &Database<ReqwestClient>) -> Result<Self::PoolType, ClientError> {
        database.index(&self.name).await
    }
}
