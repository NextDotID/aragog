use arangors::document::options::{InsertOptions, RemoveOptions, UpdateOptions};
use arangors::{Collection, Document};

use crate::error::ArangoHttpError;
use crate::{DatabaseAccess, DatabaseRecord, Record, ServiceError};
use arangors::client::reqwest::ReqwestClient;

#[maybe_async::maybe_async]
pub async fn update_record<T, D>(
    obj: T,
    key: &str,
    db_accessor: &D,
    collection_name: &str,
) -> Result<DatabaseRecord<T>, ServiceError>
where
    T: Record,
    D: DatabaseAccess + ?Sized,
{
    log::debug!("Updating document {} {}", collection_name, key);
    let collection = db_accessor.get_collection(collection_name)?;
    let response = match collection
        .update_document(key, obj, UpdateOptions::builder().return_new(true).build())
        .await
    {
        Ok(resp) => resp,
        Err(error) => {
            log::error!("{}", error);
            return Err(ServiceError::from(error));
        }
    };
    DatabaseRecord::from_response(response)
}

#[maybe_async::maybe_async]
pub async fn create_document<T>(
    obj: T,
    collection: &Collection<ReqwestClient>,
) -> Result<DatabaseRecord<T>, ServiceError>
where
    T: Record,
{
    log::debug!("Creating new {} document", collection.name());
    let response = match collection
        .create_document(obj, InsertOptions::builder().return_new(true).build())
        .await
    {
        Ok(resp) => resp,
        Err(error) => {
            log::error!("{}", error);
            return Err(ServiceError::from(error));
        }
    };
    DatabaseRecord::from_response(response)
}

#[maybe_async::maybe_async]
pub async fn create_record<T, D>(
    obj: T,
    db_accessor: &D,
    collection_name: &str,
) -> Result<DatabaseRecord<T>, ServiceError>
where
    T: Record,
    D: DatabaseAccess + ?Sized,
{
    let collection = db_accessor.get_collection(collection_name)?;
    create_document(obj, collection).await
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
    let record: Document<T> = match collection.document(key).await {
        Ok(doc) => doc,
        Err(error) => {
            log::error!("{}", error);
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
) -> Result<(), ServiceError>
where
    T: Record,
    D: DatabaseAccess + ?Sized,
{
    log::debug!("Removing {} {} from database", collection_name, key);
    let collection = db_accessor.get_collection(collection_name)?;
    match collection
        .remove_document::<T>(key, RemoveOptions::builder().build(), None)
        .await
    {
        Ok(_result) => Ok(()),
        Err(error) => {
            log::error!("{}", error);
            Err(ServiceError::from(error))
        }
    }
}
