use arangors::client::reqwest::ReqwestClient;
use arangors::{Connection, Database};
use clap::{load_yaml, App};

use crate::config::Config;
use crate::error::MigrationError;
use crate::migration::Migration;

use crate::migration_manager::MigrationManager;

mod config;
mod error;
mod migration;
mod migration_data;
mod migration_manager;
mod migration_operation;

pub const LOG_STR: &str = "[Aragog]";
pub const ERROR_STR: &str = "[Aragog] [ERROR]";

#[derive(Debug)]
pub enum MigrationDirection {
    Up,
    Down(usize),
}

fn migrate(
    direction: MigrationDirection,
    db: &Database<ReqwestClient>,
    manager: MigrationManager,
) -> Result<(), MigrationError> {
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

fn main() -> Result<(), MigrationError> {
    let yaml = load_yaml!("cli.yaml");
    let matches = App::from(yaml).get_matches();

    let config = Config::new(&matches);
    let schema_path = config.schema_path.clone();

    let connection =
        Connection::establish_basic_auth(&config.db_host, &config.db_user, &config.db_pwd).unwrap();
    let db: Database<ReqwestClient> = match connection.db(&config.db_name) {
        Ok(val) => val,
        Err(_) => {
            println!(
                "{} Missing database {}, creating it.",
                LOG_STR, &config.db_name
            );
            connection.create_database(&config.db_name).unwrap()
        }
    };

    match matches.subcommand() {
        Some(("migrate", _args)) => {
            let manager = MigrationManager::new(&schema_path)?;
            migrate(MigrationDirection::Up, &db, manager)?;
        }
        Some(("rollback", args)) => {
            let manager = MigrationManager::new(&schema_path)?;
            let count = match args.value_of("COUNT").unwrap_or("1").parse() {
                Ok(val) => val,
                Err(_error) => {
                    return Err(MigrationError::InvalidParameter {
                        name: "COUNT".to_string(),
                        message: "Must be a valid number".to_string(),
                    })
                }
            };
            migrate(MigrationDirection::Down(count), &db, manager)?;
        }
        Some(("create_migration", args)) => {
            Migration::new(args.value_of("MIGRATION_NAME").unwrap(), &schema_path)?;
        }
        Some(("truncate_database", _args)) => {
            for info in db.accessible_collections().unwrap().iter() {
                if info.is_system {
                    continue;
                }
                println!("{} Dropping Collection {}", LOG_STR, &info.name);
                db.drop_collection(&info.name).unwrap();
            }
            println!("{} Truncated database collections.", LOG_STR);
        }
        _ => (),
    };
    Ok(())
}
