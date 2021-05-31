use crate::{DatabaseAccess, Record, ServiceError};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::ops::{Deref, DerefMut};

/// Wrapper for `serde_json::Value` to be treated as a `Record`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UndefinedRecord(pub Value);

#[maybe_async::maybe_async]
impl Record for UndefinedRecord {
    fn collection_name() -> &'static str {
        "Undefined Collection"
    }

    async fn before_create_hook<D>(&mut self, _db_accessor: &D) -> Result<(), ServiceError>
    where
        D: DatabaseAccess + ?Sized,
    {
        Ok(())
    }

    async fn before_save_hook<D>(&mut self, _db_accessor: &D) -> Result<(), ServiceError>
    where
        D: DatabaseAccess + ?Sized,
    {
        Ok(())
    }

    async fn before_delete_hook<D>(&mut self, _db_accessor: &D) -> Result<(), ServiceError>
    where
        D: DatabaseAccess + ?Sized,
    {
        Ok(())
    }

    async fn after_create_hook<D>(&mut self, _db_accessor: &D) -> Result<(), ServiceError>
    where
        D: DatabaseAccess + ?Sized,
    {
        Ok(())
    }

    async fn after_save_hook<D>(&mut self, _db_accessor: &D) -> Result<(), ServiceError>
    where
        D: DatabaseAccess + ?Sized,
    {
        Ok(())
    }

    async fn after_delete_hook<D>(&mut self, _db_accessor: &D) -> Result<(), ServiceError>
    where
        D: DatabaseAccess + ?Sized,
    {
        Ok(())
    }
}

impl From<Value> for UndefinedRecord {
    fn from(json: Value) -> Self {
        Self(json)
    }
}

impl Deref for UndefinedRecord {
    type Target = Value;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for UndefinedRecord {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
