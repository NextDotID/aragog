use arangors::{Collection, Database, ClientError};
use arangors::client::reqwest::ReqwestClient;
use arangors::index::{IndexCollection, Index, IndexSettings};
use serde_json::Value;
use crate::helpers::json_helper;
use crate::AragogServiceError;

/// Struct containing the connection information on a ArangoDB collection
#[derive(Debug, Clone)]
pub struct DatabaseCollection {
    /// String name of the collection, exactly as defined in database
    pub collection_name: String,
    /// The collection wrapper accessor of `arangors` crate driver
    pub collection: Collection<ReqwestClient>,
}

impl DatabaseCollection {
    /// Retrieves all indexes of this collection
    ///
    /// # Arguments
    ///
    /// * `database` - the database accessor reference
    ///
    /// # Returns
    ///
    /// On success a instance of `arangors::index:IndexCollection` is returned, an `arangors::ClientError` is raised
    /// on failure.
    pub async fn get_indexes(&self, database: &Database<ReqwestClient>) -> Result<IndexCollection, ClientError> {
        database.indexes(&self.collection_name).await
    }

    /// Checks if the provided index already exists for the collection.
    ///
    /// # Arguments
    ///
    /// * `database` - the database accessor reference
    /// * `index` - the index to check existence
    ///
    /// # Returns
    ///
    /// `true` on success, a `arangors::ClientError` on failure
    pub async fn index_exists(&self, database: &Database<ReqwestClient>, index: &Index) -> Result<bool, ClientError> {
        let indexes = self.get_indexes(database).await?.indexes;
        if indexes.is_empty() { return Ok(false); }
        for idx in indexes {
            if idx.name == index.name && idx.fields == index.fields {
                return Ok(true);
            }
        }
        Ok(false)
    }

    /// Loads an Index from a json value
    ///
    /// # Arguments
    ///
    /// * `json` - the json value to parse
    ///
    /// # Returns
    ///
    /// returns an Index instance on success, a string error message on failure
    ///
    /// # Panics
    ///
    /// Panics if some keys are missing or of an invalid format, the deserialization must be precise.
    pub fn index_from_json(json: &Value) -> Result<Index, String> {
        let name = json_helper::load_json_string_key(json, "name")?;
        let tmp_fields = json.get("fields").unwrap().as_array().unwrap();
        let mut fields = vec![];
        for value in tmp_fields.iter() {
            fields.push(json_helper::load_json_string(value)?);
        }
        let settings: IndexSettings = serde_json::from_value(json["settings"].clone()).unwrap();
        Ok(Index::builder()
            .name(name)
            .fields(fields)
            .settings(settings)
            .build())
    }

    pub async fn record_count(&self) -> Result<u32, AragogServiceError> {
       let properties = match self.collection.document_count().await {
           Ok(value) => value,
           Err(_client_error) => return Err(AragogServiceError::UnprocessableEntity)
       };
       match properties.info.count {
           Some(value) => Ok(value),
           None => Ok(0)
       }
    }
}
