use arangors::{ClientError, Document};
use arangors::document::options::{InsertOptions, RemoveOptions, UpdateOptions};
use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::{Record, DatabaseConnectionPool, AragornServiceError, DatabaseRecord};

pub async fn update_record<T: DeserializeOwned + Serialize + Clone + Record>(obj: T, key: &str, db_pool: &DatabaseConnectionPool, collection_name: &str) -> Result<DatabaseRecord<T>, AragornServiceError> {
    let collection = db_pool.get_collection(collection_name);
    // log::debug!("Trying to update a document in {:?}: {}", model, &db_pool.collections[model].collection_name);
    let response = match collection.update_document(
        key,
        obj,
        UpdateOptions::builder()
            //.wait_for_sync(true)
            .return_new(true)
            .build(),
    ).await {
        Ok(resp) => { resp }
        Err(error) => {
            log::error!("{}", error);
            return match error {
                ClientError::Arango(arango_error) => {
                    if arango_error.code() == 409 {
                        Err(AragornServiceError::Conflict)
                    } else {
                        Err(AragornServiceError::UnprocessableEntity)
                    }
                }
                _ => Err(AragornServiceError::UnprocessableEntity)
            }
        }
    };
    DatabaseRecord::from(response)
}

pub async fn create_record<T: DeserializeOwned + Serialize + Clone + Record>(obj: T, db_pool: &DatabaseConnectionPool, collection_name: &str) -> Result<DatabaseRecord<T>, AragornServiceError> {
    let collection = db_pool.get_collection(collection_name);
    // log::debug!("Trying to create a document in {:?}: {}", model, &db_pool.collections[model].collection_name);
    let response = match collection.create_document(
        obj,
        InsertOptions::builder()
            // .wait_for_sync(true)
            .return_new(true)
            .build(),
    ).await {
        Ok(resp) => { resp }
        Err(error) => {
            log::error!("{}", error);
            return match error {
                ClientError::Arango(arango_error) => {
                    if arango_error.code() == 409 {
                        Err(AragornServiceError::Conflict)
                    } else {
                        Err(AragornServiceError::UnprocessableEntity)
                    }
                }
                _ => Err(AragornServiceError::UnprocessableEntity)
            }
        }
    };
    DatabaseRecord::from(response)
}

pub async fn retrieve_record<T: Serialize + DeserializeOwned + Clone + Record>(key: &str, db_pool: &DatabaseConnectionPool, collection_name: &str) -> Result<DatabaseRecord<T>, AragornServiceError> {
    let collection = db_pool.get_collection(collection_name);
    let record: Document<T> = match collection.document(key).await {
        Ok(doc) => { doc }
        Err(error) => {
            log::error!("{}", error);
            return Err(AragornServiceError::NotFound(format!("{} {}", collection_name, key)));
        }
    };
    Ok(
        DatabaseRecord {
            key: String::from(key),
            record: record.document,
        }
    )
}

pub async fn remove_record<T: Serialize + DeserializeOwned + Clone + Record>(key: &str, db_pool: &DatabaseConnectionPool, collection_name: &str) -> Result<(), AragornServiceError> {
    let collection = db_pool.get_collection(collection_name);

    match collection.remove_document::<T>(
        key,
        RemoveOptions::builder()
            //.wait_for_sync(true)
            .build(),
        None,
    ).await {
        Ok(_result) => { Ok(()) }
        Err(error) => {
            log::error!("{}", error);
            Err(AragornServiceError::NotFound(format!("{} {}", collection_name, key)))
        }
    }
}