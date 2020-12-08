use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::query::{Query, RecordQueryResult};
use crate::{DatabaseConnectionPool, DatabaseRecord, ServiceError};

/// The main trait of the Aragog library.
/// Trait for structures that can be stored in Database.
/// The trait must be implemented to be used as a record in [`DatabaseRecord`]
///
/// [`DatabaseRecord`]: db/database_record/struct.DatabaseRecord.html
#[maybe_async::maybe_async]
pub trait Record {
    /// Finds a document in database from its unique key.
    /// Simple wrapper for [`DatabaseRecord`]<`T`>::[`find`]
    ///
    /// [`DatabaseRecord`]: db/database_record/struct.DatabaseRecord.html
    /// [`find`]: db/database_record/struct.DatabaseRecord.html#method.find
    async fn find(
        key: &str,
        db_pool: &DatabaseConnectionPool,
    ) -> Result<DatabaseRecord<Self>, ServiceError>
    where
        Self: DeserializeOwned + Serialize + Clone,
    {
        DatabaseRecord::find(key, &db_pool).await
    }

    /// Finds all documents in database matching a `Query`.
    /// Simple wrapper for [`DatabaseRecord`]<`T`>::[`get`]
    ///
    /// [`DatabaseRecord`]: db/database_record/struct.DatabaseRecord.html
    /// [`get`]: db/database_record/struct.DatabaseRecord.html#method.get
    async fn get(
        query: Query,
        db_pool: &DatabaseConnectionPool,
    ) -> Result<RecordQueryResult<Self>, ServiceError>
    where
        Self: DeserializeOwned + Serialize + Clone,
    {
        DatabaseRecord::get(query, &db_pool).await
    }

    /// Returns true if there are any document in database matching a `Query`.
    /// Simple wrapper for [`DatabaseRecord`]<`T`>::[`exists`]
    ///
    /// [`DatabaseRecord`]: db/database_record/struct.DatabaseRecord.html
    /// [`exists`]: db/database_record/struct.DatabaseRecord.html#method.exists
    async fn exists(query: Query, db_pool: &DatabaseConnectionPool) -> bool
    where
        Self: DeserializeOwned + Serialize + Clone,
    {
        DatabaseRecord::<Self>::exists(query, &db_pool).await
    }

    /// returns the associated Collection
    /// for read and write operations.
    fn collection_name() -> &'static str;

    /// Creates a new `Query` instance for `Self`.
    ///
    /// # Example
    /// ```rust
    /// # use aragog::query::Query;
    /// # use aragog::Record;
    /// #[derive(Record)]
    /// pub struct User { }
    ///
    /// // Both statements are equivalent:
    /// let q = Query::new("User");
    /// let q = User::query();
    /// ```
    fn query() -> Query {
        Query::new(Self::collection_name())
    }
}
