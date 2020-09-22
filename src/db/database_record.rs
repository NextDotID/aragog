use arangors::AqlQuery;
use arangors::document::response::DocumentResponse;
use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json::Value;

use crate::db::database_service;
use crate::{ServiceError, Record, Validate, DatabaseConnectionPool, Authenticate};
use crate::query::{Query, QueryResult};

/// Struct representing database stored documents
///
/// The document of type `T` mut implement Serialize, DeserializeOwned, Clone and [`Record`]
///
/// [`Record`]: trait.Record.html
#[derive(Debug)]
pub struct DatabaseRecord<T: Serialize + DeserializeOwned + Clone + Record> {
    /// The Document unique and indexed key
    pub key: String,
    /// The deserialized stored document
    pub record: T,
}

#[allow(dead_code)]
impl<T: Serialize + DeserializeOwned + Clone + Record> DatabaseRecord<T> {
    /// Writes in the database the new state of the record, "saving it". The record will first be validates
    /// as it should implement the [`Validate`] trait.
    ///
    /// # Arguments:
    ///
    /// * `db_pool` - database connection pool reference
    ///
    /// # Returns
    ///
    /// On success `()` is returned, meaning that the current instance is up to date with the database state.
    /// On failure a [`ServiceError`] is returned:
    /// * [`Conflict`] on index uniqueness conflict
    /// * [`UnprocessableEntity`] on data corruption
    /// * [`ValidationError`] on failed field validations
    ///
    /// [`Validate`]: trait.Validate.html
    /// [`ServiceError`]: enum.ServiceError.html
    /// [`Conflict`]: enum.ServiceError.html#variant.Conflict
    /// [`UnprocessableEntity`]: enum.ServiceError.html#variant.UnprocessableEntity
    /// [`ValidationError`]: enum.ServiceError.html#variant.ValidationError
    pub async fn save(&mut self, db_pool: &DatabaseConnectionPool) -> Result<(), ServiceError> where T: Validate {
        self.record.validate()?;
        let new_record = database_service::update_record(self.record.clone(), &self.key, &db_pool, T::collection_name()).await?;
        self.record = new_record.record;
        Ok(())
    }

    /// Removes the record from the database.
    /// The structure won't be freed or emptied but the document won't exist in the global state
    ///
    /// # Arguments:
    ///
    /// * `db_pool` - database connection pool reference
    ///
    /// # Returns
    ///
    /// On success `()` is returned, meaning that the record is now deleted, the structure should not be used afterwards.
    /// On failure a [`ServiceError`] is returned:
    /// * [`NotFound`] on invalid document key
    /// * [`UnprocessableEntity`] on data corruption
    ///
    /// [`ServiceError`]: enum.ServiceError.html
    /// [`NotFound`]: enum.ServiceError.html#variant.NotFound
    /// [`UnprocessableEntity`]: enum.ServiceError.html#variant.UnprocessableEntity
    pub async fn delete(&self, db_pool: &DatabaseConnectionPool) -> Result<(), ServiceError> {
        database_service::remove_record::<T>(&self.key, &db_pool, T::collection_name()).await
    }

    /// Retrieves a record from the database with the associated unique `key`
    ///
    /// # Arguments:
    ///
    /// * `key` - the unique record key as a string slice
    /// * `db_pool` - database connection pool reference
    ///
    /// # Returns
    ///
    /// On success `Self` is returned,
    /// On failure a [`ServiceError`] is returned:
    /// * [`NotFound`] on invalid document key
    /// * [`UnprocessableEntity`] on data corruption
    ///
    /// [`ServiceError`]: enum.ServiceError.html
    /// [`NotFound`]: enum.ServiceError.html#variant.NotFound
    /// [`UnprocessableEntity`]: enum.ServiceError.html#variant.UnprocessableEntity
    pub async fn find(key: &str, db_pool: &DatabaseConnectionPool) -> Result<Self, ServiceError> {
        database_service::retrieve_record(key, &db_pool, T::collection_name()).await
    }

    /// Retrieves all records from the database matching the associated conditions.
    ///
    /// # Arguments:
    ///
    /// * `query` - The `Query` to match
    /// * `db_pool` - database connection pool reference
    ///
    /// # Note
    ///
    /// This is simply an AQL request wrapper.
    ///
    /// # Returns
    ///
    /// On success a `QueryResult` with a vector of `Self` is returned. It is can be empty.
    /// On failure a [`ServiceError`] is returned:
    /// * [`NotFound`] if no document matches the condition
    /// * [`UnprocessableEntity`] on data corruption
    ///
    /// # Example
    ///
    /// ```rust ignore
    /// use aragog::query::{Query, Comparison};
    ///
    /// let mut query = Query::new().filter(Filter::new(Comparison::field("username").equals_str("MichelDu93"))
    ///     .and(Comparison::field("age").greater_than(10));
    ///
    /// User::get(query, &db_pool).await.unwrap();
    /// ```
    ///
    /// [`ServiceError`]: enum.ServiceError.html
    /// [`NotFound`]: enum.ServiceError.html#variant.NotFound
    /// [`UnprocessableEntity`]: enum.ServiceError.html#variant.UnprocessableEntity
    pub async fn get(query: Query, db_pool: &DatabaseConnectionPool) -> Result<QueryResult<T>, ServiceError> {
        let query_result: Vec<Value> = match db_pool.database.aql_str(&query.render()).await {
            Ok(value) => { value }
            Err(error) => {
                log::error!("{}", error);
                return Err(ServiceError::from(error));
            }
        };
        let mut res = Vec::new();
        for value in query_result {
            res.push(DatabaseRecord {
                key: String::from(value["_key"].as_str().unwrap()),
                record: serde_json::from_str(&value.to_string()).unwrap(),
            })
        }
        Ok(QueryResult::new(res))
    }

    /// Checks if any document matching the associated conditions exist
    ///
    /// # Arguments:
    ///
    /// * `query` - The `Query` to match
    /// * `db_pool` - database connection pool reference
    ///
    /// # Note
    ///
    /// This is simply an AQL request wrapper.
    ///
    /// # Returns
    ///
    /// On success `true` is returned, `false` if nothing exists.
    ///
    /// # Example
    ///
    /// ```rust ignore
    /// use aragog::query::{Query, Comparison};
    ///
    /// let mut query = Query::new().filter(Filter::new(Comparison::field("username").equals_str("MichelDu93"))
    ///     .and(Comparison::field("age").greater_than(10));
    ///
    /// User::exists(query, &db_pool).await;
    /// ```
    pub async fn exists(query: Query, db_pool: &DatabaseConnectionPool) -> bool {
        let aql_string = query.render();
        let aql_query = AqlQuery::builder().query(&aql_string).batch_size(1).count(true).build();
        match db_pool.database.aql_query_batch::<Value>(aql_query).await {
            Ok(cursor) => match cursor.count {
                Some(count) => count > 0,
                None => false
            },
            Err(_error) => false
        }
    }

    /// Creates a document in database
    /// The function will write a new document and return a database record containing the newly created key
    ///
    /// # Arguments
    ///
    /// * `record` - The document to create, it will be returned exactly as the `DatabaseRecord<T>` record
    /// * `db_pool` - database connection pool reference
    ///
    /// # Returns
    ///
    /// On success a new instance of `Self` is returned, with the `key` value filled and `record` filled with the
    /// argument value
    /// On failure a [`ServiceError`] is returned:
    /// * [`UnprocessableEntity`] on data corruption
    ///
    /// [`ServiceError`]: enum.ServiceError.html
    /// [`UnprocessableEntity`]: enum.ServiceError.html#variant.UnprocessableEntity
    pub async fn create(record: T, db_pool: &DatabaseConnectionPool) -> Result<Self, ServiceError> where T: Validate {
        record.validate()?;
        database_service::create_record(record, &db_pool, T::collection_name()).await
    }

    /// Builds a DatabaseRecord from a arangors crate `DocumentResponse<T>`
    /// It will return the filled `DatabaseRecord` on success or will return
    /// a [`ServiceError`]::[`UnprocessableEntity`] on failure
    ///
    /// [`ServiceError`]: enum.ServiceError.html
    /// [`UnprocessableEntity`]: enum.ServiceError.html#variant.UnprocessableEntity
    pub fn from(doc_response: DocumentResponse<T>) -> Result<Self, ServiceError> {
        let header = match doc_response.header() {
            Some(value) => { value }
            None => { return Err(ServiceError::UnprocessableEntity); }
        };
        let doc: T = match doc_response.new_doc() {
            Some(value) => { (*value).clone() }
            None => { return Err(ServiceError::UnprocessableEntity); }
        };
        Ok(DatabaseRecord {
            key: header._key.clone(),
            record: doc,
        })
    }

    /// Authenticates the instance.
    /// The method is available if `T` implements [`Authenticate`] and will simply call
    /// the [`authenticate method`] on the `record`
    ///
    /// # Arguments
    ///
    /// * `password` - the value supposed to validate authentication, password or secret
    ///
    /// # Returns
    ///
    /// On success `()` is returned, on failure it will return a [`ServiceError`] according to
    /// the Authenticate implementation
    ///
    /// [`ServiceError`]: enum.ServiceError.html
    /// [`Authenticate`]: trait.Authenticate.html
    /// [`authenticate method`]: trait.Authenticate.html#tymethod.authenticate
    pub fn authenticate(&self, password: &str) -> Result<(), ServiceError> where T: Authenticate {
        self.record.authenticate(password)
    }
}