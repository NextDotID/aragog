use std::fs;
use std::fs::OpenOptions;
use std::io::Write;

use aragog::schema::DatabaseSchema;

use crate::error::MigrationError;
use crate::log;
use crate::log_level::LogLevel;
use crate::migration::Migration;
use crate::VersionedDatabase;

const SCHEMA_NAME: &str = "schema.yaml";

const HELP_MESSAGE: &str = "# \n\
                            # This schema file is auto generated and synchronized with the database.\n\
                            # Editing it will have no effect.\n\
                            # \n";

#[derive(Debug)]
pub struct MigrationManager {
    pub migrations: Vec<Migration>,
    pub schema_file_path: String,
}

impl MigrationManager {
    pub fn serialized_schema(schema: &DatabaseSchema) -> String {
        let content = serde_yaml::to_string(&schema).unwrap();
        format!("{}{}", HELP_MESSAGE, content)
    }

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
        log(format!("Loading schema.yaml"), LogLevel::Debug);
        match fs::File::open(&schema_file_path) {
            Ok(_) => (),
            Err(error) => {
                log(
                    format!("Missing schema file ({}) creating it...", error),
                    LogLevel::Debug,
                );
                fs::File::create(&schema_file_path)?;
            }
        };
        log(format!("Loading migrations..."), LogLevel::Debug);
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
        log(format!("Migrations loaded."), LogLevel::Debug);
        Ok(Self {
            migrations,
            schema_file_path,
        })
    }

    pub fn migrations_up(self, db: &mut VersionedDatabase) -> Result<(), MigrationError> {
        let current_version = db.schema_version();
        Self::write_schema(&db.schema, &self.schema_file_path)?;
        log(
            format!("Current Schema version: {}", current_version),
            LogLevel::Debug,
        );
        let mut i = 0;
        for migration in self.migrations.into_iter() {
            if migration.version > current_version {
                migration.apply_up(db)?;
                db.save()?;
                Self::write_schema(&db.schema, &self.schema_file_path)?;
                i += 1;
            }
        }
        log(format!("Applied {} migrations", i), LogLevel::Info);
        Ok(())
    }

    pub fn migrations_down(
        mut self,
        count: usize,
        db: &mut VersionedDatabase,
    ) -> Result<(), MigrationError> {
        let current_version = db.schema_version();
        Self::write_schema(&db.schema, &self.schema_file_path)?;
        log(
            format!("Current Schema version: {}", current_version),
            LogLevel::Debug,
        );
        let mut i = 0;
        self.migrations.reverse();
        for migration in self.migrations.into_iter() {
            if i >= count {
                break;
            }
            if migration.version <= current_version {
                migration.apply_down(db)?;
                db.save()?;
                Self::write_schema(&db.schema, &self.schema_file_path)?;
                i += 1;
            }
        }
        log(format!("Rollbacked {} migrations", i), LogLevel::Info);
        Ok(())
    }

    pub fn write_schema(
        schema: &DatabaseSchema,
        schema_file_path: &str,
    ) -> Result<(), MigrationError> {
        log(
            format!("Saving schema to {}", schema_file_path),
            LogLevel::Debug,
        );
        let mut file = OpenOptions::new().write(true).open(&schema_file_path)?;
        let content = Self::serialized_schema(&schema);
        // Cleans the file
        file.set_len(0)?;
        file.write_all(content.as_bytes())?;
        log(
            format!("Bumped schema to version {}", schema.version.unwrap_or(0)),
            LogLevel::Debug,
        );
        Ok(())
    }
}
