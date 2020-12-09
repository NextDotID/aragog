use crate::schema::{DatabaseSchema, SCHEMA_DEFAULT_FILE_NAME, SCHEMA_DEFAULT_PATH};
use crate::{AuthMode, DatabaseConnectionPool, ServiceError};
use std::convert::TryInto;

#[derive(Debug, Clone)]
pub(crate) struct PoolCredentials {
    db_host: String,
    db_name: String,
    db_user: String,
    db_password: String,
}

#[derive(Debug, Clone)]
pub(crate) enum PoolCredentialsOption {
    Auto,
    Custom(PoolCredentials),
}

#[derive(Debug)]
pub(crate) enum DatabaseSchemaOption {
    Auto,
    Path(String),
    Custom(DatabaseSchema),
}

impl Into<PoolCredentials> for PoolCredentialsOption {
    fn into(self) -> PoolCredentials {
        match self {
            Self::Custom(cred) => cred,
            Self::Auto => PoolCredentials {
                db_host: std::env::var("DB_HOST").expect("Please define DB_HOST env var."),
                db_name: std::env::var("DB_NAME").expect("Please define DB_NAME env var."),
                db_user: std::env::var("DB_USER").expect("Please define DB_USER env var."),
                db_password: std::env::var("DB_PASSWORD")
                    .expect("Please define DB_PASSWORD env var."),
            },
        }
    }
}

impl TryInto<DatabaseSchema> for DatabaseSchemaOption {
    type Error = ServiceError;

    fn try_into(self) -> Result<DatabaseSchema, Self::Error> {
        match self {
            Self::Custom(schema) => Ok(schema),
            Self::Path(path) => DatabaseSchema::load(&path),
            Self::Auto => {
                let schema_path = match std::env::var("SCHEMA_PATH") {
                    Ok(path) => path,
                    Err(_err) => {
                        log::debug!(
                            "Missing SCHEMA_PATH env var, using default value: {}",
                            SCHEMA_DEFAULT_PATH
                        );
                        SCHEMA_DEFAULT_PATH.to_string()
                    }
                };
                DatabaseSchema::load(&format!("{}/{}", schema_path, SCHEMA_DEFAULT_FILE_NAME))
            }
        }
    }
}

/// Builder for `DatabaseConnectionPool`
pub struct DatabaseConnectionPoolBuilder {
    pub(crate) apply_schema: bool,
    pub(crate) auth_mode: AuthMode,
    pub(crate) credentials: PoolCredentialsOption,
    pub(crate) schema: DatabaseSchemaOption,
}

impl DatabaseConnectionPoolBuilder {
    /// Initializes the Database connection pool according to specified building methods.
    ///
    /// If nothing was set like in this example:
    /// ```rust
    /// # use aragog::DatabaseConnectionPool;
    /// # #[tokio::main]
    /// # async fn main() {
    /// let db_pool = DatabaseConnectionPool::builder()
    /// # .with_schema_path("tests/schema.yaml")
    /// # .apply_schema()
    ///     .build()
    ///     .await
    ///     .unwrap();
    /// # }
    /// ```
    /// The builder will use the following:
    /// - The schema will be loaded though `SCHEMA_PATH` env var or the default value: `./src/config/db/schema.yaml`
    /// - The credentials will be loaded through the following env vars:
    ///     * `DB_HOST` - The ArangoDB host to connect to (`http://localhost:8529` by default or `http://arangodb:8529` on docker containers)
    ///     * `DB_NAME` - The name of the ArangoDB database to connect to
    ///     * `DB_USER` - The username of a ArangoDb user with access to the database
    ///     * `DB_PASSWORD` - The password associated with `DB_USER`
    /// - The auth mode will be `AuthMode::Basic`
    ///
    /// # Panics
    ///
    /// If the provided credentials are wrong or if the database is not running the function will panic.
    /// If any of the previous env var is not specified the function will panic with an explanation message.
    #[maybe_async::maybe_async]
    pub async fn build(self) -> Result<DatabaseConnectionPool, ServiceError> {
        let credentials = self.credentials();
        let auth_mode = self.auth_mode();
        let apply_schema = self.apply_schema;
        let schema = self.schema()?;
        DatabaseConnectionPool::new(
            &credentials.db_host,
            &credentials.db_name,
            &credentials.db_user,
            &credentials.db_password,
            auth_mode,
            schema,
            apply_schema,
        )
        .await
    }

    /// Specifies a custom authentication mode for ArangoDB connection.
    ///
    /// If not specified `AuthMode::Basic` will be used.
    pub fn with_auth_mode(mut self, mode: AuthMode) -> Self {
        log::debug!("[Pool builder] Auth mode {:?} will be used", mode);
        self.auth_mode = mode;
        self
    }

    /// Specifies custom credentials for ArangoDB connection
    ///
    /// # Arguments
    ///
    /// * `db_host` - The ArangoDB host to connect to (`http://localhost:8529` by default or `http://arangodb:8529` on docker containers)
    /// * `db_name` - The name of the ArangoDB database to connect to
    /// * `db_user` - The username of a ArangoDb user with access to the database
    /// * `db_password` - The password associated with `db_user`
    pub fn with_credentials<'a>(
        mut self,
        db_host: &str,
        db_name: &str,
        db_user: &str,
        db_password: &str,
    ) -> Self {
        log::debug!(
            "[Pool builder] Custom credentials for ArangoDB host {} will be used",
            db_host
        );
        self.credentials = PoolCredentialsOption::Custom(PoolCredentials {
            db_host: String::from(db_host),
            db_name: String::from(db_name),
            db_user: String::from(db_user),
            db_password: String::from(db_password),
        });
        self
    }

    /// Specifies a custom schema for ArangoDB initialization.
    ///
    /// If not specified,`SCHEMA_PATH` env var will be used or the default value: `./src/config/db/schema.yaml`
    pub fn with_schema(mut self, schema: DatabaseSchema) -> Self {
        log::debug!("[Pool builder] Custom schema will be used");
        self.schema = DatabaseSchemaOption::Custom(schema);
        self
    }

    /// Call this method if you want the schema to be applied.
    /// This will ignore any errors, so check the `debug` to find a hidden issue.
    ///
    /// Use it when you use your own custom schema and no `aragog_cli` migrations.
    pub fn apply_schema(mut self) -> Self {
        log::debug!("[Pool builder] Schema will be silently applied");
        self.apply_schema = true;
        self
    }

    /// Specifies a custom schema path for ArangoDB initialization.
    ///
    /// If not specified,`SCHEMA_PATH` env var will be used or the default value: `./src/config/db/schema.yaml`
    pub fn with_schema_path(mut self, path: &str) -> Self {
        log::debug!("[Pool builder] Schema from {} will be used", path);
        self.schema = DatabaseSchemaOption::Path(String::from(path));
        self
    }

    fn credentials(&self) -> PoolCredentials {
        self.credentials.clone().into()
    }

    fn schema(self) -> Result<DatabaseSchema, ServiceError> {
        self.schema.try_into()
    }

    fn auth_mode(&self) -> AuthMode {
        self.auth_mode.clone()
    }
}
