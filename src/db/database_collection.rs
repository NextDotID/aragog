use crate::ServiceError;
use arangors::client::reqwest::ReqwestClient;
use arangors::Collection;

/// Struct containing the connection information on a ArangoDB collection
#[derive(Debug, Clone)]
pub struct DatabaseCollection {
    /// String name of the collection, exactly as defined in database
    pub collection_name: String,
    /// The collection wrapper accessor of `arangors` crate driver
    pub collection: Collection<ReqwestClient>,
}

impl DatabaseCollection {
    /// Retrieves the total document count of this collection.
    ///
    /// # Returns
    ///
    /// On success a `i32` is returned as the document count.
    /// On failure a ServiceError wil be returned.
    #[maybe_async::maybe_async]
    pub async fn record_count(&self) -> Result<u32, ServiceError> {
        let properties = match self.collection.document_count().await {
            Ok(value) => value,
            Err(client_error) => return Err(ServiceError::from(client_error)),
        };
        match properties.info.count {
            Some(value) => Ok(value),
            None => Ok(0),
        }
    }
}
