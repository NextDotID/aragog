use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::{DatabaseRecord, Record, ServiceError};

/// Result of a succeeded [`Query`].
///
/// [`Query`]: struct.Query.html
pub struct QueryResult<T: Record + Clone + Serialize + DeserializeOwned> {
    /// Vector of the returned documents
    pub documents: Vec<DatabaseRecord<T>>,
    /// The total `documents` count
    pub doc_count: usize,
}

impl<T: Record + Clone + Serialize + DeserializeOwned> QueryResult<T> {
    /// Returns the only document of the current `QueryResult`.
    /// If there is no document or more than one, a [`ServiceError`]::[`NotFound`] is returned.
    ///
    /// [`ServiceError`]: enum.ServiceError.html
    /// [`NotFound`]: enum.ServiceError.html#variant.NotFound
    pub fn uniq(self) -> Result<DatabaseRecord<T>, ServiceError> {
        if self.doc_count <= 0 || self.doc_count > 1 {
            log::error!("Wrong number of {} returned: {}", T::collection_name(), self.doc_count);
            return Err(ServiceError::NotFound(format!("{} document not found", T::collection_name())));
        }
        Ok(self.documents.into_iter().nth(0).unwrap())
    }

    /// Returns the first document of the current `QueryResult`.
    /// Returns `None` if there are no documents
    pub fn first(self) -> Option<DatabaseRecord<T>> {
        if self.doc_count <= 0 {
            return None;
        }
        Some(self.documents.into_iter().nth(0).unwrap())
    }

    /// Returns the last document of the current `QueryResult`.
    /// Returns `None` if there are no documents
    pub fn last(self) -> Option<DatabaseRecord<T>> {
        if self.doc_count <= 0 {
            return None;
        }
        Some(self.documents.into_iter().nth(self.doc_count - 1).unwrap())
    }

    /// Returns the length of `documents`
    pub fn len(&self) -> usize { self.doc_count }
}