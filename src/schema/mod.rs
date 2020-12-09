use arangors::client::reqwest::ReqwestClient;
use arangors::{ClientError, Database};

pub use {
    collection_schema::CollectionSchema, database_schema::DatabaseSchema,
    graph_schema::GraphSchema, index_schema::IndexSchema,
};

mod collection_schema;
mod database_schema;
mod graph_schema;
mod index_schema;

/// Default schema path, can be overridden manually or set as `SCHEMA_PATH` env var
pub const SCHEMA_DEFAULT_PATH: &str = "./src/config/db";
/// Default schema file name, can be overridden manually
pub const SCHEMA_DEFAULT_FILE_NAME: &str = "schema.yaml";

/// Trait used for all schema elements allowing to synchronise schema changes.
/// Used by `aragog_cli` for migrations and `DatabaseConnectionPool`
#[maybe_async::maybe_async]
pub trait SchemaDatabaseOperation {
    /// The `arangors` type to retrieve with the `get` method
    type PoolType;

    /// Utility method to allow "silent" error handling
    fn handle_error<T>(result: Result<T, ClientError>, silent: bool) -> Result<(), ClientError> {
        match result {
            Err(error) => {
                if silent {
                    log::debug!("Ignored error: {}", error);
                    return Ok(());
                }
                return Err(error);
            }
            Ok(_val) => Ok(()),
        }
    }

    /// Applies (creates) the schema element to the database
    ///
    /// # Parameters
    /// * `database` - reference the the db connection object (`arangors`)
    /// * `silent` - Should the errors be ignored
    async fn apply_to_database(
        &mut self,
        database: &Database<ReqwestClient>,
        silent: bool,
    ) -> Result<(), ClientError>;

    /// Deletes the schema element from the database.
    async fn drop(&self, database: &Database<ReqwestClient>) -> Result<(), ClientError>;

    /// Retrieves the `arangors` element from the schema element
    async fn get(&self, database: &Database<ReqwestClient>) -> Result<Self::PoolType, ClientError>;
}
