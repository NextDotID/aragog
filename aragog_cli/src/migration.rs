use std::fs;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::Path;

use chrono::Utc;

use crate::error::AragogCliError;
use crate::log;
use crate::log_level::LogLevel;
use crate::migration_data::MigrationData;
use crate::versioned_database::VersionedDatabase;

pub type MigrationVersion = u64;

const HELP_MESSAGE: &str = "# The migration files contain two sections: \n\
                            # - up: The commands to execute on migration \n\
                            # - down: The commands to execute on rollback (optional) \n\
                            # check https://docs.rs/aragog_cli for complete documentation and examples \n";
const MIGRATION_PATH: &str = "migrations";

#[derive(Debug)]
pub struct Migration {
    pub name: String,
    pub version: MigrationVersion,
    pub data: MigrationData,
    pub path: String,
}

impl Migration {
    pub fn migration_path(schema_path: &str) -> Result<String, AragogCliError> {
        if !Path::new(&schema_path).is_dir() {
            return Err(AragogCliError::InitError {
                item: schema_path.to_string(),
                message: String::from("Is not a valid directory"),
            });
        }
        let db_path = format!("{}/{}", schema_path, MIGRATION_PATH);
        if !Path::new(&db_path).is_dir() {
            log(
                format!(
                    "Missing {}/ path in {}. creating it...",
                    MIGRATION_PATH, schema_path
                ),
                LogLevel::Debug,
            );
            fs::create_dir(&db_path)?;
        }
        Ok(db_path)
    }

    pub fn new(name: &str, schema_path: &str, write: bool) -> Result<Self, AragogCliError> {
        let data = MigrationData::default();
        let version = Utc::now().timestamp_millis() as u64;
        let migration_path = Self::migration_path(schema_path)?;
        let path = format!(
            "{}/{}_{}.yaml",
            migration_path,
            version,
            name.to_ascii_lowercase()
        );
        let res = Self {
            name: name.to_string(),
            version,
            data,
            path: path.clone(),
        };
        if write {
            res.save()?;
            log(format!("Created Migration {}", path), LogLevel::Info);
        }
        Ok(res)
    }

    pub fn file(&self) -> Result<File, AragogCliError> {
        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .open(&self.path)?;
        Ok(file)
    }

    pub fn save(&self) -> Result<(), AragogCliError> {
        let data_str = serde_yaml::to_string(&self.data).unwrap();
        let mut file = self.file()?;
        let buff = format!("{}{}", HELP_MESSAGE, data_str);
        file.write_all(buff.as_bytes())?;
        Ok(())
    }

    pub fn load(file_name: &str, schema_path: &str) -> Result<Self, AragogCliError> {
        let path = format!("{}/{}/{}", schema_path, MIGRATION_PATH, file_name);
        log(format!("Loading migration file {}", path), LogLevel::Info);
        let mut split = file_name.split('_');
        let version = match split.next() {
            Some(str) => str,
            None => {
                return Err(AragogCliError::InvalidFileName {
                    file_name: file_name.to_string(),
                });
            }
        };
        let version: MigrationVersion = match version.parse() {
            Ok(value) => value,
            Err(_error) => {
                return Err(AragogCliError::InvalidFileName {
                    file_name: file_name.to_string(),
                });
            }
        };
        let vec: Vec<&str> = split.collect();
        let name = vec.join("_");
        let data = MigrationData::load(&path)?;
        Ok(Self {
            name,
            version,
            data,
            path,
        })
    }

    pub fn apply_up(
        self,
        db: &mut VersionedDatabase,
        silent: bool,
    ) -> Result<MigrationVersion, AragogCliError> {
        log(
            format!("Apply Migration {} ...", &self.name),
            LogLevel::Info,
        );
        for operation in self.data.up {
            operation.apply(db, silent)?;
        }
        db.schema.version = Some(self.version);
        log("Done.", LogLevel::Info);
        Ok(self.version)
    }

    pub fn apply_down(
        self,
        db: &mut VersionedDatabase,
    ) -> Result<MigrationVersion, AragogCliError> {
        log(
            format!("Rollback Migration {} ...", &self.name),
            LogLevel::Info,
        );
        for operation in self.data.down.unwrap_or_default() {
            operation.apply(db, false)?;
        }
        db.schema.version = Some(self.version - 1);
        log("Done.", LogLevel::Info);
        Ok(self.version)
    }
}
