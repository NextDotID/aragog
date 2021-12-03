use crate::Error;
use arangors_lite::Collection;
use std::ops::Deref;

/// Struct containing the connection information on a `ArangoDB` collection
#[derive(Debug, Clone)]
pub struct DatabaseCollection {
    /// The collection wrapper accessor of `arangors_lite` crate driver
    collection: Collection,
}

impl DatabaseCollection {
    /// Name of the collection, exactly as defined in database
    pub fn name(&self) -> &str {
        self.collection.name()
    }

    /// Retrieves the total document count of this collection.
    ///
    /// # Returns
    ///
    /// On success a `i32` is returned as the document count.
    /// On failure a Error wil be returned.
    #[maybe_async::maybe_async]
    pub async fn record_count(&self) -> Result<u32, Error> {
        let properties = match self.collection.document_count().await {
            Ok(value) => value,
            Err(client_error) => return Err(Error::from(client_error)),
        };
        match properties.info.count {
            Some(value) => Ok(value),
            None => Ok(0),
        }
    }
}

impl From<Collection> for DatabaseCollection {
    fn from(collection: Collection) -> Self {
        Self { collection }
    }
}

impl Deref for DatabaseCollection {
    type Target = Collection;

    fn deref(&self) -> &Self::Target {
        &self.collection
    }
}
