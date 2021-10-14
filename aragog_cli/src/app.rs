use crate::completions::CompletionOptions;
use clap::{AppSettings, Clap};

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Debug, Clap)]
pub enum Command {
    /// Launches migrations
    #[clap(setting = AppSettings::ColoredHelp)]
    Migrate,
    /// Rollbacks migrations (One by default).
    #[clap(setting = AppSettings::ColoredHelp)]
    Rollback {
        /// Number of migrations to rollback
        #[clap(default_value = "1")]
        count: u32,
    },
    /// Describes the current database state, the synced schema version, collections, document couts, etc.
    #[clap(setting = AppSettings::ColoredHelp)]
    Describe,
    /// Describes a database collection current indexes.
    #[clap(setting = AppSettings::ColoredHelp)]
    DescribeIndexes {
        /// Database collection name
        collection_name: String,
    },
    /// Loads migrations and check their format.
    #[clap(setting = AppSettings::ColoredHelp)]
    Check,
    /// Truncates the database, removes all collections, graphs, indexes and documents.
    #[clap(setting = AppSettings::ColoredHelp)]
    Truncate,
    /// Generates and apply a migration for collections, indexes and graphs missing from the schema.
    #[clap(setting = AppSettings::ColoredHelp)]
    Discover,
    /// Creates a new migration file.
    #[clap(setting = AppSettings::ColoredHelp)]
    CreateMigration {
        /// Sets the migration name (will be appended to the current timestamp)
        migration_name: String,
    },
    #[clap(setting = AppSettings::ColoredHelp)]
    /// Generates tab-completion script for your shell
    Completions(CompletionOptions),
}

#[derive(Debug, Clap)]
#[clap(
name = "aragog",
version = VERSION,
author = " Felix de Maneville <felix.maneville@qonfucius.team>",
about = "CLI too for aragog crate, handles ArangoDB migrations and schema generation.",
setting = AppSettings::ColoredHelp
)]
pub struct AragogCliApp {
    #[clap(short = 'c', long = "aragog-collection")]
    /// Sets the name of the config ArangoDB collection that will be used to synchronize database and schema version (by default 'AragogConfiguration' is used).
    pub schema_collection_name: Option<String>,
    #[clap(short = 'f', long = "schema-folder")]
    /// Sets the path for the migrations and schema (by default env var SCHEMA_PATH is used or `config/db/schema.yaml`).
    pub schema_path: Option<String>,
    #[clap(short = 'h', long = "db-host")]
    /// Sets the ArangoDB host (by default env var DB_HOST is used).
    pub db_host: Option<String>,
    #[clap(short = 'n', long = "db-name")]
    /// Sets the ArangoDB database name (by default env var DB_NAME is used).
    pub db_name: Option<String>,
    #[clap(short = 'u', long = "db-user")]
    /// Sets the ArangoDB database user (by default env var DB_USER is used).
    pub db_user: Option<String>,
    #[clap(short = 'p', long = "db-password")]
    /// Sets the ArangoDB database user password (by default env var DB_PASSWORD is used).
    pub db_pwd: Option<String>,
    #[clap(short, long, parse(from_occurrences))]
    pub verbose: u8,
    #[clap(subcommand, arg_enum)]
    pub command: Command,
}
