// TODO: enable this
// #![forbid(unsafe_code)]
#![warn(
    clippy::all,
    clippy::correctness,
    clippy::suspicious,
    clippy::style,
    clippy::complexity,
    clippy::perf,
    nonstandard_style
)]

#[macro_use]
extern crate prettytable;

use clap::Parser;
use std::process::exit;

use crate::app::{AragogCliApp, Command};
pub use config::log;

use crate::config::Config;
use crate::describe::{describe_collection_indexes, describe_db};
use crate::discover::discover_migration;
use crate::error::AragogCliError;
use crate::log_level::LogLevel;
use crate::migration::Migration;
use crate::migration_manager::MigrationManager;
use crate::versioned_database::VersionedDatabase;

mod app;
mod completions;
mod config;
mod describe;
mod discover;
mod error;
mod log_level;
mod migration;
mod migration_data;
mod migration_manager;
mod migration_operation;
mod versioned_database;

#[derive(Debug)]
pub enum MigrationDirection {
    Up,
    Down(u32),
}

fn migrate(
    direction: MigrationDirection,
    db: &mut VersionedDatabase,
    manager: MigrationManager,
) -> Result<(), AragogCliError> {
    match direction {
        MigrationDirection::Up => {
            manager.migrations_up(db)?;
        }
        MigrationDirection::Down(count) => {
            manager.migrations_down(count, db)?;
        }
    };
    Ok(())
}

fn handle_commands() -> Result<(), AragogCliError> {
    let opts: AragogCliApp = AragogCliApp::parse();

    match &opts.command {
        Command::Check => {
            let config = Config::new(&opts)?;
            MigrationManager::new(&config.schema_path)?;
        }
        Command::Migrate => {
            let config = Config::new(&opts)?;
            let schema_path = config.schema_path.clone();
            let manager = MigrationManager::new(&schema_path)?;
            let mut db = VersionedDatabase::init(&config)?;
            migrate(MigrationDirection::Up, &mut db, manager)?;
        }
        Command::Rollback { count } => {
            let config = Config::new(&opts)?;
            let schema_path = config.schema_path.clone();
            let manager = MigrationManager::new(&schema_path)?;
            let mut db = VersionedDatabase::init(&config)?;
            migrate(MigrationDirection::Down(*count), &mut db, manager)?;
        }
        Command::CreateMigration { migration_name } => {
            let config = Config::new(&opts)?;
            Migration::new(migration_name, &config.schema_path, true)?;
        }
        Command::Truncate => {
            let config = Config::new(&opts)?;
            let db = VersionedDatabase::init(&config)?;
            for info in db.accessible_collections()?.iter() {
                if info.is_system {
                    continue;
                }
                log(
                    format!("Dropping Collection {}", &info.name),
                    LogLevel::Info,
                );
                db.drop_collection(&info.name)?;
            }
            for graph in db.graphs()?.graphs {
                log(format!("Dropping Graph {}", &graph.name), LogLevel::Info);
                db.drop_graph(&graph.name, false)?;
            }
            log("Truncated database collections and graphs", LogLevel::Info);
        }
        Command::Discover => {
            let config = Config::new(&opts)?;
            let schema_path = config.schema_path.clone();
            let mut db = VersionedDatabase::init(&config)?;
            let manager = MigrationManager::new(&schema_path)?;
            let migration = discover_migration(&mut db, &config)?;
            if migration.data.is_empty() {
                log(
                    "Your schema and database are synchronized, no discovery required",
                    LogLevel::Info,
                );
                return Ok(());
            }
            log(
                format!("Created discover migration {}", migration.path),
                LogLevel::Info,
            );
            migration.apply_up(&mut db, true)?;
            db.save()?;
            MigrationManager::write_schema(&db.schema, &manager.schema_file_path)?;
            log(
                format!(
                    "Applied discover migration to schema, new version: {}",
                    db.schema.version.unwrap()
                ),
                LogLevel::Info,
            );
        }
        Command::Describe => {
            let config = Config::new(&opts)?;
            describe_db(&config)?;
        }
        Command::DescribeIndexes { collection_name } => {
            let config = Config::new(&opts)?;
            describe_collection_indexes(&config, collection_name)?;
        }
        Command::Completions(opts) => {
            opts.generate();
        }
    };
    Ok(())
}

fn main() {
    let res = handle_commands();

    if let Err(e) = res {
        eprintln!("Error: {}", e);
        exit(e.exit_code())
    }
}
