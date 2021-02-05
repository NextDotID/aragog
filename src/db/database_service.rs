use arangors::document::options::{InsertOptions, RemoveOptions, UpdateOptions};
use arangors::{Collection, Document};

use crate::{DatabaseConnectionPool, DatabaseRecord, Record, ServiceError};
use arangors::client::reqwest::ReqwestClient;

#[maybe_async::maybe_async]
pub async fn update_record<T: Record>(
    obj: T,
    key: &str,
    db_pool: &DatabaseConnectionPool,
    collection_name: &str,
) -> Result<DatabaseRecord<T>, ServiceError> {
    log::debug!("Updating document {} {}", collection_name, key);
    let collection = db_pool.get_collection(collection_name);
    // log::debug!("Trying to update a document in {:?}: {}", model, &db_pool.collections[model].collection_name);
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
pub async fn create_document<T: Record>(
    obj: T,
    collection: &Collection<ReqwestClient>,
) -> Result<DatabaseRecord<T>, ServiceError> {
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
pub async fn create_record<T: Record>(
    obj: T,
    db_pool: &DatabaseConnectionPool,
    collection_name: &str,
) -> Result<DatabaseRecord<T>, ServiceError> {
    let collection = db_pool.get_collection(collection_name);
    create_document(obj, collection).await
}

#[maybe_async::maybe_async]
pub async fn retrieve_record<T: Record>(
    key: &str,
    db_pool: &DatabaseConnectionPool,
    collection_name: &str,
) -> Result<DatabaseRecord<T>, ServiceError> {
    log::debug!("Retrieving {} {} from database", collection_name, key);
    let collection = db_pool.get_collection(collection_name);
    let record: Document<T> = match collection.document(key).await {
        Ok(doc) => doc,
        Err(error) => {
            log::error!("{}", error);
            let err = ServiceError::from(error);
            if let ServiceError::NotFound(_str) = err {
                return Err(ServiceError::NotFound(format!(
                    "{} document not found",
                    collection_name
                )));
            }
            return Err(err);
        }
    };
    Ok(DatabaseRecord::from(record))
}

#[maybe_async::maybe_async]
pub async fn remove_record<T: Record>(
    key: &str,
    db_pool: &DatabaseConnectionPool,
    collection_name: &str,
) -> Result<(), ServiceError> {
    log::debug!("Removing {} {} from database", collection_name, key);
    let collection = db_pool.get_collection(collection_name);
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
