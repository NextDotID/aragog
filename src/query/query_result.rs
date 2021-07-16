use crate::undefined_record::UndefinedRecord;
use crate::{DatabaseRecord, Record, ServiceError};
use std::ops::{Deref, DerefMut};

/// Query result containing the queried documents
#[derive(Debug, Clone)]
pub struct QueryResult<T>(pub Vec<DatabaseRecord<T>>);

impl<T: Clone + Record> QueryResult<T> {
    /// Instantiates a new `QueryResult` from a document collection
    pub fn new(documents: Vec<DatabaseRecord<T>>) -> Self {
        Self(documents)
    }

    /// Consumes and returns the only document of the current `QueryResult`.
    /// If there is no document or more than one, a [`ServiceError`]::[`NotFound`] is returned.
    ///
    /// [`ServiceError`]: enum.ServiceError.html
    /// [`NotFound`]: enum.ServiceError.html#variant.NotFound
    pub fn uniq(self) -> Result<DatabaseRecord<T>, ServiceError> {
        if self.is_empty() || self.len() > 1 {
            log::error!(
                "Wrong number of {} returned: {}",
                T::COLLECTION_NAME,
                self.len()
            );
            return Err(ServiceError::NotFound {
                item: T::COLLECTION_NAME.to_string(),
                id: "queried".to_string(),
                source: None,
            });
        }
        Ok(self.0.into_iter().next().unwrap())
    }

    /// Consumes and returns the first document of the current `QueryResult`
    pub fn first_record(self) -> Option<DatabaseRecord<T>> {
        self.0.into_iter().next()
    }
}

impl QueryResult<UndefinedRecord> {
    /// Retrieves deserialized documents from the json results. The documents not matching `T` will not be returned.
    ///
    /// # Example
    /// If you want to do a graph query that can return different models you can use this method to retrieve the serialized record:
    ///
    /// ```rust no_run
    /// # use aragog::{query::Query, Record, DatabaseConnection};
    /// # use serde::{Serialize, Deserialize};
    /// #
    /// # #[derive(Record, Clone, Serialize, Deserialize)]
    /// # struct User {}
    /// # #[derive(Record, Clone, Serialize, Deserialize)]
    /// # struct Topic {}
    /// # #[derive(Record, Clone, Serialize, Deserialize)]
    /// # struct Role {}
    /// # #[tokio::main]
    /// # async fn main() {
    /// # let db_accessor = DatabaseConnection::builder().build().await.unwrap();
    /// let json_results = Query::outbound(1, 5, "ChildOf", "User/123").call(&db_accessor).await.unwrap();
    ///
    /// let user_results = json_results.get_records::<User>();
    /// let topic_results = json_results.get_records::<Topic>();
    /// let role_results = json_results.get_records::<Role>();
    /// # }
    /// ```
    pub fn get_records<T: Record>(&self) -> QueryResult<T> {
        let mut res = Vec::new();
        for db_record in self.iter() {
            let doc = serde_json::from_value(db_record.0.clone());
            if let Ok(record) = doc {
                res.push(DatabaseRecord {
                    key: db_record.key.clone(),
                    id: db_record.id.clone(),
                    rev: db_record.rev.clone(),
                    record,
                });
            } else {
                continue;
            }
        }
        res.into()
    }
}

impl<T: Record> From<Vec<DatabaseRecord<T>>> for QueryResult<T> {
    fn from(documents: Vec<DatabaseRecord<T>>) -> Self {
        Self::new(documents)
    }
}

impl<T: Record> Deref for QueryResult<T> {
    type Target = Vec<DatabaseRecord<T>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: Record> DerefMut for QueryResult<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
