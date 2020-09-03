use std::collections::HashMap;
use std::fs;

use arangors::{Collection, Connection, Database};
use arangors::client::reqwest::ReqwestClient;
use serde_json::Value;

use crate::helpers::{json_helper};
use crate::db::database_collection::DatabaseCollection;

const SCHEMA_DEFAULT_PATH: &str = "./src/config/db/schema.json";
const SCHEMA_COLLECTION_KEY: &str = "collections";
const SCHEMA_COLLECTION_NAME: &str = "name";

/// Struct containing ArangoDB connections and information to access the database, collections and documents
#[derive(Clone)]
pub struct DatabaseConnectionPool {
    db_host: String,
    db_user: String,
    db_name: String,
    /// Map between a collection name and a `DatabaseCollection` instance
    pub collections: HashMap<String, DatabaseCollection>,
    /// The database accessor
    pub database: Database<ReqwestClient>,
}

impl DatabaseConnectionPool {
    /// Creates and returns a new struct instance.
    /// This function will base itself on environment variables and on the schema json file:
    /// `./src/config/db/schema.json`
    ///
    /// # Panics
    ///
    /// If any of the required env variables are missing the function will panic with a explanation
    pub async fn new(db_host: &str, db_name :&str, db_user: &str, db_password: &str) -> Self {
        log::info!("Connecting to database server...");
        let db_connection = Connection::establish_basic_auth(
            db_host,
            db_user,
            db_password).await.unwrap();
        log::info!("Connected to database server.");
        let database = db_connection.db(&db_name).await.unwrap();
        let collections = DatabaseConnectionPool::load_schema(&database).await.unwrap();
        DatabaseConnectionPool {
            db_name: String::from(db_name), db_user: String::from(db_user), db_host: String::from(db_host),
            collections, database }
    }

    /// Simple wrapper to retrieve a Collection without using the HashMap directly.
    /// Can panic if the key matching `collection` is missing
    pub fn get_collection(&self, collection: &str) -> &Collection<ReqwestClient> {
        &self.collections[collection].collection
    }

    async fn load_schema(database: &Database<ReqwestClient>) -> Result<HashMap<String, DatabaseCollection>, String> {
        let schema_path = match std::env::var("SCHEMA_PATH") {
            Ok(path) => path,
            Err(_err) => SCHEMA_DEFAULT_PATH.to_string()
        };
        let file = fs::File::open(&schema_path).expect(&format!("{} doesn't open correctly", &schema_path));
        let json: Value = serde_json::from_reader(file).expect(&format!("{} is not formatted correctly", &schema_path));

        let mut json_collections: Vec<Value> = Vec::new();
        if let Value::Array(values) = &json[SCHEMA_COLLECTION_KEY] {
            json_collections = values.clone();
        }

        let mut collections_map = HashMap::new();

        for json_collection in json_collections {
            let collection_name =
                json_helper::load_json_string_key(&json_collection, &SCHEMA_COLLECTION_NAME)?;
            let collection: Collection<ReqwestClient>;

            match database.collection(&collection_name).await {
                Ok(coll) => {
                    collection = coll
                }
                Err(_error) => {
                    log::info!("Collection {} not found, creating...", &collection_name);
                    collection = database.create_collection(&collection_name).await.unwrap()
                }
            }
            let collection_container = DatabaseCollection {
                collection_name,
                collection,
            };
            Self::handle_index(database, json_collection, &collection_container).await?;
            collections_map.insert(collection_container.collection_name.clone(), collection_container);
        }
        Ok(collections_map)
    }

    async fn handle_index(database: &Database<ReqwestClient>, json_collection: Value, collection: &DatabaseCollection) -> Result<(), String> {
        let indexes = json_collection["indexes"].as_array().unwrap();

        for index in indexes {
            let index = DatabaseCollection::index_from_json(index)?;
            if collection.index_exists(database, &index).await.unwrap() {
                log::info!("Index {} exists, skipping...", index.name);
                continue;
            }
            log::info!("Index {} not found, creating...", index.name);
            database.create_index(&collection.collection_name, &index).await.unwrap();
        }
        Ok(())
    }
}