extern crate aragog;

use aragog::error::{ArangoError, ArangoHttpError};
use aragog::{DatabaseRecord, Error, OperationOptions, Record};
use serde::{Deserialize, Serialize};

pub mod common;

#[derive(Serialize, Deserialize, Debug, Record, Clone)]
pub struct Dish {
    pub name: String,
    pub description: String,
    pub price: u16,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct SerializedDishRecord {
    pub _key: String,
    pub _id: String,
    pub _rev: String,
    pub name: String,
    pub description: String,
    pub price: u16,
}

#[maybe_async::test(
    feature = "blocking",
    async(all(not(feature = "blocking")), tokio::test)
)]
async fn custom_key() {
    let connection = common::setup_db().await;
    let doc = Dish {
        name: "Pizza".to_string(),
        description: "Italian Dish".to_string(),
        price: 13,
    };
    let record = DatabaseRecord::create_with_key(doc, "CustomKey".to_string(), &connection)
        .await
        .unwrap();
    assert_eq!(record.key(), "CustomKey");
    let queried: DatabaseRecord<Dish> = DatabaseRecord::find("CustomKey", &connection)
        .await
        .unwrap();
    assert_eq!(queried.key(), "CustomKey");
    let json = serde_json::to_string(&queried).unwrap();
    let serialized_dish: SerializedDishRecord = serde_json::from_str(&json).unwrap();
    assert_eq!(serialized_dish.price, record.price);
    assert_eq!(serialized_dish.name, record.name);
    assert_eq!(serialized_dish.description, record.description);
    assert_eq!(&serialized_dish._key, "CustomKey");
    assert_eq!(&serialized_dish._id, "Dish/CustomKey");
    assert_eq!(&serialized_dish._rev, record.rev());
}

#[maybe_async::test(
    feature = "blocking",
    async(all(not(feature = "blocking")), tokio::test)
)]
async fn serialization_works() {
    let connection = common::setup_db().await;
    let doc = Dish {
        name: "Pizza".to_string(),
        description: "Italian Dish".to_string(),
        price: 13,
    };
    let record = DatabaseRecord::create(doc, &connection).await.unwrap();
    let json = serde_json::to_string(&record).unwrap();
    let serialized_dish: SerializedDishRecord = serde_json::from_str(&json).unwrap();
    assert_eq!(serialized_dish.price, record.price);
    assert_eq!(serialized_dish.name, record.name);
    assert_eq!(serialized_dish.description, record.description);
    assert_eq!(&serialized_dish._key, record.key());
    assert_eq!(&serialized_dish._id, record.id());
    assert_eq!(&serialized_dish._rev, record.rev());
}

#[maybe_async::test(
    feature = "blocking",
    async(all(not(feature = "blocking")), tokio::test)
)]
async fn deserialization_works() {
    let connection = common::setup_db().await;
    let doc = Dish {
        name: "Pizza".to_string(),
        description: "Italian Dish".to_string(),
        price: 13,
    };
    let record = DatabaseRecord::create(doc, &connection).await.unwrap();
    let json = serde_json::to_string(&record).unwrap();
    let deserialize_record: DatabaseRecord<Dish> = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialize_record.price, record.price);
    assert_eq!(deserialize_record.name, record.name);
    assert_eq!(deserialize_record.description, record.description);
    assert_eq!(deserialize_record.key(), record.key());
    assert_eq!(deserialize_record.id(), record.id());
    assert_eq!(deserialize_record.rev(), record.rev());
}

#[maybe_async::test(
    feature = "blocking",
    async(all(not(feature = "blocking")), tokio::test)
)]
async fn revision_check_works() -> Result<(), String> {
    let connection = common::setup_db().await;
    let doc = Dish {
        name: "Piza".to_string(),
        description: "Italian Dish".to_string(),
        price: 13,
    };
    let mut record = DatabaseRecord::create(doc, &connection).await.unwrap();
    // We save the revision
    let old_rev = record.rev().clone();
    // We modify the document
    record.name = String::from("Pizza");
    // We save it
    record.save(&connection).await.unwrap();
    // The new revision should be changed
    assert_ne!(record.rev(), &old_rev);

    // Little trick to update the _rev field to the old value
    let json = serde_json::to_string(&record).unwrap();
    let mut serialized_dish: SerializedDishRecord = serde_json::from_str(&json).unwrap();
    serialized_dish._rev = old_rev.clone();
    let json = serde_json::to_string(&serialized_dish).unwrap();
    let mut deserialize_record: DatabaseRecord<Dish> = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialize_record.rev(), &old_rev);
    // End of trick

    // Should fail with rev check
    match deserialize_record
        .save_with_options(&connection, OperationOptions::default().ignore_revs(false))
        .await
    {
        Ok(_) => return Err(String::from("_rev check should have failed")),
        Err(e) => match e {
            Error::ArangoError(e) => {
                assert!(e.message.contains("conflict"));
                assert_eq!(e.http_error, ArangoHttpError::PreconditionFailed);
                assert_eq!(e.arango_error, ArangoError::ArangoConflict);
            }
            _ => return Err(String::from("Expected Arango Error")),
        },
    }
    // Should succeed without rev check
    deserialize_record.save(&connection).await.unwrap();

    Ok(())
}
