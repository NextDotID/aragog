use std::collections::HashMap;

use arangors::client::reqwest::ReqwestClient;
use arangors::{Collection, Connection, Database};
use serde_json::Value;

use crate::db::database_collection::DatabaseCollection;
use crate::db::database_connection_pool_builder::{
    DatabaseConnectionPoolBuilder, DatabaseSchemaOption, PoolCredentialsOption,
};
use crate::query::JsonQueryResult;
use crate::schema::{DatabaseSchema, SchemaDatabaseOperation};
use crate::ServiceError;

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
    /// Starts the builder for the database connection pool instance.
    ///
    /// For default use with env var
    /// ```rust
    /// # use aragog::{AuthMode, DatabaseConnectionPool};
    /// # use aragog::schema::DatabaseSchema;
    /// # #[tokio::main]
    /// # async fn main() {
    /// let db_pool = DatabaseConnectionPool::builder()
    ///     // You can specify a host and credentials with this method.
    ///     // Otherwise, the builder will look for the env vars: `DB_HOST`, `DB_NAME`, `DB_USER` and `DB_PASSWORD`.
    ///     .with_credentials("http://localhost:8529", "db", "user", "password")
    ///     // You can specify a authentication mode between `Basic` and `Jwt`
    ///     // Otherwise the default value will be used (`Basic`).
    ///     .with_auth_mode(AuthMode::Basic)
    ///     // You can specify a schema path to initialize the database pool
    ///     // Otherwise the env var `SCHEMA_PATH` or the default value `config/db/schema.yaml` will be used.
    ///     .with_schema_path("config/db/schema.yaml")
    ///     // If you prefer you can use your own custom schema
    ///     .with_schema(DatabaseSchema::default())
    /// # .with_schema_path("tests/schema.yaml")
    /// # .with_credentials(
    /// #       &std::env::var("DB_HOST").unwrap_or("http://localhost:8529".to_string()),
    /// #       &std::env::var("DB_NAME").unwrap_or("aragog_test".to_string()),
    /// #       &std::env::var("DB_USER").unwrap_or("test".to_string()),
    /// #       &std::env::var("DB_PWD").unwrap_or("test".to_string())
    /// #     )
    ///     // The schema wil silently apply to the database, useful only if you don't use the CLI and migrations
    ///     .apply_schema()
    ///     // You then need to build the pool
    ///     .build()
    ///     .await
    ///     .unwrap();
    /// # }
    /// ```
    ///
    /// You can also specify a custom `DatabaseSchema` using `with_schema`.
    pub fn builder() -> DatabaseConnectionPoolBuilder {
        DatabaseConnectionPoolBuilder {
            apply_schema: false,
            auth_mode: AuthMode::default(),
            credentials: PoolCredentialsOption::Auto,
            schema: DatabaseSchemaOption::Auto,
        }
    }

    #[maybe_async::maybe_async]
    pub(crate) async fn new(
        db_host: &str,
        db_name: &str,
        db_user: &str,
        db_password: &str,
        auth_mode: AuthMode,
        mut schema: DatabaseSchema,
        apply_schema: bool,
    ) -> Result<Self, ServiceError> {
        log::info!("Connecting to database server...");
        let db_connection = match auth_mode {
            AuthMode::Basic => {
                Connection::establish_basic_auth(db_host, db_user, db_password).await?
            }
            AuthMode::Jwt => Connection::establish_jwt(db_host, db_user, db_password).await?,
        };
        log::info!("Connected to database server.");
        let database = db_connection.db(&db_name).await.unwrap();
        if apply_schema {
            schema.apply_to_database(&database, true).await?
        }
        DatabaseConnectionPool::load_schema(database, schema).await
    }

    /// Simple wrapper to retrieve a Collection without using the HashMap directly.
    /// Can panic if the key matching `collection` is missing
    pub fn get_collection(&self, collection: &str) -> &Collection<ReqwestClient> {
        if !self.collections.contains_key(collection) {
            panic!(
                "Undefined collection {}, check your schema.yaml file",
                collection
            )
        }
        &self.collections[collection].collection
    }

    /// **DESTRUCTIVE OPERATION**
    ///
    /// This will truncate all collections in the database pool, the collection will still exist but
    /// every document will be destryed.
    ///
    /// # Panics
    ///
    /// If the truncate fails on some collection the method will panic, see the `arangors` documentation
    /// on collection truncate.
    #[maybe_async::maybe_async]
    pub async fn truncate(&self) {
        for collection in self.collections.iter() {
            collection.1.collection.truncate().await.unwrap();
        }
    }

    /// Runs an AQL query and returns the found documents
    #[maybe_async::maybe_async]
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

    #[maybe_async::maybe_async]
    async fn load_schema(
        database: Database<ReqwestClient>,
        schema: DatabaseSchema,
    ) -> Result<DatabaseConnectionPool, ServiceError> {
        let mut collections = HashMap::new();
        for collection in schema.collections.iter() {
            collections.insert(
                collection.name.clone(),
                DatabaseCollection {
                    collection_name: collection.name.clone(),
                    collection: collection.get(&database).await?,
                },
            );
        }
        Ok(DatabaseConnectionPool {
            collections,
            database,
        })
    }
}
