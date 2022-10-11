use crate::db::database_collection::DatabaseCollection;
use crate::db::database_connection_builder::{
    DatabaseConnectionBuilder, DatabaseSchemaOption, DbCredentialsOption,
};
use crate::schema::{DatabaseSchema, SchemaDatabaseOperation};
use crate::{DatabaseAccess, Error, OperationOptions};
use arangors_lite::{Connection, Database};
use std::collections::HashMap;
use std::marker::Copy;

/// Struct containing `ArangoDB` connections and information to access the database, collections and documents
#[derive(Clone, Debug)]
pub struct DatabaseConnection {
    /// Map between a collection name and a `DatabaseCollection` instance
    collections: HashMap<String, DatabaseCollection>,
    /// The database accessor
    database: Database,
    /// The default options for all `write` operations
    operation_options: OperationOptions,
}

/// Defines which `ArangoDB` authentication mode will be used
#[derive(Debug, Copy, Clone)]
pub enum AuthMode {
    /// Basic Username/Password authentication mode
    Basic,
    /// Recommended JWT authentication.
    ///
    /// # Note:
    /// The JWT has a 1 month validity (see [`arangors_lite`] documentation)
    /// And can lead to issues on docker
    ///
    /// [`arangors_lite`]: https://github.com/fMeow/arangors_lite
    Jwt,
}

impl Default for AuthMode {
    fn default() -> Self {
        Self::Basic
    }
}

impl DatabaseConnection {
    /// Starts the builder for the database connection instance.
    ///
    /// For default use with env var
    /// ```rust
    /// # use aragog::{AuthMode, DatabaseConnection};
    /// # use aragog::schema::DatabaseSchema;
    /// # #[tokio::main]
    /// # async fn main() {
    /// let db_connection = DatabaseConnection::builder()
    ///     // You can specify a host and credentials with this method.
    ///     // Otherwise, the builder will look for the env vars: `DB_HOST`, `DB_NAME`, `DB_USER` and `DB_PASSWORD`.
    ///     .with_credentials("http://localhost:8529", "db", "user", "password")
    ///     // You can specify a authentication mode between `Basic` and `Jwt`
    ///     // Otherwise the default value will be used (`Basic`).
    ///     .with_auth_mode(AuthMode::Basic)
    ///     // You can specify a schema path to initialize the database connection
    ///     // Otherwise the env var `SCHEMA_PATH` or the default value `config/db/schema.yaml` will be used.
    ///     .with_schema_path("config/db/schema.yaml")
    ///     // If you prefer you can use your own custom schema
    ///     .with_schema(DatabaseSchema::default())
    /// # .with_schema_path("tests/schema.yaml")
    /// # .with_credentials(
    /// #       &std::env::var("DB_HOST").unwrap_or("http://localhost:8529".to_string()),
    /// #       &std::env::var("DB_NAME").unwrap_or("aragog_test".to_string()),
    /// #       &std::env::var("DB_USER").unwrap_or("test".to_string()),
    /// #       &std::env::var("DB_PASSWORD").unwrap_or("test".to_string())
    /// #     )
    ///     // The schema wil silently apply to the database, useful only if you don't use the CLI and migrations
    ///     .apply_schema()
    ///     // You then need to build the database connection
    ///     .build()
    ///     .await
    ///     .unwrap();
    /// # }
    /// ```
    ///
    /// You can also specify a custom `DatabaseSchema` using `with_schema`.
    #[must_use]
    #[inline]
    pub fn builder() -> DatabaseConnectionBuilder {
        DatabaseConnectionBuilder {
            apply_schema: false,
            auth_mode: AuthMode::default(),
            credentials: DbCredentialsOption::Auto,
            schema: DatabaseSchemaOption::Auto,
            operation_options: OperationOptions::default(),
        }
    }

    #[maybe_async::maybe_async]
    pub(crate) async fn new(
        database: Database,
        schema: DatabaseSchema,
        apply_schema: bool,
        operation_options: OperationOptions,
    ) -> Result<Self, Error> {
        if apply_schema {
            schema.apply_to_database(&database, true).await?;
        }
        Ok(Self {
            collections: Self::load_schema(&database, schema).await?,
            database,
            operation_options,
        })
    }

    #[maybe_async::maybe_async]
    pub(crate) async fn connect(
        db_host: &str,
        db_name: &str,
        db_user: &str,
        db_password: &str,
        auth_mode: AuthMode,
    ) -> Result<Database, Error> {
        log::debug!("Connecting to database server on {} ...", db_host);
        let db_connection = match auth_mode {
            AuthMode::Basic => {
                Connection::establish_basic_auth(db_host, db_user, db_password).await?
            }
            AuthMode::Jwt => Connection::establish_jwt(db_host, db_user, db_password).await?,
        };
        log::debug!("Connecting to database {} ...", db_name);
        Ok(db_connection.db(db_name).await?)
    }

    /// retrieves a vector of all collection names from the database
    #[must_use]
    pub fn collections_names(&self) -> Vec<String> {
        self.collections.keys().cloned().collect()
    }

    #[must_use]
    pub(crate) fn collections(&self) -> Vec<&DatabaseCollection> {
        self.collections.values().collect()
    }

    /// **DESTRUCTIVE OPERATION**
    ///
    /// This will truncate all collections in the database, the collection will still exist but
    /// every document will be destryed.
    ///
    /// # Panics
    ///
    /// If the truncate fails on some collection the method will panic, see the `arangors_lite` documentation
    /// on collection truncate.
    #[maybe_async::maybe_async]
    pub async fn truncate(&self) {
        for collection in &self.collections {
            collection.1.truncate().await.unwrap();
        }
    }

    #[maybe_async::maybe_async]
    async fn load_schema(
        database: &Database,
        schema: DatabaseSchema,
    ) -> Result<HashMap<String, DatabaseCollection>, Error> {
        log::debug!(
            "Loading Schema with version {}",
            schema.version.unwrap_or(0)
        );
        let mut collections = HashMap::new();
        for collection in schema.collections {
            let coll = collection.get(database).await?;
            collections.insert(collection.name, DatabaseCollection::from(coll));
        }
        Ok(collections)
    }

    /// Returns the number of currently running server-side transactions
    #[maybe_async::maybe_async]
    pub async fn transactions_count(&self) -> Result<usize, Error> {
        let vec = self.database().list_transactions().await?;
        Ok(vec.len())
    }

    /// Return the check result of db_name
    #[maybe_async::maybe_async]
    pub async fn check_database(&self, name: &str) -> Result<bool, Error> {
        let info = self.database.info().await?;
        return Ok(info.name == name.to_string());
    }
}

impl DatabaseAccess for DatabaseConnection {
    fn operation_options(&self) -> OperationOptions {
        self.operation_options.clone()
    }

    fn collection(&self, collection: &str) -> Option<&DatabaseCollection> {
        self.collections.get(collection)
    }

    fn database(&self) -> &Database {
        &self.database
    }
}
