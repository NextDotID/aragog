use std::collections::HashMap;
use std::fs;

use arangors::client::reqwest::ReqwestClient;
use arangors::{Collection, Connection, Database};
use serde_json::Value;

use crate::db::database_collection::DatabaseCollection;
use crate::helpers::json_helper;
use crate::query::JsonQueryResult;
use crate::ServiceError;

const SCHEMA_DEFAULT_PATH: &str = "./src/config/db/schema.json";
const SCHEMA_COLLECTION_KEY: &str = "collections";
const SCHEMA_EDGE_COLLECTION_KEY: &str = "edge_collections";
const SCHEMA_COLLECTION_NAME: &str = "name";

/// Struct containing ArangoDB connections and information to access the database, collections and documents
#[derive(Clone)]
pub struct DatabaseConnectionPool {
    /// Map between a collection name and a `DatabaseCollection` instance
    pub collections: HashMap<String, DatabaseCollection>,
    /// The database accessor
    pub database: Database<ReqwestClient>,
}

/// Defines which ArangoDB authentication mode will be used
#[derive(Debug, Clone)]
pub enum AuthMode {
    /// Basic Username/Password authentication mode
    Basic,
    /// Recommended JWT authentication.
    ///
    /// # Note:
    /// The JWT has a 1 month validity (see [arangors] documentation)
    /// And can lead to issues on docker
    ///
    /// [arangors]: https://github.com/fMeow/arangors
    Jwt,
}

impl Default for AuthMode {
    fn default() -> Self {
        Self::Basic
    }
}

impl DatabaseConnectionPool {
    /// Creates and returns a new instance according to provided parameters.
    ///
    /// # Arguments
    ///
    /// * `db_host` - The ArangoDB host to connect to (`http://localhost:8529` by default or `http://arangodb:8529` on docker containers)
    /// * `db_name` - The name of the ArangoDB database to connect to
    /// * `db_user` - The username of a ArangoDb user with access to the database
    /// * `db_password` - The password associated with `db_user`
    /// * `auth_mode` - The chosen authentication mode, if set to `default` the basic auth will be used
    ///
    /// To load the schema this function will try to access `SCHEMA_PATH` env var or use the default value: `./src/config/db/schema.json`
    ///
    /// # Panics
    ///
    /// If the provided credentials are wrong or if the database is not running  the function will panic.
    pub async fn new(
        db_host: &str,
        db_name: &str,
        db_user: &str,
        db_password: &str,
        auth_mode: AuthMode,
    ) -> Self {
        log::info!("Connecting to database server...");
        let db_connection = match auth_mode {
            AuthMode::Basic => Connection::establish_basic_auth(db_host, db_user, db_password)
                .await
                .unwrap(),
            AuthMode::Jwt => Connection::establish_jwt(db_host, db_user, db_password)
                .await
                .unwrap(),
        };
        log::info!("Connected to database server.");
        let database = db_connection.db(&db_name).await.unwrap();
        DatabaseConnectionPool::load_schema(database).await.unwrap()
    }

    /// Creates and returns a pool instance based on env variables, reducing boilerplate code.
    ///
    /// # Environnment
    ///
    /// * `DB_HOST` - The ArangoDB host to connect to (`http://localhost:8529` by default or `http://arangodb:8529` on docker containers)
    /// * `DB_NAME` - The name of the ArangoDB database to connect to
    /// * `DB_USER` - The username of a ArangoDb user with access to the database
    /// * `DB_PASSWORD` - The password associated with `db_user`
    ///
    /// To load the schema this function will try to access `SCHEMA_PATH` env var or use the default value: `./src/config/db/schema.json`
    ///
    /// # Panics
    ///
    /// If the provided credentials are wrong or if the database is not running  the function will panic.
    /// If any of the previous env var is not specified the function will panic with an explanation message.
    pub async fn auto_setup() -> Self {
        let db_host = std::env::var("DB_HOST").expect("Please define DB_HOST env var.");
        let db_name = std::env::var("DB_NAME").expect("Please define DB_NAME env var.");
        let db_user = std::env::var("DB_USER").expect("Please define DB_USER env var.");
        let db_password = std::env::var("DB_PASSWORD").expect("Please define DB_PASSWORD env var.");
        DatabaseConnectionPool::new(
            &db_host,
            &db_name,
            &db_user,
            &db_password,
            AuthMode::default(),
        )
        .await
    }

    /// Simple wrapper to retrieve a Collection without using the HashMap directly.
    /// Can panic if the key matching `collection` is missing
    pub fn get_collection(&self, collection: &str) -> &Collection<ReqwestClient> {
        if !self.collections.contains_key(collection) {
            panic!(
                "Undefined collection {}, check your schema.json file",
                collection
            )
        }
        &self.collections[collection].collection
    }

    /// **DESTRUCTIVE OPERATION**
    /// This will truncate all collections in the database pool, the collection will still exist but
    /// every document will be destryed.
    ///
    /// # Panics
    ///
    /// If the truncate fails on some collection the method will panic, see the `arangors` documentation
    /// on collection truncate.
    pub async fn truncate(&self) {
        for collection in self.collections.iter() {
            collection.1.collection.truncate().await.unwrap();
        }
    }

    /// Runs an AQL query and returns the found documents
    pub async fn aql_get(&self, aql: &str) -> Result<JsonQueryResult, ServiceError> {
        let query_result: Vec<Value> = match self.database.aql_str(aql).await {
            Ok(value) => value,
            Err(error) => {
                log::error!("{}", error);
                return Err(ServiceError::from(error));
            }
        };
        Ok(JsonQueryResult::new(query_result))
    }

    async fn load_schema(
        database: Database<ReqwestClient>,
    ) -> Result<DatabaseConnectionPool, String> {
        let schema_path = match std::env::var("SCHEMA_PATH") {
            Ok(path) => path,
            Err(_err) => SCHEMA_DEFAULT_PATH.to_string(),
        };
        let file = fs::File::open(&schema_path)
            .expect(&format!("{} doesn't open correctly", &schema_path));
        let json: Value = serde_json::from_reader(file)
            .expect(&format!("{} is not formatted correctly", &schema_path));

        let mut json_collections: Vec<Value> = Vec::new();
        if let Value::Array(values) = &json[SCHEMA_COLLECTION_KEY] {
            json_collections = values.clone();
        }
        let mut collections = Self::load_collections(&database, json_collections)
            .await
            .unwrap();
        let mut json_collections: Vec<Value> = Vec::new();
        if let Value::Array(values) = &json[SCHEMA_EDGE_COLLECTION_KEY] {
            json_collections = values.clone();
        }
        Self::load_edge_collections(&database, json_collections, &mut collections)
            .await
            .unwrap();
        Ok(DatabaseConnectionPool {
            collections,
            database,
        })
    }

    async fn load_collections(
        database: &Database<ReqwestClient>,
        json_collections: Vec<Value>,
    ) -> Result<HashMap<String, DatabaseCollection>, String> {
        let mut collections_map = HashMap::new();
        for json_collection in json_collections {
            let collection_name =
                json_helper::load_json_string_key(&json_collection, &SCHEMA_COLLECTION_NAME)?;
            let collection: Collection<ReqwestClient>;

            match database.collection(&collection_name).await {
                Ok(coll) => collection = coll,
                Err(_error) => {
                    log::info!("Collection {} not found, creating...", &collection_name);
                    collection = database.create_collection(&collection_name).await.unwrap()
                }
            }
            let collection_container = DatabaseCollection {
                collection_name,
                collection,
            };
            Self::handle_index(&database, json_collection, &collection_container).await?;
            collections_map.insert(
                collection_container.collection_name.clone(),
                collection_container,
            );
        }
        Ok(collections_map)
    }

    async fn load_edge_collections(
        database: &Database<ReqwestClient>,
        json_collections: Vec<Value>,
        collections: &mut HashMap<String, DatabaseCollection>,
    ) -> Result<(), String> {
        for json_collection in json_collections {
            let collection_name =
                json_helper::load_json_string_key(&json_collection, &SCHEMA_COLLECTION_NAME)?;
            let collection: Collection<ReqwestClient>;

            match database.collection(&collection_name).await {
                Ok(coll) => collection = coll,
                Err(_error) => {
                    log::info!(
                        "Edge Collection {} not found, creating...",
                        &collection_name
                    );
                    collection = database
                        .create_edge_collection(&collection_name)
                        .await
                        .unwrap()
                }
            }
            let collection_container = DatabaseCollection {
                collection_name,
                collection,
            };
            collections.insert(
                collection_container.collection_name.clone(),
                collection_container,
            );
        }
        Ok(())
    }

    async fn handle_index(
        database: &Database<ReqwestClient>,
        json_collection: Value,
        collection: &DatabaseCollection,
    ) -> Result<(), String> {
        let indexes = json_collection["indexes"].as_array().unwrap();

        for index in indexes {
            let index = DatabaseCollection::index_from_json(index)?;
            if collection.index_exists(database, &index).await.unwrap() {
                log::info!("Index {} exists, skipping...", index.name);
                continue;
            }
            log::info!("Index {} not found, creating...", index.name);
            database
                .create_index(&collection.collection_name, &index)
                .await
                .unwrap();
        }
        Ok(())
    }
}
