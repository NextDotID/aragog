use crate::db::database_record_dto::DatabaseRecordDto;
use crate::error::ArangoHttpError;
use crate::query::{QueryCursor, QueryResult};
use crate::{DatabaseAccess, DatabaseRecord, OperationOptions, Record, ServiceError};
use arangors::{AqlOptions, AqlQuery};
use std::convert::TryInto;

#[maybe_async::maybe_async]
pub async fn update_record<T, D>(
    obj: DatabaseRecord<T>,
    key: &str,
    db_accessor: &D,
    collection_name: &str,
    options: OperationOptions,
) -> Result<DatabaseRecord<T>, ServiceError>
where
    T: Record,
    D: DatabaseAccess + ?Sized,
{
    log::debug!("Updating document {} {}", collection_name, key);
    let collection = db_accessor.get_collection(collection_name)?;
    let response = match collection.update_document(key, obj, options.into()).await {
        Ok(resp) => resp,
        Err(error) => return Err(ServiceError::from(error)),
    };
    response.try_into()
}

#[maybe_async::maybe_async]
pub async fn create_record<T, D>(
    obj: T,
    db_accessor: &D,
    collection_name: &str,
    options: OperationOptions,
) -> Result<DatabaseRecord<T>, ServiceError>
where
    T: Record,
    D: DatabaseAccess + ?Sized,
{
    let collection = db_accessor.get_collection(collection_name)?;
    log::debug!("Creating new {} document", collection.name());
    let dto = DatabaseRecordDto::new(obj);
    let response = match collection.create_document(dto, options.into()).await {
        Ok(resp) => resp,
        Err(error) => return Err(ServiceError::from(error)),
    };
    response.try_into()
}

#[maybe_async::maybe_async]
pub async fn retrieve_record<T, D>(
    key: &str,
    db_accessor: &D,
    collection_name: &str,
) -> Result<DatabaseRecord<T>, ServiceError>
where
    T: Record,
    D: DatabaseAccess + ?Sized,
{
    log::debug!("Retrieving {} {} from database", collection_name, key);
    let collection = db_accessor.get_collection(collection_name)?;
    let record = match collection.document(key).await {
        Ok(doc) => doc,
        Err(error) => {
            let err = ServiceError::from(error);
            if let ServiceError::ArangoError(ref db_error) = err {
                if let ArangoHttpError::NotFound = db_error.http_error {
                    return Err(ServiceError::NotFound {
                        item: collection_name.to_string(),
                        id: key.to_string(),
                        source: Some(db_error.clone()),
                    });
                }
            }
            return Err(err);
        }
    };
    Ok(DatabaseRecord::from(record))
}

#[maybe_async::maybe_async]
pub async fn remove_record<T, D>(
    key: &str,
    db_accessor: &D,
    collection_name: &str,
    options: OperationOptions,
) -> Result<(), ServiceError>
where
    T: Record,
    D: DatabaseAccess + ?Sized,
{
    log::debug!("Removing {} {} from database", collection_name, key);
    let collection = db_accessor.get_collection(collection_name)?;
    match collection
        .remove_document::<T>(key, options.into(), None)
        .await
    {
        Ok(_result) => Ok(()),
        Err(error) => Err(ServiceError::from(error)),
    }
}

#[maybe_async::maybe_async]
pub async fn query_records<T, D>(db_accessor: &D, aql: &str) -> Result<QueryResult<T>, ServiceError>
where
    T: Record,
    D: DatabaseAccess + ?Sized,
{
    log::debug!(
        "Querying {} records through AQL: `{}`",
        T::COLLECTION_NAME,
        aql
    );
    let query_result = match db_accessor.database().aql_str(aql).await {
        Ok(value) => value,
        Err(error) => return Err(ServiceError::from(error)),
    };
    Ok(query_result.into())
}

#[maybe_async::maybe_async]
pub async fn query_records_in_batches<T, D>(
    db_accessor: &D,
    aql: &str,
    batch_size: u32,
) -> Result<QueryCursor<T>, ServiceError>
where
    T: Record,
    D: DatabaseAccess + ?Sized,
{
    log::debug!(
        "Querying {} records through AQL with {} batch size: `{}`",
        T::COLLECTION_NAME,
        batch_size,
        aql
    );
    let query = AqlQuery::builder()
        .query(aql)
        .count(true)
        .batch_size(batch_size)
        .options(AqlOptions::builder().full_count(true).build())
        .build();
    let cursor = match db_accessor.database().aql_query_batch(query).await {
        Ok(value) => value,
        Err(error) => return Err(ServiceError::from(error)),
    };
    Ok(QueryCursor::new(
        cursor,
        db_accessor.database().clone(),
        aql.to_string(),
    ))
}
