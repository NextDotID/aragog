use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::query::{Query, RecordQueryResult};
use crate::{DatabaseAccess, DatabaseRecord, ServiceError};

/// The main trait of the Aragog library.
/// Trait for structures that can be stored in Database.
/// The trait must be implemented to be used as a record in [`DatabaseRecord`]
///
/// [`DatabaseRecord`]: struct.DatabaseRecord.html
#[maybe_async::maybe_async]
pub trait Record: DeserializeOwned + Serialize + Clone {
    /// Finds a document in database from its unique key.
    /// Simple wrapper for [`DatabaseRecord`]<`T`>::[`find`]
    ///
    /// [`DatabaseRecord`]: struct.DatabaseRecord.html
    /// [`find`]: struct.DatabaseRecord.html#method.find
    async fn find<D>(key: &str, db_pool: &D) -> Result<DatabaseRecord<Self>, ServiceError>
    where
        D: DatabaseAccess,
    {
        DatabaseRecord::find(key, db_pool).await
    }

    /// Finds all documents in database matching a `Query`.
    /// Simple wrapper for [`DatabaseRecord`]<`T`>::[`get`]
    ///
    /// [`DatabaseRecord`]: struct.DatabaseRecord.html
    /// [`get`]: struct.DatabaseRecord.html#method.get
    async fn get<D>(query: Query, db_pool: &D) -> Result<RecordQueryResult<Self>, ServiceError>
    where
        D: DatabaseAccess,
    {
        DatabaseRecord::get(query, db_pool).await
    }

    /// Returns true if there are any document in database matching a `Query`.
    /// Simple wrapper for [`DatabaseRecord`]<`T`>::[`exists`]
    ///
    /// [`DatabaseRecord`]: struct.DatabaseRecord.html
    /// [`exists`]: struct.DatabaseRecord.html#method.exists
    async fn exists<D>(query: Query, db_pool: &D) -> bool
    where
        D: DatabaseAccess,
    {
        DatabaseRecord::<Self>::exists(query, db_pool).await
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
    /// # use serde::{Serialize, Deserialize};
    /// #[derive(Record, Clone, Serialize, Deserialize)]
    /// pub struct User { }
    ///
    /// // All three statements are equivalent:
    /// let q = User::query();
    /// let q = Query::new(User::collection_name());
    /// let q = Query::new("User");
    /// ```
    fn query() -> Query {
        Query::new(Self::collection_name())
    }
}
