use arangors::document::response::DocumentResponse;
use arangors::{AqlQuery, Document};
use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json::Value;
use std::fmt::{self, Display, Formatter};

use crate::db::database_service;
use crate::query::{Query, RecordQueryResult};
use crate::{DatabaseAccess, EdgeRecord, Record, ServiceError};
use std::ops::{Deref, DerefMut};

/// Struct representing database stored documents.
///
/// The document of type `T` mut implement [`Record`]
///
/// # Note
///
/// `DatabaseRecord` implements `Deref` and `DerefMut` into `T`
///
/// [`Record`]: trait.Record.html
#[derive(Debug, Clone)]
pub struct DatabaseRecord<T: Record> {
    /// The Document unique and indexed `_key`
    pub(crate) key: String,
    /// The Document unique and indexed `_id`
    pub(crate) id: String,
    /// The Document revision `_rev`
    pub(crate) rev: String,
    /// The deserialized stored document
    pub record: T,
}

#[allow(dead_code)]
impl<T: Record> DatabaseRecord<T> {
    /// Creates a document in database.
    /// The function will write a new document and return a database record containing the newly created key
    ///
    /// # Hooks
    ///
    /// This function will launch `T` hooks `before_create` and `after_create`.
    ///
    /// # Arguments
    ///
    /// * `record` - The document to create, it will be returned exactly as the `DatabaseRecord<T>` record
    /// * `db_accessor` - database connection pool reference
    ///
    /// # Returns
    ///
    /// On success a new instance of `Self` is returned, with the `key` value filled and `record` filled with the
    /// argument value
    /// On failure a [`ServiceError`] is returned:
    /// * [`UnprocessableEntity`] on data corruption
    /// * Any error returned by hooks
    ///
    /// [`ServiceError`]: enum.ServiceError.html
    /// [`UnprocessableEntity`]: enum.ServiceError.html#variant.UnprocessableEntity
    #[maybe_async::maybe_async]
    pub async fn create<D>(mut record: T, db_accessor: &D) -> Result<Self, ServiceError>
    where
        D: DatabaseAccess,
    {
        record.before_create_hook(db_accessor).await?;
        let mut res =
            database_service::create_record(record, db_accessor, T::collection_name()).await?;
        res.record.after_create_hook(db_accessor).await?;
        Ok(res)
    }

    /// Creates a document in database skipping hooks.
    /// The function will write a new document and return a database record containing the newly created key.
    ///
    /// # Hooks
    ///
    /// This function will skip all hooks.
    ///
    /// # Arguments
    ///
    /// * `record` - The document to create, it will be returned exactly as the `DatabaseRecord<T>` record
    /// * `db_accessor` - database connection pool reference
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
    #[maybe_async::maybe_async]
    pub async fn force_create<D>(record: T, db_accessor: &D) -> Result<Self, ServiceError>
    where
        D: DatabaseAccess,
    {
        database_service::create_record(record, db_accessor, T::collection_name()).await
    }

    /// Writes in the database the new state of the record, "saving it".
    ///
    /// # Hooks
    ///
    /// This function will launch `T` hooks `before_save` and `after_save`.
    ///
    /// # Arguments:
    ///
    /// * `db_accessor` - database connection pool reference
    ///
    /// # Returns
    ///
    /// On success `()` is returned, meaning that the current instance is up to date with the database state.
    /// On failure a [`ServiceError`] is returned:
    /// * [`Conflict`] on index uniqueness conflict
    /// * [`UnprocessableEntity`] on data corruption
    /// * Any error returned by hooks
    ///
    /// [`ServiceError`]: enum.ServiceError.html
    /// [`Conflict`]: enum.ServiceError.html#variant.Conflict
    /// [`UnprocessableEntity`]: enum.ServiceError.html#variant.UnprocessableEntity
    #[maybe_async::maybe_async]
    pub async fn save<D>(&mut self, db_accessor: &D) -> Result<(), ServiceError>
    where
        D: DatabaseAccess,
    {
        self.record.before_save_hook(db_accessor).await?;
        let new_record = database_service::update_record(
            self.record.clone(),
            &self.key,
            db_accessor,
            T::collection_name(),
        )
        .await?;
        self.record.after_save_hook(db_accessor).await?;
        self.record = new_record.record;
        Ok(())
    }

    /// Writes in the database the new state of the record, skipping hooks.
    ///
    /// # Hooks
    ///
    /// This function will skip all hooks.
    ///
    /// # Arguments:
    ///
    /// * `db_accessor` - database connection pool reference
    ///
    /// # Returns
    ///
    /// On success `()` is returned, meaning that the current instance is up to date with the database state.
    /// On failure a [`ServiceError`] is returned:
    /// * [`Conflict`] on index uniqueness conflict
    /// * [`UnprocessableEntity`] on data corruption
    ///
    /// [`ServiceError`]: enum.ServiceError.html
    /// [`Conflict`]: enum.ServiceError.html#variant.Conflict
    /// [`UnprocessableEntity`]: enum.ServiceError.html#variant.UnprocessableEntity
    #[maybe_async::maybe_async]
    pub async fn force_save<D>(&mut self, db_accessor: &D) -> Result<(), ServiceError>
    where
        D: DatabaseAccess,
    {
        let new_record = database_service::update_record(
            self.record.clone(),
            &self.key,
            db_accessor,
            T::collection_name(),
        )
        .await?;
        self.record = new_record.record;
        Ok(())
    }

    /// Removes the record from the database.
    /// The structure won't be freed or emptied but the document won't exist in the global state
    ///
    /// # Hooks
    ///
    /// This function will launch `T` hooks  `before_delete` and `after_delete`.
    ///
    /// # Arguments:
    ///
    /// * `db_accessor` - database connection pool reference
    ///
    /// # Returns
    ///
    /// On success `()` is returned, meaning that the record is now deleted, the structure should not be used afterwards.
    /// On failure a [`ServiceError`] is returned:
    /// * [`NotFound`] on invalid document key
    /// * [`UnprocessableEntity`] on data corruption
    /// * Any error returned by hooks
    ///
    /// [`ServiceError`]: enum.ServiceError.html
    /// [`NotFound`]: enum.ServiceError.html#variant.NotFound
    /// [`UnprocessableEntity`]: enum.ServiceError.html#variant.UnprocessableEntity
    #[maybe_async::maybe_async]
    pub async fn delete<D>(&mut self, db_accessor: &D) -> Result<(), ServiceError>
    where
        D: DatabaseAccess,
    {
        self.record.before_delete_hook(db_accessor).await?;
        database_service::remove_record::<T, D>(&self.key, db_accessor, T::collection_name())
            .await?;
        self.record.after_delete_hook(db_accessor).await?;
        Ok(())
    }

    /// Removes the record from the database, skipping hooks.
    /// The structure won't be freed or emptied but the document won't exist in the global state
    ///
    /// # Hooks
    ///
    /// This function will skip all hooks.
    ///
    /// # Arguments:
    ///
    /// * `db_accessor` - database connection pool reference
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
    #[maybe_async::maybe_async]
    pub async fn force_delete<D>(&self, db_accessor: &D) -> Result<(), ServiceError>
    where
        D: DatabaseAccess,
    {
        database_service::remove_record::<T, D>(&self.key, db_accessor, T::collection_name()).await
    }

    /// Creates and returns edge between `from_record` and `target_record`.
    ///
    /// # Hooks
    ///
    /// This function will launch `T` hooks `before_create` and `after_create`.
    ///
    /// # Example
    /// ```rust no_run
    /// # use aragog::{DatabaseRecord, EdgeRecord, Record, DatabaseConnectionPool};
    /// # use serde::{Serialize, Deserialize};
    /// #
    /// # #[derive(Record, Clone, Serialize, Deserialize)]
    /// # struct User {}
    /// #[derive(Clone, EdgeRecord, Record, Serialize, Deserialize)]
    /// struct Edge {
    ///     _from: String,
    ///     _to: String,
    ///     description: String,
    /// }
    /// # #[tokio::main]
    /// # async fn main() {
    /// # let db_accessor = DatabaseConnectionPool::builder().build().await.unwrap();
    /// let record_a = User::find("123", &db_accessor).await.unwrap();
    /// let record_b = User::find("234", &db_accessor).await.unwrap();
    ///
    /// let edge = DatabaseRecord::link(&record_a, &record_b, &db_accessor, |_from, _to| {
    ///     Edge { _from, _to, description: "description".to_string() }
    /// }).await.unwrap();
    /// # }
    /// ```
    #[maybe_async::maybe_async]
    pub async fn link<A, B, D, E>(
        from_record: &DatabaseRecord<A>,
        to_record: &DatabaseRecord<B>,
        db_accessor: &D,
        edge_record: E,
    ) -> Result<DatabaseRecord<T>, ServiceError>
    where
        A: Serialize + DeserializeOwned + Clone + Record,
        B: Serialize + DeserializeOwned + Clone + Record,
        D: DatabaseAccess,
        E: FnOnce(String, String) -> T,
        T: EdgeRecord,
    {
        let edge = edge_record(from_record.id.clone(), to_record.id.clone());
        DatabaseRecord::create(edge, db_accessor).await
    }

    /// Retrieves a record from the database with the associated unique `key`
    ///
    /// # Arguments:
    ///
    /// * `key` - the unique record key as a string slice
    /// * `db_accessor` - database connection pool reference
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
    #[maybe_async::maybe_async]
    pub async fn find<D>(key: &str, db_accessor: &D) -> Result<Self, ServiceError>
    where
        D: DatabaseAccess,
    {
        database_service::retrieve_record(key, db_accessor, T::collection_name()).await
    }

    /// Reloads a record from the database, returning the new record.
    ///
    /// # Arguments
    ///
    /// * `db_accessor` - database connection pool reference
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
    #[maybe_async::maybe_async]
    pub async fn reload<D>(self, db_accessor: &D) -> Result<Self, ServiceError>
    where
        D: DatabaseAccess,
        T: Send,
    {
        T::find(&self.key, db_accessor).await
    }

    /// Reloads a record from the database.
    ///
    /// # Returns
    ///
    /// On success `()` is returned and `self` is updated,
    /// On failure a [`ServiceError`] is returned:
    /// * [`NotFound`] on invalid document key
    /// * [`UnprocessableEntity`] on data corruption
    ///
    /// [`ServiceError`]: enum.ServiceError.html
    /// [`NotFound`]: enum.ServiceError.html#variant.NotFound
    /// [`UnprocessableEntity`]: enum.ServiceError.html#variant.UnprocessableEntity
    #[maybe_async::maybe_async]
    pub async fn reload_mut<D>(&mut self, db_accessor: &D) -> Result<(), ServiceError>
    where
        D: DatabaseAccess,
        T: Send,
    {
        *self = T::find(&self.key, db_accessor).await?;
        Ok(())
    }

    /// Retrieves all records from the database matching the associated conditions.
    ///
    /// # Arguments:
    ///
    /// * `query` - The `Query` to match
    /// * `db_accessor` - database connection pool reference
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
    /// ```rust no_run
    /// # use aragog::query::{Comparison, Filter};
    /// # use serde::{Serialize, Deserialize};
    /// # use aragog::{DatabaseConnectionPool, Record, DatabaseRecord};
    /// #
    /// # #[derive(Record, Clone, Serialize, Deserialize)]
    /// # struct User {}
    /// #
    /// # #[tokio::main]
    /// # async fn main() {
    /// # let db_accessor = DatabaseConnectionPool::builder().build().await.unwrap();
    /// let query = User::query().filter(Filter::new(Comparison::field("username").equals_str("MichelDu93"))
    ///     .and(Comparison::field("age").greater_than(10)));
    ///
    /// // Both lines are equivalent:
    /// DatabaseRecord::<User>::get(query.clone(), &db_accessor).await.unwrap();
    /// User::get(query.clone(), &db_accessor).await.unwrap();
    /// # }
    /// ```
    ///
    /// [`ServiceError`]: enum.ServiceError.html
    /// [`NotFound`]: enum.ServiceError.html#variant.NotFound
    /// [`UnprocessableEntity`]: enum.ServiceError.html#variant.UnprocessableEntity
    #[maybe_async::maybe_async]
    pub async fn get<D>(query: Query, db_accessor: &D) -> Result<RecordQueryResult<T>, ServiceError>
    where
        D: DatabaseAccess,
    {
        Self::aql_get(&query.to_aql(), db_accessor).await
    }

    /// Retrieves all records from the database matching the associated conditions.
    ///
    /// # Arguments:
    ///
    /// * `query` - The AQL request string
    /// * `db_accessor` - database connection pool reference
    ///
    /// # Returns
    ///
    /// On success a `QueryResult` with a vector of `Self` is returned. It is can be empty.
    /// On failure a [`ServiceError`] is returned:
    /// * [`NotFound`] if no document matches the condition
    /// * [`UnprocessableEntity`] on data corruption
    ///
    /// # Warning
    ///
    /// If you call this method on a graph query only the documents that can be serialized into `T` will be returned.
    ///
    /// # Example
    ///
    /// ```rust no_run
    /// # use serde::{Serialize, Deserialize};
    /// # use aragog::{DatabaseConnectionPool, Record, DatabaseRecord};
    /// #
    /// # #[derive(Record, Clone, Serialize, Deserialize)]
    /// # struct User {}
    /// #
    /// # #[tokio::main]
    /// # async fn main() {
    /// # let db_accessor = DatabaseConnectionPool::builder().build().await.unwrap();
    /// let query = r#"FOR i in User FILTER i.username == "MichelDu93" && i.age > 10 return i"#;
    ///
    /// DatabaseRecord::<User>::aql_get(query, &db_accessor).await.unwrap();
    /// # }
    /// ```
    ///
    /// [`ServiceError`]: enum.ServiceError.html
    /// [`NotFound`]: enum.ServiceError.html#variant.NotFound
    /// [`UnprocessableEntity`]: enum.ServiceError.html#variant.UnprocessableEntity
    #[maybe_async::maybe_async]
    pub async fn aql_get<D>(
        query: &str,
        db_accessor: &D,
    ) -> Result<RecordQueryResult<T>, ServiceError>
    where
        D: DatabaseAccess,
    {
        let result = db_accessor.aql_get(query).await?;
        Ok(result.into())
    }

    /// Creates a new outbound graph `Query` with `self` as a start vertex
    ///
    /// # Arguments
    ///
    /// * `edge_collection`- The name of the queried edge collection
    /// * `min` - The minimum depth of the graph request
    /// * `max` - The maximum depth of the graph request
    ///
    /// # Example
    /// ```rust no_run
    /// # use serde::{Serialize, Deserialize};
    /// # use aragog::query::Query;
    /// # use aragog::{DatabaseConnectionPool, Record};
    /// #
    /// # #[derive(Record, Clone, Serialize, Deserialize)]
    /// # struct User {}
    /// #
    /// # #[tokio::main]
    /// # async fn main() {
    /// # let db_accessor = DatabaseConnectionPool::builder().build().await.unwrap();
    /// let record = User::find("123", &db_accessor).await.unwrap();
    /// // Both statements are equivalent
    /// let q = record.outbound_query(1, 2, "ChildOf");
    /// let q = Query::outbound(1, 2, "ChildOf", record.id());
    /// # }
    /// ```
    pub fn outbound_query(&self, min: u16, max: u16, edge_collection: &str) -> Query {
        Query::outbound(min, max, edge_collection, &self.id)
    }

    /// Creates a new inbound graph `Query` with `self` as a start vertex
    ///
    /// # Arguments
    ///
    /// * `edge_collection`- The name of the queried edge collection
    /// * `min` - The minimum depth of the graph request
    /// * `max` - The maximum depth of the graph request
    ///
    /// # Example
    /// ```rust no_run
    /// # use serde::{Serialize, Deserialize};
    /// # use aragog::query::Query;
    /// # use aragog::{DatabaseConnectionPool, Record};
    /// #
    /// # #[derive(Record, Clone, Serialize, Deserialize)]
    /// # struct User {}
    /// #
    /// # #[tokio::main]
    /// # async fn main() {
    /// # let db_accessor = DatabaseConnectionPool::builder().build().await.unwrap();
    /// #
    /// let record = User::find("123", &db_accessor).await.unwrap();
    /// // Both statements are equivalent
    /// let q = record.inbound_query(1, 2, "ChildOf");
    /// let q = Query::inbound(1, 2, "ChildOf", record.id());
    /// # }
    /// ```
    pub fn inbound_query(&self, min: u16, max: u16, edge_collection: &str) -> Query {
        Query::inbound(min, max, edge_collection, &self.id)
    }

    /// Creates a new outbound graph `Query` with `self` as a start vertex
    ///
    /// # Arguments
    ///
    /// * `min` - The minimum depth of the graph request
    /// * `max` - The maximum depth of the graph request
    /// * `named_graph`- The named graph to traverse
    ///
    /// # Example
    /// ```rust no_run
    /// # use serde::{Serialize, Deserialize};
    /// # use aragog::query::Query;
    /// # use aragog::{DatabaseConnectionPool, Record};
    /// #
    /// # #[derive(Record, Clone, Serialize, Deserialize)]
    /// # struct User {}
    /// #
    /// # #[tokio::main]
    /// # async fn main() {
    /// # let db_accessor = DatabaseConnectionPool::builder().build().await.unwrap();
    /// let record = User::find("123", &db_accessor).await.unwrap();
    /// // Both statements are equivalent
    /// let q = record.outbound_graph(1, 2, "SomeGraph");
    /// let q = Query::outbound_graph(1, 2, "SomeGraph", record.id());
    /// # }
    /// ```
    pub fn outbound_graph(&self, min: u16, max: u16, named_graph: &str) -> Query {
        Query::outbound_graph(min, max, named_graph, &self.id)
    }

    /// Creates a new inbound graph `Query` with `self` as a start vertex
    ///
    /// # Arguments
    ///
    /// * `min` - The minimum depth of the graph request
    /// * `max` - The maximum depth of the graph request
    /// * `named_graph`- The named graph to traverse
    ///
    /// # Example
    /// ```rust no_run
    /// # use serde::{Serialize, Deserialize};
    /// # use aragog::query::Query;
    /// # use aragog::{DatabaseConnectionPool, Record};
    /// #
    /// # #[derive(Record, Clone, Serialize, Deserialize)]
    /// # struct User {}
    /// #
    /// # #[tokio::main]
    /// # async fn main() {
    /// # let db_accessor = DatabaseConnectionPool::builder().build().await.unwrap();
    /// let record = User::find("123", &db_accessor).await.unwrap();
    /// // Both statements are equivalent
    /// let q = record.inbound_graph(1, 2, "SomeGraph");
    /// let q = Query::inbound_graph(1, 2, "SomeGraph", record.id());
    /// # }
    /// ```
    pub fn inbound_graph(&self, min: u16, max: u16, named_graph: &str) -> Query {
        Query::inbound_graph(min, max, named_graph, &self.id)
    }

    /// Checks if any document matching the associated conditions exist
    ///
    /// # Arguments:
    ///
    /// * `query` - The `Query` to match
    /// * `db_accessor` - database connection pool reference
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
    /// ```rust no_run
    /// # use serde::{Serialize, Deserialize};
    /// # use aragog::{DatabaseConnectionPool, Record};
    /// # use aragog::query::{Query, Comparison, Filter};
    /// #
    /// # #[derive(Record, Clone, Serialize, Deserialize)]
    /// # struct User {}
    /// #
    /// # #[tokio::main]
    /// # async fn main() {
    /// # let db_accessor = DatabaseConnectionPool::builder().build().await.unwrap();
    /// let query = User::query().filter(
    ///     Filter::new(Comparison::field("username").equals_str("MichelDu93"))
    ///         .and(Comparison::field("age").greater_than(10)));
    /// User::exists(query, &db_accessor).await;
    /// # }
    /// ```
    #[maybe_async::maybe_async]
    pub async fn exists<D>(query: Query, db_accessor: &D) -> bool
    where
        D: DatabaseAccess,
    {
        let aql_string = query.to_aql();
        let aql_query = AqlQuery::builder()
            .query(&aql_string)
            .batch_size(1)
            .count(true)
            .build();
        match db_accessor
            .database()
            .aql_query_batch::<Value>(aql_query)
            .await
        {
            Ok(cursor) => match cursor.count {
                Some(count) => count > 0,
                None => false,
            },
            Err(_error) => false,
        }
    }

    /// Builds a DatabaseRecord from a arangors crate `DocumentResponse<T>`
    /// It will return the filled `DatabaseRecord` on success or will return
    /// a [`ServiceError`]::[`UnprocessableEntity`] on failure
    ///
    /// [`ServiceError`]: enum.ServiceError.html
    /// [`UnprocessableEntity`]: enum.ServiceError.html#variant.UnprocessableEntity
    pub(crate) fn from_response(doc_response: DocumentResponse<T>) -> Result<Self, ServiceError> {
        let header = match doc_response.header() {
            Some(value) => value,
            None => {
                return Err(ServiceError::UnprocessableEntity);
            }
        };
        let doc: T = match doc_response.new_doc() {
            Some(value) => (*value).clone(),
            None => {
                return Err(ServiceError::UnprocessableEntity);
            }
        };
        Ok(DatabaseRecord {
            key: header._key.clone(),
            id: header._id.clone(),
            rev: header._rev.clone(),
            record: doc,
        })
    }

    /// Getter for the Document `_id` built as `$collection_name/$_key
    pub fn id(&self) -> &String {
        &self.id
    }

    /// Getter for the Document `_key`
    pub fn key(&self) -> &String {
        &self.key
    }

    /// Getter for the Document `_rev`
    pub fn rev(&self) -> &String {
        &self.rev
    }
}

impl<T: Record> From<Document<T>> for DatabaseRecord<T> {
    fn from(doc: Document<T>) -> Self {
        Self {
            key: doc.header._key,
            id: doc.header._id,
            rev: doc.header._rev,
            record: doc.document,
        }
    }
}

impl<T: Record> Display for DatabaseRecord<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} Database Record", T::collection_name(), self.key)
    }
}

impl<T: Record> Deref for DatabaseRecord<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.record
    }
}

impl<T: Record> DerefMut for DatabaseRecord<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.record
    }
}
