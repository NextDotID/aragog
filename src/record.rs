use async_trait::async_trait;
use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::{ServiceError, DatabaseConnectionPool, DatabaseRecord};

/// The main trait of the Aragog library.
/// Trait for structures that can be stored in Database.
/// The trait must be implemented to be used as a record in [`DatabaseRecord`]
///
/// [`DatabaseRecord`]: db/database_record/struct.DatabaseRecord.html
#[async_trait]
pub trait Record {
    /// Finds a document in database from its unique key.
    /// Simple wrapper for [`DatabaseRecord`]<`T`>::[`find`]
    ///
    /// [`DatabaseRecord`]: db/database_record/struct.DatabaseRecord.html
    /// [`find`]: db/database_record/struct.DatabaseRecord.html#method.find
    async fn find(key: &str, db_pool: &DatabaseConnectionPool)
                  -> Result<DatabaseRecord<Self>, ServiceError>
        where Self: DeserializeOwned + Serialize + Clone {
        DatabaseRecord::find(key, &db_pool).await
    }

    /// Finds a document in database matching a specific condition.
    /// Simple wrapper for [`DatabaseRecord`]<`T`>::[`find_by`]
    ///
    /// [`DatabaseRecord`]: db/database_record/struct.DatabaseRecord.html
    /// [`find_by`]: db/database_record/struct.DatabaseRecord.html#method.find_by
    async fn find_by(condition: &str, db_pool: &DatabaseConnectionPool)
                     -> Result<DatabaseRecord<Self>, ServiceError>
        where Self: DeserializeOwned + Serialize + Clone {
        DatabaseRecord::find_by(condition, &db_pool).await
    }

    /// Finds a document in database matching a Vec of conditions.
    /// Simple wrapper for [`DatabaseRecord`]<`T`>::[`find_where`]
    ///
    /// [`DatabaseRecord`]: db/database_record/struct.DatabaseRecord.html
    /// [`find_where`]: db/database_record/struct.DatabaseRecord.html#method.find_where
    async fn find_where(conditions: Vec<&str>, db_pool: &DatabaseConnectionPool)
                           -> Result<DatabaseRecord<Self>, ServiceError>
        where Self: DeserializeOwned + Serialize + Clone {
        DatabaseRecord::find_where(conditions, &db_pool).await
    }

    /// Finds all documents in database matching a Vec of conditions.
    /// Simple wrapper for [`DatabaseRecord`]<`T`>::[`get_where`]
    ///
    /// [`DatabaseRecord`]: db/database_record/struct.DatabaseRecord.html
    /// [`get_where`]: db/database_record/struct.DatabaseRecord.html#method.get_where
    async fn get_where(conditions: Vec<&str>, db_pool: &DatabaseConnectionPool)
                          -> Result<Vec<DatabaseRecord<Self>>, ServiceError>
        where Self: DeserializeOwned + Serialize + Clone {
        DatabaseRecord::get_where(conditions, &db_pool).await
    }

    /// Returns true if there are any document in database matching a Vec of conditions.
    /// Simple wrapper for [`DatabaseRecord`]<`T`>::[`exists_where`]
    ///
    /// [`DatabaseRecord`]: db/database_record/struct.DatabaseRecord.html
    /// [`exists_where`]: db/database_record/struct.DatabaseRecord.html#method.exists_where
    async fn exists_where(conditions: Vec<&str>, db_pool: &DatabaseConnectionPool) -> bool
        where Self: DeserializeOwned + Serialize + Clone
    {
        DatabaseRecord::<Self>::exists_where(conditions, &db_pool).await
    }

    /// returns the associated Collection
    /// for read and write operations.
    fn collection_name() -> &'static str;
}