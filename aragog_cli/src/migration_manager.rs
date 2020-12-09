use std::fs;
use std::io::{Read, Write};

use arangors::client::reqwest::ReqwestClient;
use arangors::Database;

use aragog::schema::DatabaseSchema;

use crate::error::MigrationError;
use crate::migration::Migration;
use crate::LOG_STR;
use std::fs::OpenOptions;

const SCHEMA_NAME: &str = "schema.yaml";

#[derive(Debug)]
pub struct MigrationManager {
    pub migrations: Vec<Migration>,
    pub schema: DatabaseSchema,
    pub schema_file_path: String,
}

impl MigrationManager {
    pub fn new(schema_path: &str) -> Result<Self, MigrationError> {
        let db_path = Migration::migration_path(schema_path)?;
        let dir = match fs::read_dir(&db_path) {
            Ok(val) => val,
            Err(error) => {
                return Err(MigrationError::IOError {
                    message: error.to_string(),
                });
            }
        };
        let schema_file_path = format!("{}/{}", schema_path, SCHEMA_NAME);
        println!("{} Loading schema.yaml", LOG_STR);
        let schema = match fs::File::open(&schema_file_path) {
            Ok(mut file) => {
                let mut buff = String::new();
                file.read_to_string(&mut buff)?;
                serde_yaml::from_str(&buff)?
            }
            Err(error) => {
                println!("{} Missing schema file ({}) creating it...", LOG_STR, error);
                let mut file = fs::File::create(&schema_file_path)?;
                let schema = DatabaseSchema::default();
                let content = serde_yaml::to_string(&schema).unwrap();
                file.write_all(content.as_bytes())?;
                schema
            }
        };
        println!("{} Loading migrations...", LOG_STR);
        let mut migrations = Vec::new();
        for entry in dir {
            let entry = entry?;
            let path = entry.path();
            let path = match path.to_str() {
                None => {
                    return Err(MigrationError::InvalidFileName {
                        file_name: format!("{}", entry.path().display()),
                    });
                }
                Some(val) => val,
            };
            let file_name = match entry.file_name().into_string() {
                Ok(str) => str,
                Err(_) => {
                    return Err(MigrationError::InvalidFileName {
                        file_name: path.to_string(),
                    });
                }
            };
            let migration = Migration::load(&file_name, &schema_path)?;
            migrations.push(migration);
        }
        if migrations.is_empty() {
            return Err(MigrationError::NoMigrations);
        }
        migrations.sort_by(|a, b| a.version.cmp(&b.version));
        println!("{} Migrations loaded.", LOG_STR);
        Ok(Self {
            migrations,
            schema,
            schema_file_path,
        })
    }

    pub fn current_version(&self) -> u64 {
        self.schema.version.unwrap_or(0)
    }

    pub fn migrations_up(mut self, db: &Database<ReqwestClient>) -> Result<(), MigrationError> {
        let current_version = self.current_version();
        println!("{} Current Schema version: {}", LOG_STR, current_version);
        let mut i = 0;
        for migration in self.migrations.into_iter() {
            if migration.version > current_version {
                let version = migration.apply_up(&mut self.schema, db)?;
                self.schema.version = Some(version);
                Self::write_schema(&self.schema, &self.schema_file_path)?;
                i += 1;
            }
        }
        println!("{} Applied {} migrations", LOG_STR, i);
        Ok(())
    }

    pub fn migrations_down(
        mut self,
        count: usize,
        db: &Database<ReqwestClient>,
    ) -> Result<(), MigrationError> {
        let current_version = self.current_version();
        println!("{} Current Schema version: {}", LOG_STR, current_version);
        let mut i = 0;
        self.migrations.reverse();
        for migration in self.migrations.into_iter() {
            if i >= count {
                break;
            }
            if migration.version <= current_version {
                let version = migration.apply_down(&mut self.schema, db)?;
                self.schema.version = Some(version - 1);
                Self::write_schema(&self.schema, &self.schema_file_path)?;
                i += 1;
            }
        }
        println!("{} Rollbacked {} migrations", LOG_STR, i);
        Ok(())
    }

    pub fn write_schema(
        schema: &DatabaseSchema,
        schema_file_path: &str,
    ) -> Result<(), MigrationError> {
        println!("{} Saving schema to {}", LOG_STR, schema_file_path);
        let mut file = OpenOptions::new().write(true).open(&schema_file_path)?;
        let content = serde_yaml::to_string(&schema).unwrap();
        // Cleans the file
        file.set_len(0)?;
        file.write_all(content.as_bytes())?;
        println!(
            "{} Bumped schema to version {}",
            LOG_STR,
            schema.version.unwrap()
        );
        Ok(())
    }
}
